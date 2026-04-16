//! MQTT topics для Бахилизатора
//!
//! Топики построены по шаблону: bahilizator/{machine_id}/{type}
//!
//!   - settings  — настройки (JSON, от сервера к автомату)
//!   - errors    — ошибки (JSON, от автомата к серверу)
//!   - event     — события (JSON, от автомата к серверу)
//!   - state     — состояние (JSON, каждые 5 мин)
//!   - accounting — учёт (JSON, каждые 1 час)

use core::fmt::Write;

// ── Константы ────────────────────────────────────────────────────────────

/// Префикс всех топиков
const TOPIC_PREFIX: &str = "bahilizator";
/// Максимальная длина полного топика
pub const MAX_TOPIC_LEN: usize = 64;

// ── Типы топиков ────────────────────────────────────────────────────────

/// Тип MQTT топика
#[derive(Debug, Clone, Copy, defmt::Format)]
pub enum TopicKind {
    /// Настройки (от сервера — подписка)
    Settings,
    /// Ошибки (от автомата — publish, немедленно)
    Errors,
    /// События (от автомата — publish, batched 5с)
    Event,
    /// Состояние (от автомата — publish, каждые 5 мин)
    State,
    /// Учёт (от автомата — publish, каждые 1 час)
    Accounting,
}

impl TopicKind {
    /// Суффикс топика
    pub fn as_str(&self) -> &'static str {
        match self {
            TopicKind::Settings => "settings",
            TopicKind::Errors => "errors",
            TopicKind::Event => "event",
            TopicKind::State => "state",
            TopicKind::Accounting => "accounting",
        }
    }
}

// ── Построение топиков ──────────────────────────────────────────────────

/// Построить полный топик: bahilizator/{machine_id}/{kind}
///
/// machine_id — числовой идентификатор автомата из Settings
pub fn build_topic(machine_id: i32, kind: TopicKind) -> heapless::String<MAX_TOPIC_LEN> {
    let mut topic = heapless::String::new();
    let _ = write!(topic, "{}/{}/{}", TOPIC_PREFIX, machine_id, kind.as_str());
    topic
}

/// Топик для настроек (подписка)
pub fn settings_topic(machine_id: i32) -> heapless::String<MAX_TOPIC_LEN> {
    build_topic(machine_id, TopicKind::Settings)
}

/// Топик для ошибок (publish — немедленно)
pub fn errors_topic(machine_id: i32) -> heapless::String<MAX_TOPIC_LEN> {
    build_topic(machine_id, TopicKind::Errors)
}

/// Топик для событий (publish — batched 5с)
pub fn event_topic(machine_id: i32) -> heapless::String<MAX_TOPIC_LEN> {
    build_topic(machine_id, TopicKind::Event)
}

/// Топик для состояния (publish — каждые 5 мин)
pub fn state_topic(machine_id: i32) -> heapless::String<MAX_TOPIC_LEN> {
    build_topic(machine_id, TopicKind::State)
}

/// Топик для учёта (publish — каждые 1 час)
pub fn accounting_topic(machine_id: i32) -> heapless::String<MAX_TOPIC_LEN> {
    build_topic(machine_id, TopicKind::Accounting)
}

// ── Интервалы публикации ────────────────────────────────────────────────

/// Интервал публикации state (5 минут)
pub const STATE_PUBLISH_INTERVAL_SECS: u64 = 300;
/// Интервал публикации accounting (1 час)
pub const ACCOUNTING_PUBLISH_INTERVAL_SECS: u64 = 3600;
/// Интервал batched event публикации (5 секунд)
pub const EVENT_BATCH_INTERVAL_SECS: u64 = 5;
/// MQTT keepalive (60 секунд)
pub const MQTT_KEEPALIVE_SECS: u16 = 60;
/// Интервал переподключения MQTT (30 секунд)
pub const MQTT_RECONNECT_INTERVAL_SECS: u64 = 30;