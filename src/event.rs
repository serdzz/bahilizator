//! Лог событий и транзакций
//!
//! Перенос из bah_events.h / bah_events.c
//! Кольцевой буфер на 32 события с форматированием
//! Транзакции — FIFO буфер с автоматическим закрытием

use crate::state::{Cash, Level, Timestamp};
use crate::error::Errors;

// ── Константы ────────────────────────────────────────────────────────────

/// Размер кольцевого буфера событий
pub const EVENT_RING_SIZE: usize = 32;

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
//
/// Перенос из bah_events.c: EVENT_TYPE_NONE, ACCESS, ACTION,
/// COINS_REFILL, ITEMS_REFILL, ERROR
/// Дополнено: CoinIn, ItemDispensed, HopperError, Door, Power, Gsm, Ibutton

#[derive(Debug, Clone, Copy, Default, defmt::Format, PartialEq)]
#[repr(u8)]
pub enum EventKind {
    #[default]
    None = 0,
    /// Монета внесена (канал, номинал)
    CoinIn = 1,
    /// Товар выдан
    ItemDispensed = 2,
    /// Ошибка хоппера
    HopperError = 3,
    /// Дверь открыта
    DoorOpen = 6,
    /// Дверь закрыта
    DoorClose = 7,
    /// Отключение питания
    PowerFail = 8,
    /// Ошибка GSM
    GsmError = 10,
    /// Ключ iButton поднесён
    IbuttonKey = 12,
    /// Бесплатная выдача
    FreeItemDispensed = 14,
    /// Сброс ошибок
    ErrorCleared = 4,
    /// Настройки изменены
    SettingsChanged = 5,
    /// SMS отправлено
    GsmSmsSent = 15,
    /// SMS получено
    GsmSmsReceived = 16,
    /// Пополнение монет хоппера
    CoinsRefill = 17,
    /// Пополнение товара
    ItemsRefill = 18,
}

// ── EventInfo — дополнительная информация о событии ───────────────────────

#[derive(Debug, Clone, Copy, Default, defmt::Format)]
pub struct EventInfo {
    /// Параметр 1 (канал монеты, номер хоппера, номер двери и т.д.)
    pub param1: u16,
    /// Параметр 2 (номинал монеты, количество и т.д.)
    pub param2: u16,
}

// ── EventEntry — запись в логе событий ────────────────────────────────────

#[derive(Debug, Clone, Copy, Default, defmt::Format)]
pub struct EventEntry {
    pub timestamp: Timestamp,
    pub kind: EventKind,
    pub info: EventInfo,
}

// ── EventRing — кольцевой буфер событий ──────────────────────────────────
//
/// Фиксированный кольцевой буфер на EVENT_RING_SIZE событий.
/// Новые события перезаписывают самые старые.
/// Перенос из bah_events.c: AddEmptyEvent, GetNextEvent, GetLastEvent

pub struct EventRing {
    /// Кольцевой буфер
    buf: [EventEntry; EVENT_RING_SIZE],
    /// Индекс следующей записи (write pointer)
    head: usize,
    /// Количество записанных событий (до EVENT_RING_SIZE)
    count: usize,
}

impl EventRing {
    /// Создать пустой кольцевой буфер
    pub fn new() -> Self {
        Self {
            buf: [EventEntry::default(); EVENT_RING_SIZE],
            head: 0,
            count: 0,
        }
    }

    /// Добавить событие в кольцевой буфер
    ///
    /// В оригинале: AddEmptyEvent + заполнение полей
    /// У нас — атомарная запись сразу всей записи
    pub fn push(&mut self, kind: EventKind, info: EventInfo, timestamp: Timestamp) {
        self.buf[self.head] = EventEntry {
            timestamp,
            kind,
            info,
        };
        self.head = (self.head + 1) % EVENT_RING_SIZE;
        if self.count < EVENT_RING_SIZE {
            self.count += 1;
        }
    }

    /// Получить последние n событий (от нового к старому)
    ///
    /// Возвращает Vec<EventEntry> размером не более n
    /// В оригинале: GetLastEvent + итерация назад
    pub fn get_last(&self, n: usize) -> heapless::Vec<EventEntry, EVENT_RING_SIZE> {
        let mut result = heapless::Vec::new();
        let actual_n = n.min(self.count);
        for i in 0..actual_n {
            // Индекс: (head - 1 - i) по модулю размера
            let idx = (self.head + EVENT_RING_SIZE - 1 - i) % EVENT_RING_SIZE;
            result.push(self.buf[idx]).ok();
        }
        result
    }

    /// Общее количество записанных событий (до переполнения буфера)
    pub fn count(&self) -> usize {
        self.count
    }

    /// Очистить буфер событий
    pub fn clear(&mut self) {
        self.head = 0;
        self.count = 0;
    }
}

// ── Форматирование событий ──────────────────────────────────────────────

/// Форматировать событие в короткую строку (до 32 символов)
///
/// В оригинале: GetEventTypeStr + GetActionEventStr
/// Формат: "ТИП п1:п2" или просто "ТИП"
pub fn format_event(event: &EventEntry) -> heapless::String<32> {
    use core::fmt::Write;
    let mut s = heapless::String::new();

    match event.kind {
        EventKind::CoinIn => {
            let _ = write!(s, "Coin ch{} v{}", event.info.param1, event.info.param2);
        }
        EventKind::ItemDispensed => {
            let _ = write!(s, "Item out:{}", event.info.param1);
        }
        EventKind::FreeItemDispensed => {
            let _ = write!(s, "Free item:{}", event.info.param1);
        }
        EventKind::HopperError => {
            let _ = write!(s, "Hopper err:{}", event.info.param1);
        }
        EventKind::DoorOpen => {
            let _ = write!(s, "Door {} open", event.info.param1 + 1);
        }
        EventKind::DoorClose => {
            let _ = write!(s, "Door {} close", event.info.param1 + 1);
        }
        EventKind::PowerFail => {
            let _ = s.push_str("Power fail");
        }
        EventKind::GsmError => {
            let _ = write!(s, "GSM err:{}", event.info.param1);
        }
        EventKind::IbuttonKey => {
            let _ = write!(s, "iBtn {:02X}{:02X}..",
                (event.info.param1 >> 8) as u8,
                (event.info.param1 & 0xFF) as u8);
        }
        EventKind::ErrorCleared => {
            let _ = s.push_str("Err cleared");
        }
        EventKind::SettingsChanged => {
            let _ = s.push_str("Settings chg");
        }
        EventKind::GsmSmsSent => {
            let _ = s.push_str("SMS sent");
        }
        EventKind::GsmSmsReceived => {
            let _ = s.push_str("SMS recv");
        }
        EventKind::CoinsRefill => {
            let _ = write!(s, "Coins ref H{}:{}", event.info.param1, event.info.param2);
        }
        EventKind::ItemsRefill => {
            let _ = write!(s, "Items ref:{}", event.info.param1);
        }
        EventKind::None => {
            let _ = s.push_str("---");
        }
    }

    s
}

// ── Функции логирования для совместимости с state.rs ─────────────────────
//
/// Добавить событие в массив (старый формат из state.rs — FIFO сдвиг)
/// Оставлено для совместимости с VendingStateData.events
pub fn log_event(events: &mut [EventEntry], kind: EventKind, info: EventInfo, timestamp: Timestamp) {
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
///
/// В оригинале: CloseTransaction + OpenTransaction
pub fn close_transaction(transactions: &mut [TransactionEntry], timestamp: Timestamp) {
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

/// Открыть новую транзакцию (сумма внесена / выдана)
///
/// В оригинале: OpenTransaction
pub fn open_transaction(
    transactions: &mut [TransactionEntry],
    cash_in: Cash,
    cash_out: Cash,
    items_out: Level,
    timestamp: Timestamp,
) {
    // Найти следующий свободный слот (или самый старый)
    let mut idx = 0;
    let len = transactions.len();
    for i in 0..len {
        if transactions[i].closed || transactions[i].timestamp == 0 {
            idx = i;
            break;
        }
    }

    transactions[idx] = TransactionEntry {
        timestamp,
        cash_in,
        cash_out,
        items_out,
        errors: Errors::NONE,
        closed: false,
    };
}