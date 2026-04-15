//! Лог событий и транзакций
//!
//! Перенос из bah_events.h / bah_events.c

use crate::state::{Cash, Level, Timestamp};
use crate::error::Errors;

// ── TransactionEntry — запись в логе транзакций ──────────────────────────

#[derive(Debug, Clone, Copy, Default, defmt::Format)]
pub struct TransactionEntry {
    pub timestamp: Timestamp,
    pub cash_in: Cash,
    pub cash_out: Cash,
    pub items_out: Level,
    pub errors: Errors,
    pub closed: bool,
}

// ── EventKind — типы событий ─────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Default, defmt::Format)]
pub enum EventKind {
    #[default]
    None = 0,
    CoinInserted = 1,
    ItemDispensed = 2,
    CoinDispensed = 3,
    ErrorCleared = 4,
    SettingsChanged = 5,
    DoorOpened = 6,
    DoorClosed = 7,
    PowerUp = 8,
    PowerDown = 9,
    GsmSmsSent = 10,
    GsmSmsReceived = 11,
    IbuttonAccepted = 12,
    IbuttonRejected = 13,
    FreeItemDispensed = 14,
}

// ── EventInfo — дополнительная информация о событии ───────────────────────

#[derive(Debug, Clone, Copy, Default, defmt::Format)]
pub struct EventInfo {
    pub param1: u16,
    pub param2: u16,
}

// ── EventEntry — запись в логе событий ────────────────────────────────────

#[derive(Debug, Clone, Copy, Default, defmt::Format)]
pub struct EventEntry {
    pub timestamp: Timestamp,
    pub kind: EventKind,
    pub info: EventInfo,
}

// ── Функции логирования ──────────────────────────────────────────────────

/// Добавить событие в кольцевой буфер
pub fn log_event(events: &mut [EventEntry], kind: EventKind, info: EventInfo, timestamp: Timestamp) {
    // Сдвигаем все записи на одну позицию назад (FIFO)
    let len = events.len();
    if len > 1 {
        events.copy_within(0..len - 1, 1);
    }
    events[0] = EventEntry {
        timestamp,
        kind,
        info,
    };
}

/// Закрыть текущую транзакцию и начать новую
pub fn close_transaction(transactions: &mut [TransactionEntry], timestamp: Timestamp) {
    // Сдвигаем все записи
    let len = transactions.len();
    if len > 1 {
        transactions.copy_within(0..len - 1, 1);
    }
    // Закрываем текущую (если не закрыта)
    if !transactions[0].closed {
        transactions[0].closed = true;
    }
    // Новая пустая транзакция
    transactions[0] = TransactionEntry {
        timestamp,
        ..Default::default()
    };
}