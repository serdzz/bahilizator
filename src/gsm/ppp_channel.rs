//! PPP over CMUX DLC2 — embassy-net-ppp для GPRS
//!
//! SIM800L: ATD*99***1# → CONNECT → PPP negotiation → IP
//! Используем `embassy_net_ppp` для PPP сессии:
//!   - `embassy_net_ppp::new()` создаёт Device + Runner
//!   - Runner.run() обрабатывает PPP протокол (LCP, PAP, IPCP)
//!   - Device передаётся в embassy_net::Stack::new()
//!
//! Жизненный цикл:
//!   1. ATD*99***1# через at_channel (DLC1)
//!   2. CONNECT → PPP данные через DLC2
//!   3. embassy-net-ppp Runner: LCP + PAP + IPCP
//!   4. embassy-net Stack: IP + DNS + TCP/UDP
//!
//! Перенос из gsm.c: PPP обработка на DLC2.
//! Старый software PPP заменён на embassy-net-ppp crate.

// ── PPP состояние ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, defmt::Format)]
pub enum PppState {
    /// Соединение разорвано
    Dead,
    /// PPP Runner запущен, LCP negotiation
    Establish,
    /// IP получен — сеть работает
    Open,
}

// ── Константы ────────────────────────────────────────────────────────────

/// Количество RX буферов для embassy-net-ppp State
pub const PPP_N_RX: usize = 4;
/// Количество TX буферов для embassy-net-ppp State
pub const PPP_N_TX: usize = 4;
/// Количество сокетов в embassy-net Stack
pub const NET_SOCKET_COUNT: usize = 3;

// ── PppChannel — обёртка для PPP через embassy-net-ppp ──────────────────
//
/// PppChannel управляет жизненным циклом PPP.
/// Реальный PPP протокол обрабатывается embassy-net-ppp Runner.
/// Здесь — только состояние и вспомогательные функции.

pub struct PppChannel {
    /// Текущее состояние PPP
    state: PppState,
}

impl PppChannel {
    /// Создать новый PPP канал
    pub fn new() -> Self {
        Self {
            state: PppState::Dead,
        }
    }

    /// Текущее состояние
    pub fn state(&self) -> PppState {
        self.state
    }

    /// PPP поднят?
    pub fn is_up(&self) -> bool {
        matches!(self.state, PppState::Open)
    }

    /// Установить состояние "negotiation"
    pub fn set_establishing(&mut self) {
        self.state = PppState::Establish;
    }

    /// Установить состояние "подключён"
    pub fn set_up(&mut self) {
        self.state = PppState::Open;
    }

    /// Установить состояние "отключён"
    pub fn set_down(&mut self) {
        self.state = PppState::Dead;
    }
}

impl Default for PppChannel {
    fn default() -> Self {
        Self::new()
    }
}

// ── Конфигурация сети ────────────────────────────────────────────────────

/// Создать конфигурацию embassy-net для GPRS соединения
///
/// SIM800L GPRS получает IP через IPCP (аналог DHCP).
/// Используем DHCPv4 — embassy-net получит IP/DNS/gateway от PPP.
pub fn net_config() -> embassy_net::Config {
    embassy_net::Config::dhcpv4(embassy_net::DhcpConfig::default())
}

/// Создать конфигурацию embassy-net со статическим IP
///
/// Fallback: IP из PPP IPCP, DNS 8.8.8.8
#[allow(dead_code)]
pub fn net_config_static(ip: [u8; 4], gateway: [u8; 4], dns: [u8; 4]) -> embassy_net::Config {
    embassy_net::Config::ipv4_static(embassy_net::StaticConfigV4 {
        address: embassy_net::Ipv4Cidr::new(
            embassy_net::Ipv4Address::new(ip[0], ip[1], ip[2], ip[3]),
            24,
        ),
        gateway: Some(embassy_net::Ipv4Address::new(gateway[0], gateway[1], gateway[2], gateway[3])),
        dns_servers: heapless::Vec::from_iter([
            embassy_net::Ipv4Address::new(dns[0], dns[1], dns[2], dns[3]),
        ].iter().copied()),
    })
}

// ── PPP протокол конфигурация ────────────────────────────────────────────

/// Создать Config для SIM800L PPP
///
/// SIM800L GPRS: пустой логин/пароль для PAP
pub fn ppp_config() -> embassy_net_ppp::Config<'static> {
    embassy_net_ppp::Config {
        username: b"",
        password: b"",
    }
}

// ── Вспомогательные функции для CMUX интеграции ─────────────────────────

/// Упаковать PPP данные в CMUX кадр для DLC2
#[allow(dead_code)]
pub fn wrap_ppp_for_cmux(ppp_data: &[u8]) -> heapless::Vec<u8, 300> {
    crate::gsm::cmux::encode_cmux_frame(2, crate::gsm::cmux::UIH, ppp_data)
}