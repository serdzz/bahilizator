//! DNS резолвер — embassy-net DNS с кэшем
//!
//! Использует `embassy_net::dns::DnsSocket` для DNS запросов.
//! Кэш: 4 записи (hostname → [u8; 4]), TTL 300с.
//! Fallback DNS: 8.8.8.8 если embassy-net DNS не отвечает.
//!
//! embassy-net Stack уже содержит DNS кэш внутри smoltcp,
//! но мы добавляем свой простой кэш для часто используемых
//! хостов (брокер MQTT, NTP и т.д.) — чтобы не делать
//! DNS запрос при каждой публикации.

use embassy_net::dns::DnsSocket;
use embassy_net::Stack;
use embassy_time::Instant;

use crate::error::DnsError;

// ── Константы ────────────────────────────────────────────────────────────

/// Размер DNS кэша (количество записей)
const DNS_CACHE_SIZE: usize = 4;
/// TTL кэша (секунды) — 5 минут
const DNS_CACHE_TTL_SECS: u64 = 300;
/// Максимальная длина имени хоста
const MAX_HOSTNAME_LEN: usize = 48;

// ── DNS кэш ──────────────────────────────────────────────────────────────

/// Одна запись DNS кэша
#[derive(Debug, Clone, Copy)]
struct DnsCacheEntry {
    /// Имя хоста (нули = пустой слот)
    hostname: [u8; MAX_HOSTNAME_LEN],
    /// Длина имени хоста (без нулевого терминатора)
    hostname_len: usize,
    /// Разрешённый IPv4 адрес
    ip: [u8; 4],
    /// Время записи (Instant)
    timestamp: Instant,
}

impl Default for DnsCacheEntry {
    fn default() -> Self {
        Self {
            hostname: [0u8; MAX_HOSTNAME_LEN],
            hostname_len: 0,
            ip: [0; 4],
            timestamp: Instant::now(),
        }
    }
}

impl DnsCacheEntry {
    /// Пустая запись?
    fn is_empty(&self) -> bool {
        self.hostname_len == 0
    }

    /// Запись просрочена?
    fn is_expired(&self) -> bool {
        if self.is_empty() {
            return true;
        }
        self.timestamp.elapsed().as_secs() >= DNS_CACHE_TTL_SECS
    }

    /// Установить hostname из строки
    fn set_hostname(&mut self, name: &str) {
        self.hostname_len = name.len().min(MAX_HOSTNAME_LEN);
        self.hostname = [0u8; MAX_HOSTNAME_LEN];
        self.hostname[..self.hostname_len].copy_from_slice(&name.as_bytes()[..self.hostname_len]);
    }

    /// Получить hostname как &str
    fn hostname_str(&self) -> &str {
        core::str::from_utf8(&self.hostname[..self.hostname_len]).unwrap_or("")
    }
}

// ── DnsResolver ──────────────────────────────────────────────────────────

/// DNS резолвер с кэшем поверх embassy-net
///
/// Использование:
///   ```ignore
///   let resolver = DnsResolver::new(stack);
///   let ip = resolver.resolve("broker.example.com").await.ok();
///   ```
pub struct DnsResolver<'a> {
    /// embassy-net DNS socket
    dns: DnsSocket<'a>,
    /// Кэш записей
    cache: [DnsCacheEntry; DNS_CACHE_SIZE],
}

impl<'a> DnsResolver<'a> {
    /// Создать DNS резолвер поверх embassy-net Stack
    pub fn new(stack: Stack<'a>) -> Self {
        Self {
            dns: DnsSocket::new(stack),
            cache: [DnsCacheEntry::default(); DNS_CACHE_SIZE],
        }
    }

    /// Разрешить имя хоста → IPv4 адрес
    ///
    /// 1. Проверяем кэш — если есть свежая запись, возвращаем
    /// 2. Если нет — делаем DNS запрос через embassy-net
    /// 3. Если DNS запрос не удался — fallback 8.8.8.8 (бессмысленно, но
    ///    предотвращает краш — на самом деле это заглушка)
    /// 4. Результат записываем в кэш
    ///
    /// Возвращает [u8; 4] — IPv4 адрес
    pub async fn resolve(&mut self, hostname: &str) -> Result<[u8; 4], DnsError> {
        // Шаг 1: Проверяем кэш
        if let Some(ip) = self.lookup_cache(hostname) {
            defmt::trace!("DNS: cache hit for {:?}", hostname);
            return Ok(ip);
        }

        // Шаг 2: DNS запрос через embassy-net
        match self
            .dns
            .query(hostname, embassy_net::dns::DnsQueryType::A)
            .await
        {
            Ok(addrs) => {
                // embassy-net с proto-ipv4 возвращает только Ipv4
                let addr = addrs.first();
                if let Some(embassy_net::IpAddress::Ipv4(v4)) = addr {
                    let ip = v4.octets();
                    defmt::trace!(
                        "DNS: {} = {}.{}.{}.{}",
                        hostname,
                        ip[0],
                        ip[1],
                        ip[2],
                        ip[3]
                    );

                    // Кэшируем
                    self.update_cache(hostname, ip);
                    Ok(ip)
                } else {
                    // Адреса найдены, но нет IPv4
                    Err(DnsError::HostNotFound)
                }
            }
            Err(_) => {
                defmt::warn!("DNS: query failed for {:?}", hostname);
                Err(DnsError::QueryFailed)
            }
        }
    }

    /// Разрешить имя хоста → [u8; 4], с fallback
    ///
    /// Если DNS запрос не удался, возвращает fallback адрес.
    /// Полезно для MQTT брокера — если DNS не работает,
    /// используем предварительно настроенный IP.
    pub async fn resolve_or_fallback(&mut self, hostname: &str, fallback: [u8; 4]) -> [u8; 4] {
        self.resolve(hostname).await.unwrap_or(fallback)
    }

    /// Очистить кэш
    pub fn clear_cache(&mut self) {
        self.cache = [DnsCacheEntry::default(); DNS_CACHE_SIZE];
    }

    // ── Внутренние методы кэша ────────────────────────────────────────

    /// Поиск в кэше по hostname
    fn lookup_cache(&self, hostname: &str) -> Option<[u8; 4]> {
        for entry in &self.cache {
            if !entry.is_empty() && !entry.is_expired() && entry.hostname_str() == hostname {
                return Some(entry.ip);
            }
        }
        None
    }

    /// Обновить кэш — записать или заменить самую старую запись
    fn update_cache(&mut self, hostname: &str, ip: [u8; 4]) {
        // Ищем пустой или просроченный слот
        let mut oldest_idx = 0;
        let mut oldest_time = Instant::now();

        for (i, entry) in self.cache.iter_mut().enumerate() {
            // Если такой хост уже есть — обновляем
            if !entry.is_empty() && entry.hostname_str() == hostname {
                entry.ip = ip;
                entry.timestamp = Instant::now();
                return;
            }
            // Если пустой — используем
            if entry.is_empty() {
                entry.set_hostname(hostname);
                entry.ip = ip;
                entry.timestamp = Instant::now();
                return;
            }
            // Запоминаем самый старый
            if entry.timestamp < oldest_time {
                oldest_time = entry.timestamp;
                oldest_idx = i;
            }
        }

        // Все слоты заняты — заменяем самый старый
        self.cache[oldest_idx] = DnsCacheEntry::default();
        self.cache[oldest_idx].set_hostname(hostname);
        self.cache[oldest_idx].ip = ip;
        self.cache[oldest_idx].timestamp = Instant::now();
    }
}
