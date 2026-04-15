//! Генерация SMS отчётов
//!
//! Формирование текста SMS для различных типов отчётов

use heapless::String;

use crate::state::{Accounting, MessageKind, VendingState};

const SMS_MAX_LEN: usize = 160;

/// Сгенерировать текст SMS отчёта по типу
pub fn generate_report(state: &VendingState, kind: MessageKind) -> String<SMS_MAX_LEN> {
    match kind {
        MessageKind::ReportState => report_state(state),
        MessageKind::ReportPeriodAccounting => report_accounting(&state.data.period_accounting),
        MessageKind::ReportOverallAccounting => report_accounting(&state.data.overall_accounting),
        MessageKind::ReportErrors => report_errors(state),
        MessageKind::ReportIntrusion => {
            let mut s = String::new();
            let _ = s.push_str("ВЗЛОМ! Дверь открыта!");
            s
        }
        MessageKind::ReportNoIntrusion => {
            let mut s = String::new();
            let _ = s.push_str("Дверь закрыта.");
            s
        }
        MessageKind::PowerUp => {
            let mut s = String::new();
            use core::fmt::Write;
            let _ = write!(s, "BAH #{} UP", state.settings.machine_id);
            s
        }
        MessageKind::PowerDown => {
            let mut s = String::new();
            use core::fmt::Write;
            let _ = write!(s, "BAH #{} DOWN", state.settings.machine_id);
            s
        }
        MessageKind::ResetErrors => {
            let mut s = String::new();
            let _ = s.push_str("Ошибки сброшены");
            s
        }
        MessageKind::ResetKeys => {
            let mut s = String::new();
            let _ = s.push_str("Ключи сброшены");
            s
        }
        MessageKind::CoinHopperWarningLevel => {
            let mut s = String::new();
            use core::fmt::Write;
            let _ = write!(s, "Монеты: A={} B={}",
                state.data.coin_levels.get(0).unwrap_or(&0),
                state.data.coin_levels.get(1).unwrap_or(&0));
            s
        }
        MessageKind::ItemDispenserWarningLevel => {
            let mut s = String::new();
            use core::fmt::Write;
            let _ = write!(s, "Товар: {}", state.data.item_level);
            s
        }
        _ => {
            let mut s = String::new();
            let _ = s.push_str("N/A");
            s
        }
    }
}

fn report_state(state: &VendingState) -> String<SMS_MAX_LEN> {
    let mut s = String::new();
    use core::fmt::Write;
    let _ = write!(
        s,
        "#{} Cash:{} Item:{} CA:{} CB:{}",
        state.settings.machine_id,
        state.data.cash,
        state.data.item_level,
        state.data.coin_levels.get(0).unwrap_or(&0),
        state.data.coin_levels.get(1).unwrap_or(&0)
    );
    s
}

fn report_accounting(accounting: &Accounting) -> String<SMS_MAX_LEN> {
    let mut s = String::new();
    use core::fmt::Write;
    let _ = write!(
        s,
        "IN:{} OUT:{} Items:{} Free:{}",
        accounting.cash_in,
        accounting.cash_out,
        accounting.items_out,
        accounting.free_items_out
    );
    s
}

fn report_errors(state: &VendingState) -> String<SMS_MAX_LEN> {
    let mut s = String::new();
    use core::fmt::Write;
    if state.errors.is_empty() {
        let _ = write!(s, "No errors");
    } else {
        let _ = write!(s, "Errors: {:X}", state.errors.bits());
    }
    s
}