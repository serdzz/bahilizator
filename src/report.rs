//! Генерация SMS отчётов
//!
//! Формирование текста SMS для различных типов отчётов
//! Перенос из Bahilizator.c: CreateStateReport, CreateAccountingReport,
//! CreateErrorsReport, CreateIntrusionReport, CreatePowerUp/DownMessage
//!
//! Все отчёты — heapless::String<160> (SMS limit)
//! Формат приближен к оригиналу, но сжат для 160 символов

use heapless::String;
use core::fmt::Write;

use crate::state::{Accounting, MessageKind, VendingState};
use crate::config;

const SMS_MAX_LEN: usize = 160;

/// Сгенерировать текст SMS отчёта по типу
///
/// В оригинале: CreateReportHeader + Create*ReportBody
pub fn generate_report(state: &VendingState, kind: MessageKind) -> String<SMS_MAX_LEN> {
    match kind {
        MessageKind::ReportState => report_state(state),
        MessageKind::ReportPeriodAccounting => report_accounting(&state.data.period_accounting, false),
        MessageKind::ReportOverallAccounting => report_accounting(&state.data.overall_accounting, true),
        MessageKind::ReportErrors => report_errors(state),
        MessageKind::ReportIntrusion => report_intrusion(state),
        MessageKind::ReportNoIntrusion => report_no_intrusion(),
        MessageKind::PowerUp => report_power_up(state),
        MessageKind::PowerDown => report_power_down(state),
        MessageKind::ResetErrors => {
            let mut s = String::new();
            let _ = s.push_str("Errors cleared");
            s
        }
        MessageKind::ResetKeys => {
            let mut s = String::new();
            let _ = s.push_str("Keys reset");
            s
        }
        MessageKind::CoinHopperWarningLevel => report_hopper_warning(state),
        MessageKind::ItemDispenserWarningLevel => report_item_warning(state),
        MessageKind::Report => report_state(state),
        MessageKind::None => {
            let mut s = String::new();
            let _ = s.push_str("N/A");
            s
        }
    }
}

// ── Заголовок отчёта ─────────────────────────────────────────────────────
//
/// В оригинале: CreateReportHeader — "ID: x\nTime: ..."
/// Для SMS экономим символы — формат компактный

fn report_header(state: &VendingState) -> String<SMS_MAX_LEN> {
    let mut s = String::new();
    let _ = write!(s, "ID:{} ", state.settings.machine_id);
    s
}

// ── State report ─────────────────────────────────────────────────────────
//
/// В оригинале: CreateStateReport → header + items + hoppers + errors
/// Формат SMS: "ID:x Item:xx HA:xx HB:xx Err:yes/no"

fn report_state(state: &VendingState) -> String<SMS_MAX_LEN> {
    let mut s = report_header(state);

    // Уровень товара
    let _ = write!(s, "Item:{} ", state.data.item_level);

    // Уровни хопперов
    for i in 0..config::HOPPER_COUNT {
        let level = *state.data.coin_levels.get(i).unwrap_or(&0);
        let _ = write!(s, "H{}:{} ", i + 1, level);
    }

    // Наличность
    let _ = write!(s, "Cash:{} ", state.data.cash);

    // Ошибки
    if state.errors.is_empty() {
        let _ = s.push_str("Err:no");
    } else {
        let _ = write!(s, "Err:{:X}", state.errors.bits());
    }

    s
}

// ── Accounting report ────────────────────────────────────────────────────
//
/// В оригинале: CreateAccountingReport → header + "Overall/Period\nIn:x\nOut:x\nProfit:x\nSold:x\nFree:x\nErrors:yes/no"
/// Формат SMS: "ID:x [Overall/Period] IN:xx OUT:xx PROF:xx SOLD:xx FREE:xx Err:yes/no"

fn report_accounting(accounting: &Accounting, overall: bool) -> String<SMS_MAX_LEN> {
    let mut s = String::new();

    // Тип отчёта
    if overall {
        let _ = s.push_str("Overall ");
    } else {
        let _ = s.push_str("Period ");
    }

    // Внесено / Выдано / Прибыль
    let _ = write!(s, "IN:{} OUT:{} PROF:{} ",
        accounting.cash_in,
        accounting.cash_out,
        accounting.cash_in - accounting.cash_out);

    // Продано / Бесплатно
    let _ = write!(s, "SOLD:{} FREE:{} ",
        accounting.items_out,
        accounting.free_items_out);

    // Пополнения монет (если есть)
    for i in 0..config::HOPPER_COUNT {
        let refill = accounting.coins_refill.get(i).unwrap_or(&0);
        if *refill > 0 {
            let _ = write!(s, "CR{}:{} ", i + 1, refill);
        }
    }

    // Пополнение товара
    if accounting.items_refill > 0 {
        let _ = write!(s, "IR:{} ", accounting.items_refill);
    }

    s
}

// ── Errors report ────────────────────────────────────────────────────────
//
/// В оригинале: CreateErrorsReport → header + список ошибок по имени
/// Формат SMS: "ERR:xxx CASH:xxxx ITEM:xxxx DOOR:x"
/// Битовые ошибки + касса + товар + дверь (из описания задачи)

fn report_errors(state: &VendingState) -> String<SMS_MAX_LEN> {
    let mut s = String::new();

    if state.errors.is_empty() {
        let _ = s.push_str("No errors");
        return s;
    }

    // Битовая маска ошибок (как в оригинале — hex)
    let _ = write!(s, "ERR:{:X} ", state.errors.bits());

    // Касса
    let _ = write!(s, "CASH:{} ", state.data.cash);

    // Товар
    let _ = write!(s, "ITEM:{} ", state.data.item_level);

    // Дверь (из ошибок)
    if state.errors.contains(Errors::DOOR_OPENED) {
        let _ = s.push_str("DOOR:1");
    } else {
        let _ = s.push_str("DOOR:0");
    }

    s
}

// ── Intrusion report ─────────────────────────────────────────────────────
//
/// В оригинале: CreateIntrusionReport → "Intrusions:\nDoor 1 opened\nDoor 2 closed"
/// Формат SMS: "ALARM! D1:open D2:closed"

fn report_intrusion(state: &VendingState) -> String<SMS_MAX_LEN> {
    let mut s = String::new();
    let _ = s.push_str("ALARM! ");

    // В оригинале: doors_to_report с битами для каждой двери
    // У нас — просто проверить ошибку DOOR_OPENED
    if state.errors.contains(Errors::DOOR_OPENED) {
        let _ = s.push_str("Door open!");
    } else {
        let _ = s.push_str("Door!");
    }

    let _ = write!(s, " Cash:{}", state.data.cash);
    s
}

fn report_no_intrusion() -> String<SMS_MAX_LEN> {
    let mut s = String::new();
    let _ = s.push_str("Door closed OK");
    s
}

// ── Power up/down ────────────────────────────────────────────────────────
//
/// В оригинале: CreatePowerUpMessage → header + "Power up!"
/// Формат SMS: "BAH #x UP" / "BAH #x DOWN"

fn report_power_up(state: &VendingState) -> String<SMS_MAX_LEN> {
    let mut s = String::new();
    let _ = write!(s, "BAH #{} UP", state.settings.machine_id);
    s
}

fn report_power_down(state: &VendingState) -> String<SMS_MAX_LEN> {
    let mut s = String::new();
    let _ = write!(s, "BAH #{} DOWN", state.settings.machine_id);
    s
}

// ── Warning levels ────────────────────────────────────────────────────────
//
/// В оригинале: CreateItemWarningLevelMessage → header + "Items level is low!" + item body
/// Формат SMS: "LOW! Item:xx"

fn report_item_warning(state: &VendingState) -> String<SMS_MAX_LEN> {
    let mut s = String::new();
    let _ = write!(s, "LOW! Item:{}", state.data.item_level);
    s
}

/// В оригинале: CreateHopperWarningLevelMessage → header + "Coins level is low!" + hopper body
/// Формат SMS: "LOW! H1:xx H2:xx"

fn report_hopper_warning(state: &VendingState) -> String<SMS_MAX_LEN> {
    let mut s = String::new();
    let _ = s.push_str("LOW! ");
    for i in 0..config::HOPPER_COUNT {
        let level = *state.data.coin_levels.get(i).unwrap_or(&0);
        let _ = write!(s, "H{}:{} ", i + 1, level);
    }
    s
}

// ── Импорт Errors для contains ──────────────────────────────────────────

use crate::error::Errors;