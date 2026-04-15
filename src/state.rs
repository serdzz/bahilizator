//! Состояние и настройки автомата
//!
//! VendingState — общее разделяемое состояние
//! Settings — конфигурация (хранится во Flash)
//! VendingStateData — рабочее состояние (хранится в EEPROM)

use crate::config;
use crate::error::Errors;
use crate::event::{EventEntry, TransactionEntry};

// ── Типы ──────────────────────────────────────────────────────────────────

pub type Cash = i32;
pub type Level = i32;
pub type Timestamp = u32;
pub type Ticks = u64;

// ── Константы размеров ────────────────────────────────────────────────────

pub const TRANSACTION_LOG_SIZE: usize = 5;
pub const EVENT_LOG_SIZE: usize = 20;
pub const PHONE_NUMBER_LEN: usize = 16;
pub const SMS_BUF_SIZE: usize = 160;
pub const IBUTTON_LEN: usize = 8;
pub const PHONE_ACCESS_LEVELS: usize = 2;
pub const PHONES_PER_LEVEL: usize = 2;

// ── AppState — состояния автомата ─────────────────────────────────────────

#[derive(Debug, Clone, Copy, Default, defmt::Format, PartialEq)]
#[repr(u8)]
pub enum AppState {
    #[default]
    AcceptCash = 0,
    PayoutItems = 1,
    PayoutReminder = 2,
    ProcessResidual = 3,
}

// ── Язык и валюта ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Default, defmt::Format)]
pub enum Language {
    #[default]
    Latvian = 0,
    Russian = 1,
}

#[derive(Debug, Clone, Copy, Default, defmt::Format)]
pub enum Currency {
    #[default]
    Eur = 0,
}

// ── Бухгалтерия ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Default, defmt::Format)]
pub struct Accounting {
    pub cash_in: Cash,
    pub cash_out: Cash,
    pub items_out: Level,
    pub free_items_out: Level,
    pub coins_refill: [Level; config::HOPPER_COUNT],
    pub items_refill: Level,
}

// ── Настройки монетоприёмника ─────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Default, defmt::Format)]
pub struct CoinAcceptorSettings {
    pub pulse_mode: bool,
    pub coin_values: [Cash; config::COIN_CHANNEL_COUNT],
    pub coin_enable: [bool; config::COIN_CHANNEL_COUNT],
}

// ── Настройки хоппера ────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Default, defmt::Format)]
pub struct HopperSettings {
    pub coin_value: Cash,
    pub coin_count: Level,
    pub warning_level: Level,
}

// ── Settings — хранится во Flash ─────────────────────────────────────────

pub type IbuttonKey = [u8; IBUTTON_LEN];

#[derive(Debug, Clone, Copy, Default, defmt::Format)]
pub struct Settings {
    pub version: u8,
    pub user_language: Language,
    pub currency: Currency,
    pub machine_id: Level,
    pub coin_acceptor: CoinAcceptorSettings,
    pub coin_hoppers: [HopperSettings; config::HOPPER_COUNT],
    pub item_dispenser: HopperSettings,
    pub keys: [[IbuttonKey; 2]; 3],
    /// Номера телефонов — хранятся как массивы байт (не String, т.к. String не Copy)
    pub phone_numbers: [[[u8; PHONE_NUMBER_LEN]; PHONES_PER_LEVEL]; PHONE_ACCESS_LEVELS],
    pub accounting_report_interval: Level,
    pub state_report_interval: Level,
    pub workday_start_hour: u8,
    pub workday_end_hour: u8,
    pub residual_timeout: u8,
    pub menu_exit_timeout: u8,
    pub cash_clear_timeout: u8,
    pub thanks_message_delay: u8,
    pub payout_message_delay: u8,
    pub options: u32,
    pub crc: u16,
}

// ── VendingStateData — хранится в EEPROM ─────────────────────────────────

#[derive(Debug, Clone, Copy, Default, defmt::Format)]
pub struct VendingStateData {
    pub version: u8,
    pub item_level: Level,
    pub coin_levels: [Level; config::HOPPER_COUNT],
    pub cash: Cash,
    pub coins_pending: [Level; config::HOPPER_COUNT],
    pub items_pending: Level,
    pub free_items_pending: Level,
    pub app_state: AppState,
    pub transactions: [TransactionEntry; TRANSACTION_LOG_SIZE],
    pub events: [EventEntry; EVENT_LOG_SIZE],
    pub overall_accounting: Accounting,
    pub period_accounting: Accounting,
    pub messages_pending: [[Errors; PHONES_PER_LEVEL]; PHONE_ACCESS_LEVELS],
    pub crc: u16,
}

// ── VendingState — общее состояние автомата ───────────────────────────────

#[derive(Debug, Default, defmt::Format)]
pub struct VendingState {
    pub data: VendingStateData,
    pub settings: Settings,
    pub errors: Errors,
}

// ── PersistReason ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, defmt::Format)]
pub enum PersistReason {
    StateChanged,
    SettingsChanged,
}

// ── MessageKind ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, defmt::Format)]
pub enum MessageKind {
    None,
    ResetKeys,
    ReportState,
    ReportPeriodAccounting,
    ReportOverallAccounting,
    Report,
    ReportErrors,
    CoinHopperWarningLevel,
    ItemDispenserWarningLevel,
    ReportIntrusion,
    ReportNoIntrusion,
    PowerUp,
    PowerDown,
    ResetErrors,
}