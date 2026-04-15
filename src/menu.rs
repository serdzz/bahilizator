//! Меню навигации — заготовка
//!
//! Перенос из bah_menu.c — навигация по меню настроек

use crate::state::{MessageKind, VendingState};
use crate::ui::DisplayCommand;
use heapless::String;

// ── Пункты меню ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, defmt::Format, PartialEq)]
pub enum MenuItem {
    CashOnHand,
    ItemLevel,
    CoinLevelA,
    CoinLevelB,
    Language,
    MachineId,
    CoinValue0,
    HopperCoinValueA,
    ItemWarningLevel,
    PhoneNumber00,
    SendReportState,
    SendReportAccounting,
    SendReportErrors,
    ClearErrors,
    ResetKeys,
    FirmwareVersion,
    GsmInfo,
}

impl MenuItem {
    /// Следующий пункт меню
    pub fn next(self) -> Self {
        match self {
            Self::CashOnHand => Self::ItemLevel,
            Self::ItemLevel => Self::CoinLevelA,
            Self::CoinLevelA => Self::CoinLevelB,
            Self::CoinLevelB => Self::Language,
            Self::Language => Self::MachineId,
            Self::MachineId => Self::CoinValue0,
            Self::CoinValue0 => Self::HopperCoinValueA,
            Self::HopperCoinValueA => Self::ItemWarningLevel,
            Self::ItemWarningLevel => Self::PhoneNumber00,
            Self::PhoneNumber00 => Self::SendReportState,
            Self::SendReportState => Self::SendReportAccounting,
            Self::SendReportAccounting => Self::SendReportErrors,
            Self::SendReportErrors => Self::ClearErrors,
            Self::ClearErrors => Self::ResetKeys,
            Self::ResetKeys => Self::FirmwareVersion,
            Self::FirmwareVersion => Self::GsmInfo,
            Self::GsmInfo => Self::CashOnHand,
        }
    }

    /// Предыдущий пункт меню
    pub fn prev(self) -> Self {
        // Упрощённая — проходим вперёд до нахождения предыдущего
        let target = self;
        let mut current = self;
        for _ in 0..16 {
            let next = current.next();
            if next == target {
                return current;
            }
            current = next;
        }
        current
    }
}

// ── Менеджер меню ────────────────────────────────────────────────────────

pub struct MenuNavigator {
    pub current: MenuItem,
    pub editing: bool,
    pub edit_value: i32,
}

impl MenuNavigator {
    pub fn new() -> Self {
        Self {
            current: MenuItem::CashOnHand,
            editing: false,
            edit_value: 0,
        }
    }

    /// Кнопка NEXT
    pub fn next(&mut self) {
        self.current = self.current.next();
    }

    /// Кнопка PREV
    pub fn prev(&mut self) {
        self.current = self.current.prev();
    }

    /// Кнопка OK — войти в редактирование или подтвердить
    pub fn ok(&mut self) -> Option<MessageKind> {
        if self.editing {
            self.editing = false;
            Some(MessageKind::ResetErrors) // placeholder
        } else {
            self.editing = true;
            self.edit_value = 0;
            match self.current {
                MenuItem::SendReportState => Some(MessageKind::ReportState),
                MenuItem::SendReportAccounting => Some(MessageKind::ReportPeriodAccounting),
                MenuItem::SendReportErrors => Some(MessageKind::ReportErrors),
                MenuItem::ClearErrors => Some(MessageKind::ResetErrors),
                MenuItem::ResetKeys => Some(MessageKind::ResetKeys),
                _ => None,
            }
        }
    }

    /// Кнопка CANCEL — выйти из редактирования
    pub fn cancel(&mut self) -> bool {
        if self.editing {
            self.editing = false;
            true
        } else {
            false
        }
    }

    /// Получить текст для отображения текущего пункта
    pub fn display_text(&self, state: &VendingState) -> (String<16>, String<16>) {
        let mut line1 = String::new();
        let mut line2 = String::new();

        match self.current {
            MenuItem::CashOnHand => {
                let _ = line1.push_str("НАЛИЧНОСТЬ");
                let _ = write_i32(&mut line2, state.data.cash);
            }
            MenuItem::ItemLevel => {
                let _ = line1.push_str("ТОВАР");
                let _ = write_i32(&mut line2, state.data.item_level);
            }
            MenuItem::CoinLevelA => {
                let _ = line1.push_str("МОНЕТЫ A");
                let _ = write_i32(&mut line2, state.data.coin_levels[0]);
            }
            MenuItem::CoinLevelB => {
                let _ = line1.push_str("МОНЕТЫ B");
                let _ = write_i32(&mut line2, state.data.coin_levels[1]);
            }
            MenuItem::FirmwareVersion => {
                let _ = line1.push_str("ВЕРСИЯ");
                let _ = write_i32(&mut line2, crate::config::FIRMWARE_VERSION as i32);
            }
            _ => {
                let _ = line1.push_str("MENU");
                let _ = line2.push_str("---");
            }
        }

        (line1, line2)
    }
}

fn write_i32(s: &mut String<16>, v: i32) {
    use core::fmt::Write;
    let _ = write!(s, "{}", v);
}