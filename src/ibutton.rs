//! 1-Wire iButton драйвер — на базе crate `one-wire-bus`
//!
//! DS1990A: Reset pulse → Read ROM (0x33) → 8 байт (family+serial+CRC)
//!
//! Использует `one_wire_bus::OneWire` для 1-Wire протокола
//! (reset, read/write bit/byte, CRC проверка).
//!
//! Адаптер `EmbassyDelay` реализует `embedded_hal::blocking::delay::DelayUs`
//! поверх `cortex_m::asm::delay()` — busy-wait с µs точностью.
//! Это стандартный подход для 1-Wire на Cortex-M, т.к. тайминги
//! критичны (6–480 µs) и async задержки недостаточно точны.
//!
//! Перенос из 1-wire.c: bit-bang заменён на one-wire-bus crate,
//! но логика проверки ключей по whitelist сохранена.

use embassy_sync::channel::Sender;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;

use crate::state::{IBUTTON_LEN, Settings, VendingState};

// ── 1-Wire команды ────────────────────────────────────────────────────────

/// Read ROM — чтение 64-битного адреса единственного устройства на шине
/// DS1990A iButton: family=0x01, serial(48bit), CRC8
/// one-wire-bus crate не экспортирует эту константу — определяем сами
const CMD_READ_ROM: u8 = 0x33;

use one_wire_bus::{OneWire, Address};

// ── IbuttonEvent — событие от iButton ─────────────────────────────────────

#[derive(Debug, Clone, defmt::Format)]
pub struct IbuttonEvent {
    /// 8-байтовый ключ iButton
    pub key: [u8; IBUTTON_LEN],
    /// Ключ принят (true) или отклонён (false)
    pub accepted: bool,
}

// ── EmbassyDelay — адаптер для one-wire-bus ──────────────────────────────
//
/// Реализация `embedded_hal::blocking::delay::DelayUs<u16>` через
/// `cortex_m::asm::delay()` — busy-wait с µs точностью.
///
/// STM32F103C8T6 на 72 MHz: 1 µs ≈ 72 такта.
/// `cortex_m::asm::delay(n)` выполняет n циклов (~3 такта каждый на Cortex-M3),
/// поэтому для x µs: delay(x * 72 / 3) ≈ x * 24.
///
/// Внимание: при работе блокирует CPU — для 1-Wire это нормально,
/// т.к. тайминги требуют µs точности (6–480 µs).
/// Общее время чтения ROM: ~15 мс — приемлемо для 200 мс опроса.

pub struct EmbassyDelay {
    /// Тактовая частота CPU (MHz)
    clock_mhz: u32,
}

impl EmbassyDelay {
    /// Создать delay для заданной частоты (MHz)
    ///
    /// Для STM32F103C8T6 Bluepill: 72 MHz
    pub fn new(clock_mhz: u32) -> Self {
        Self { clock_mhz }
    }
}

impl embedded_hal::blocking::delay::DelayUs<u16> for EmbassyDelay {
    fn delay_us(&mut self, us: u16) {
        // cortex_m::asm::delay(n) выполняет n итераций по ~3 такта
        // На 72 MHz: 1 µs = 72 такта → n = 72/3 = 24 циклов на µs
        let cycles_per_us = self.clock_mhz / 3;
        let total_cycles = us as u32 * cycles_per_us;
        cortex_m::asm::delay(total_cycles);
    }
}

// ── IbuttonDriver — драйвер 1-Wire шины ─────────────────────────────────
//
/// Обёртка над `one_wire_bus::OneWire` для чтения iButton ключей.
/// Пин PA11 конфигурируется как OutputOpenDrain — это позволяет
/// и читать, и писать на 1-Wire шину.
///
/// embassy-stm32 `OutputOpenDrain` реализует `InputPin + OutputPin`
/// из embedded-hal 0.2 — это именно то, что нужно для one-wire-bus.
///
/// В оригинале (1-wire.c): bit-bang на GPIO
/// У нас: one-wire-bus crate + embassy-stm32 OutputOpenDrain

pub struct IbuttonDriver {
    /// 1-Wire шина (one-wire-bus)
    bus: OneWire<embassy_stm32::gpio::OutputOpenDrain<'static>>,
    /// Delay адаптер для one-wire-bus
    delay: EmbassyDelay,
}

impl IbuttonDriver {
    /// Создать драйвер из OutputOpenDrain пина
    ///
    /// Пин должен быть сконфигурирован как OutputOpenDrain:
    /// ```ignore
    /// let pin = p.PA11.into_output_open_drain(
    ///     embassy_stm32::gpio::OutputType::OpenDrain,
    ///     embassy_stm32::gpio::Pull::Up,
    ///     embassy_stm32::gpio::Speed::Low,
    /// );
    /// let driver = IbuttonDriver::new(pin);
    /// ```
    pub fn new(pin: embassy_stm32::gpio::OutputOpenDrain<'static>) -> Self {
        let delay = EmbassyDelay::new(72); // STM32F103C8T6: 72 MHz
        // embassy-stm32 OutputOpenDrain имеет Error=Infallible,
        // поэтому OneWire::new() не может вернуть ошибку
        let bus = OneWire::new(pin).unwrap();

        Self { bus, delay }
    }

    /// Отправить reset pulse и проверить presence
    /// Возвращает true если устройство на шине есть
    ///
    /// В оригинале: oneWireReset()
    ///   ONE_WIRE_OUT → 480µs low → ONE_WIRE_IN → 70µs → read → 410µs
    pub fn reset(&mut self) -> bool {
        self.bus.reset(&mut self.delay).unwrap_or(false)
    }

    /// Полная операция чтения ключа DS1990A:
    /// 1. Reset pulse
    /// 2. Read ROM command (0x33)
    /// 3. Прочитать 8 байт (family + serial + CRC)
    /// 4. Проверить CRC через one_wire_bus::crc
    ///
    /// Возвращает Some(key) если ключ прочитан и CRC верный,
    /// None — если нет устройства или CRC не совпал
    pub fn read_rom(&mut self) -> Option<[u8; IBUTTON_LEN]> {
        // Reset pulse — проверяем наличие устройства
        let device_present = self.reset();
        if !device_present {
            return None;
        }

        // Команда Read ROM (0x33)
        // Используется когда на шине ровно одно устройство —
        // иначе нужно SEARCH_ROM + MATCH_ROM
        self.bus.write_byte(CMD_READ_ROM, &mut self.delay).ok()?;

        // Читаем 8 байт: family(1) + serial(6) + CRC(1)
        let mut key = [0u8; IBUTTON_LEN];
        self.bus.read_bytes(&mut key, &mut self.delay).ok()?;

        // Проверяем CRC: one_wire_bus::crc::crc8(data) вернёт 0
        // если весь массив (включая CRC байт) корректен
        if one_wire_bus::crc::crc8(&key) == 0 {
            Some(key)
        } else {
            defmt::trace!("iButton CRC mismatch");
            None
        }
    }

    /// Поиск всех устройств на шине (SEARCH_ROM = 0xF0)
    ///
    /// Для DS1990A обычно одно устройство, но метод полезен
    /// для диагностики. Возвращает до 4 адресов.
    pub fn find_devices(&mut self) -> heapless::Vec<Address, 4> {
        let mut found = heapless::Vec::new();
        for result in self.bus.devices(false, &mut self.delay) {
            match result {
                Ok(addr) => {
                    found.push(addr).ok();
                }
                Err(_) => break,
            }
        }
        found
    }
}

// ── Проверка ключа по whitelist ──────────────────────────────────────────
//
/// Сравнить прочитанный ROM с whitelist в Settings
/// Settings.keys = [[IbuttonKey; 2]; 3] — до 6 ключей
/// Возвращает уровень доступа (0=сервисный, 1=технический) или None
///
/// Без изменений по сравнению с оригиналом — логика whitelist
/// не зависит от реализации 1-Wire протокола

#[derive(Debug, Clone, Copy, defmt::Format, PartialEq)]
pub enum KeyAccess {
    /// Сервисный ключ (полный доступ)
    Service = 0,
    /// Технический ключ (ограниченный доступ)
    Technical = 1,
}

pub fn check_key(settings: &Settings, key: &[u8; IBUTTON_LEN]) -> Option<KeyAccess> {
    // Все нули — пустой слот, пропускаем
    let is_empty = key.iter().all(|&b| b == 0);
    if is_empty {
        return None;
    }

    // Проверяем каждый уровень доступа
    for level in 0..settings.keys.len() {
        for slot in 0..settings.keys[level].len() {
            let stored = &settings.keys[level][slot];
            let stored_empty = stored.iter().all(|&b| b == 0);
            if !stored_empty && stored == key {
                return match level {
                    0 => Some(KeyAccess::Service),
                    _ => Some(KeyAccess::Technical),
                };
            }
        }
    }

    None
}

// ── Преобразование Address → [u8; 8] ────────────────────────────────────
//
/// Конвертировать one_wire_bus::Address в массив 8 байт
/// Address(u64) хранится в little-endian: family byte → serial → CRC

pub fn address_to_bytes(addr: &Address) -> [u8; IBUTTON_LEN] {
    addr.0.to_le_bytes()
}

// ── Обратная совместимость: CRC функции ──────────────────────────────────
//
/// Делегируем CRC вычисление в one_wire_bus::crc
/// Оставляем для совместимости с другими модулями

/// Вычислить CRC-8 по Dallas/Maxim (делегирует в one_wire_bus::crc::crc8)
pub fn compute_crc(data: &[u8]) -> u8 {
    one_wire_bus::crc::crc8(data)
}

/// Проверить CRC ключа iButton (8 байт: 7 данных + 1 CRC)
pub fn check_crc(key: &[u8; IBUTTON_LEN]) -> bool {
    one_wire_bus::crc::crc8(key) == 0
}

// ── Задача iButton ───────────────────────────────────────────────────────
//
/// Основная задача опроса iButton
/// Опрашивает шину каждые 200мс, при обнаружении ключа —
/// проверяет по whitelist и отправляет событие
///
/// Драйвер передаётся как Option — пока GPIO пины не подключены
/// в main(), задача работает в режиме idle (без опроса шины).
/// После подключения PA11:
///   let pin = p.PA11.into_output_open_drain(...);
///   let driver = IbuttonDriver::new(pin);
///   spawner.spawn(task_ibutton(Some(driver), ...).unwrap());

pub async fn run(
    driver: Option<IbuttonDriver>,
    ibutton_tx: Sender<'static, CriticalSectionRawMutex, IbuttonEvent, 1>,
    state: &'static embassy_sync::mutex::Mutex<
        CriticalSectionRawMutex,
        core::cell::RefCell<VendingState>,
    >,
) {
    // Если драйвер не передан — idle режим (опрос каждые 5с)
    let mut driver = match driver {
        Some(d) => d,
        None => {
            defmt::warn!("iButton: драйвер не инициализирован (нет пина PA11)");
            loop {
                embassy_time::Timer::after_secs(5).await;
            }
        }
    };

    loop {
        // Шаг 1: Попытка чтения ключа
        if let Some(key) = driver.read_rom() {
            // Шаг 2: Проверка по whitelist
            let accepted = {
                let guard = state.lock().await;
                let s = guard.borrow();
                check_key(&s.settings, &key).is_some()
            };

            // Шаг 3: Отправить событие
            let _ = ibutton_tx.try_send(IbuttonEvent { key, accepted });

            defmt::debug!("iButton: ключ обнаружен, accepted={}", accepted);
        }

        // Опрос раз в 200мс
        embassy_time::Timer::after_millis(200).await;
    }
}