//! Меню навигации — сервисное меню автомата
//!
//! Перенос из bah_menu.c — навигация по меню настроек
//! Перенос из bah_scroller.c — автопрокрутка длинных строк на 16x2 дисплее
//!
//! Навигация: Up/Down — перемещение, Enter — выбор/вход, Back — выход
//! Scroller: автопрокрутка каждые 1.5с для строк >16 символов

use crate::config;
use crate::state::{MessageKind, VendingState};
use core::fmt::Write;
use heapless::String;

// ── Пункты сервисного меню ───────────────────────────────────────────────
//
/// Перенос из bah_menu.c: MenuItem с уровнем доступа и вложенностью
/// Уровни: 0 = сервисный ключ, 1 = технический ключ
#[derive(Debug, Clone, Copy, defmt::Format, PartialEq)]
#[repr(u8)]
pub enum MenuItem {
    /// Наличность в кассе
    CashOnHand = 0,
    /// Уровень товара
    ItemLevel = 1,
    /// Уровень монет хоппер A
    CoinLevelA = 2,
    /// Уровень монет хоппер B
    CoinLevelB = 3,
    /// Тест хопперов
    HopperTest = 4,
    /// Тест дисплея
    DisplayTest = 5,
    /// Настройки GSM
    GsmSettings = 6,
    /// Калибровка монетоприёмника
    CoinCalibration = 7,
    /// Язык
    Language = 8,
    /// ID автомата
    MachineId = 9,
    /// Номинал монеты канал 0
    CoinValue0 = 10,
    /// Номинал хоппера A
    HopperCoinValueA = 11,
    /// Уровень предупреждения товара
    ItemWarningLevel = 12,
    /// Телефон владельца
    PhoneNumber00 = 13,
    /// Отправить отчёт состояния
    SendReportState = 14,
    /// Отправить бухгалтерский отчёт
    SendReportAccounting = 15,
    /// Отправить отчёт ошибок
    SendReportErrors = 16,
    /// Сбросить ошибки
    ClearErrors = 17,
    /// Сбросить ключи
    ResetKeys = 18,
    /// Версия прошивки
    Version = 19,
    /// Информация GSM
    GsmInfo = 20,
    /// Количество пунктов (маркер)
    Count = 21,
}

impl MenuItem {
    /// Следующий пункт меню (циклически)
    pub fn next(self) -> Self {
        let cur = self as u8;
        let next = if cur >= (MenuItem::Count as u8 - 1) {
            0
        } else {
            cur + 1
        };
        Self::from_u8(next)
    }

    /// Предыдущий пункт меню (циклически)
    pub fn prev(self) -> Self {
        let cur = self as u8;
        let prev = if cur == 0 {
            MenuItem::Count as u8 - 1
        } else {
            cur - 1
        };
        Self::from_u8(prev)
    }

    fn from_u8(v: u8) -> Self {
        match v {
            0 => Self::CashOnHand,
            1 => Self::ItemLevel,
            2 => Self::CoinLevelA,
            3 => Self::CoinLevelB,
            4 => Self::HopperTest,
            5 => Self::DisplayTest,
            6 => Self::GsmSettings,
            7 => Self::CoinCalibration,
            8 => Self::Language,
            9 => Self::MachineId,
            10 => Self::CoinValue0,
            11 => Self::HopperCoinValueA,
            12 => Self::ItemWarningLevel,
            13 => Self::PhoneNumber00,
            14 => Self::SendReportState,
            15 => Self::SendReportAccounting,
            16 => Self::SendReportErrors,
            17 => Self::ClearErrors,
            18 => Self::ResetKeys,
            19 => Self::Version,
            20 => Self::GsmInfo,
            _ => Self::CashOnHand,
        }
    }

    /// Является ли пункт действием (не редактируемое поле)
    pub fn is_action(self) -> bool {
        matches!(
            self,
            Self::HopperTest
                | Self::DisplayTest
                | Self::GsmSettings
                | Self::CoinCalibration
                | Self::SendReportState
                | Self::SendReportAccounting
                | Self::SendReportErrors
                | Self::ClearErrors
                | Self::ResetKeys
                | Self::GsmInfo
        )
    }

    /// Является ли пункт редактируемым полем
    pub fn is_editable(self) -> bool {
        !self.is_action() && !matches!(self, Self::CashOnHand | Self::Version | Self::Count)
    }
}

// ── MenuAction — результат действия меню ──────────────────────────────────

#[derive(Debug, Clone, Copy, defmt::Format)]
pub enum MenuAction {
    /// Нет действия
    None,
    /// Отправить сообщение GSM
    SendMessage(MessageKind),
    /// Запустить тест хопперов
    TestHoppers,
    /// Запустить тест дисплея
    TestDisplay,
    /// Настройки GSM
    GsmSettings,
    /// Калибровка монетоприёмника
    CoinCalibration,
    /// Выйти из меню
    Exit,
}

// ── Scroller — автопрокрутка для 16x2 ────────────────────────────────────
//
/// Перенос из bah_scroller.c: TextScroller
/// Режимы: Left (без прокрутки), ScrollBounce (туда-сюда)
/// Интервал прокрутки: 1500мс (TEXT_SCROLL_INTERVAL в оригинале)
/// Интервал паузы на краях: 2000мс (TEXT_STOP_INTERVAL)
pub const SCROLL_INTERVAL_MS: u64 = 1500;
pub const SCROLL_PAUSE_MS: u64 = 2000;
pub const DISPLAY_WIDTH: usize = config::LCD_COLS as usize;

#[derive(Debug, Clone, Copy, defmt::Format, PartialEq)]
pub enum ScrollAlign {
    Left,
    Center,
    ScrollBounce,
}

#[derive(Debug, Clone)]
pub struct Scroller {
    /// Текст для отображения (до 40 символов)
    pub text: String<40>,
    /// Режим выравнивания
    pub align: ScrollAlign,
    /// Текущая позиция прокрутки
    pub pos: i16,
    /// Направление прокрутки (1 = вправо, -1 = влево)
    pub direction: i8,
    /// Время следующего обновления (ticks)
    pub next_update: u64,
}

impl Default for Scroller {
    fn default() -> Self {
        Self::new()
    }
}

impl Scroller {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            align: ScrollAlign::Left,
            pos: 0,
            direction: 1,
            next_update: 0,
        }
    }

    /// Установить текст и режим
    pub fn set_text(&mut self, text: &str, align: ScrollAlign) {
        self.text.clear();
        let _ = self.text.push_str(text);
        self.align = align;
        self.pos = 0;
        self.direction = 1;
        // Если текст короче ширины — нет смысла в прокрутке
        if self.text.len() <= DISPLAY_WIDTH && self.align == ScrollAlign::ScrollBounce {
            self.align = ScrollAlign::Center;
        }
    }

    /// Обновить позицию scroller'а
    ///
    /// Возвращает true если позиция изменилась (нужно перерисовать)
    /// Вызывать периодически с текущим временем (мс)
    ///
    /// В оригинале: UpdateScroller() — вызывается из главного цикла
    pub fn update(&mut self, now_ms: u64) -> bool {
        if now_ms < self.next_update {
            return false;
        }

        let len = self.text.len() as i16;
        let changed;

        match self.align {
            ScrollAlign::Left => {
                self.pos = 0;
                changed = false;
                self.next_update = now_ms + SCROLL_INTERVAL_MS;
            }
            ScrollAlign::Center => {
                if len < DISPLAY_WIDTH as i16 {
                    self.pos = -((DISPLAY_WIDTH as i16 - len) / 2);
                } else {
                    self.pos = 0;
                }
                changed = false;
                self.next_update = now_ms + SCROLL_INTERVAL_MS;
            }
            ScrollAlign::ScrollBounce => {
                self.pos += self.direction as i16;
                let max_pos = len - DISPLAY_WIDTH as i16;
                if max_pos <= 0 {
                    self.pos = 0;
                    changed = false;
                    self.next_update = now_ms + SCROLL_PAUSE_MS;
                } else if (self.direction > 0 && self.pos >= max_pos)
                    || (self.direction < 0 && self.pos <= 0)
                {
                    // Достигли края — пауза и разворот
                    changed = true;
                    self.next_update = now_ms + SCROLL_PAUSE_MS;
                    self.direction = -self.direction;
                } else {
                    changed = true;
                    self.next_update = now_ms + SCROLL_INTERVAL_MS;
                }
            }
        }

        changed
    }

    /// Получить видимую часть текста для строки дисплея
    ///
    /// В оригинале: посимвольный вывод через DisplayChar
    /// У нас — возвращаем String<16> для отображения
    pub fn visible_text(&self) -> String<16> {
        let mut result = String::new();
        let text_bytes = self.text.as_bytes();
        for i in 0..DISPLAY_WIDTH {
            let src_idx = self.pos + i as i16;
            if src_idx >= 0 && src_idx < text_bytes.len() as i16 {
                result.push(text_bytes[src_idx as usize] as char).ok();
            } else {
                result.push(' ').ok();
            }
        }
        result
    }
}

// ── MenuState — состояние навигации по меню ──────────────────────────────
//
/// Перенос из bah_menu.c: навигация с поддержкой Up/Down/Enter/Back
/// Добавлен scroller для длинных строк
pub struct MenuNavigator {
    /// Текущий пункт меню
    pub current: MenuItem,
    /// Режим редактирования поля
    pub editing: bool,
    /// Значение при редактировании
    pub edit_value: i32,
    /// Scroller для строки 1 (название пункта)
    pub scroller: Scroller,
    /// Время последней активности (для auto-exit)
    pub last_activity_ms: u64,
}

impl Default for MenuNavigator {
    fn default() -> Self {
        Self::new()
    }
}

impl MenuNavigator {
    pub fn new() -> Self {
        Self {
            current: MenuItem::CashOnHand,
            editing: false,
            edit_value: 0,
            scroller: Scroller::new(),
            last_activity_ms: 0,
        }
    }

    /// Кнопка NEXT (Down) — перейти к следующему пункту
    pub fn next(&mut self, now_ms: u64) {
        if self.editing {
            // В режиме редактирования — увеличить значение
            self.edit_value += 1;
        } else {
            self.current = self.current.next();
        }
        self.last_activity_ms = now_ms;
    }

    /// Кнопка PREV (Up) — перейти к предыдущему пункту
    pub fn prev(&mut self, now_ms: u64) {
        if self.editing {
            // В режиме редактирования — уменьшить значение
            self.edit_value -= 1;
        } else {
            self.current = self.current.prev();
        }
        self.last_activity_ms = now_ms;
    }

    /// Кнопка OK (Enter) — войти в редактирование или выполнить действие
    ///
    /// Возвращает MenuAction если нужно что-то сделать внешнему коду
    pub fn ok(&mut self, state: &VendingState, now_ms: u64) -> MenuAction {
        self.last_activity_ms = now_ms;

        if self.editing {
            // Подтвердить редактирование — применить значение
            self.editing = false;
            self.apply_edit_value(state);
            return MenuAction::None;
        }

        // Действия (не редактирование)
        match self.current {
            MenuItem::SendReportState => MenuAction::SendMessage(MessageKind::ReportState),
            MenuItem::SendReportAccounting => {
                MenuAction::SendMessage(MessageKind::ReportPeriodAccounting)
            }
            MenuItem::SendReportErrors => MenuAction::SendMessage(MessageKind::ReportErrors),
            MenuItem::ClearErrors => MenuAction::SendMessage(MessageKind::ResetErrors),
            MenuItem::ResetKeys => MenuAction::SendMessage(MessageKind::ResetKeys),
            MenuItem::HopperTest => MenuAction::TestHoppers,
            MenuItem::DisplayTest => MenuAction::TestDisplay,
            MenuItem::GsmSettings => MenuAction::GsmSettings,
            MenuItem::CoinCalibration => MenuAction::CoinCalibration,
            _ => {
                // Редактируемое поле — войти в режим редактирования
                if self.current.is_editable() {
                    self.editing = true;
                    self.edit_value = self.read_current_value(state);
                }
                MenuAction::None
            }
        }
    }

    /// Кнопка CANCEL (Back) — выйти из редактирования или из меню
    ///
    /// Возвращает true если нужно выйти из меню совсем
    pub fn cancel(&mut self, now_ms: u64) -> bool {
        self.last_activity_ms = now_ms;
        if self.editing {
            self.editing = false;
            false
        } else {
            true // Выйти из меню
        }
    }

    /// Проверить таймаут auto-exit
    ///
    /// В оригинале: menu_exit_timeout из настроек (по умолчанию 30с)
    pub fn check_timeout(&self, now_ms: u64, timeout_s: u8) -> bool {
        let elapsed = now_ms.saturating_sub(self.last_activity_ms);
        elapsed >= (timeout_s as u64) * 1000
    }

    /// Прочитать текущее значение поля из состояния
    fn read_current_value(&self, state: &VendingState) -> i32 {
        match self.current {
            MenuItem::ItemLevel => state.data.item_level,
            MenuItem::CoinLevelA => *state.data.coin_levels.first().unwrap_or(&0),
            MenuItem::CoinLevelB => *state.data.coin_levels.get(1).unwrap_or(&0),
            MenuItem::Language => state.settings.user_language as i32,
            MenuItem::MachineId => state.settings.machine_id,
            MenuItem::CoinValue0 => *state
                .settings
                .coin_acceptor
                .coin_values
                .first()
                .unwrap_or(&0),
            MenuItem::HopperCoinValueA => state
                .settings
                .coin_hoppers
                .first()
                .map(|h| h.coin_value)
                .unwrap_or(0),
            MenuItem::ItemWarningLevel => state.settings.item_dispenser.warning_level,
            MenuItem::PhoneNumber00 => 0, // Телефон — не число
            _ => 0,
        }
    }

    /// Применить отредактированное значение (placeholder — запись в State)
    ///
    /// В реальной интеграции: запись в VendingState через Mutex
    fn apply_edit_value(&self, _state: &VendingState) {
        // Запись edit_value в соответствующее поле state через
        // Mutex<CriticalSectionRawMutex, RefCell<VendingState>>
        // и установка флага PersistReason::SettingsChanged
        // Текущая реализация: no-op, значение не сохраняется
    }

    /// Получить текст для отображения текущего пункта
    ///
    /// Возвращает (line1, line2) — две строки по 16 символов
    /// line1 — название пункта, line2 — значение
    pub fn display_text(&self, state: &VendingState) -> (String<40>, String<16>) {
        let mut line1 = String::new();
        let mut line2 = String::new();

        match self.current {
            MenuItem::CashOnHand => {
                let _ = line1.push_str("НАЛИЧНОСТЬ");
                write_i32(&mut line2, state.data.cash);
            }
            MenuItem::ItemLevel => {
                let _ = line1.push_str("ТОВАР");
                if self.editing {
                    write_i32(&mut line2, self.edit_value);
                } else {
                    write_i32(&mut line2, state.data.item_level);
                }
            }
            MenuItem::CoinLevelA => {
                let _ = line1.push_str("МОНЕТЫ A");
                if self.editing {
                    write_i32(&mut line2, self.edit_value);
                } else {
                    write_i32(&mut line2, *state.data.coin_levels.first().unwrap_or(&0));
                }
            }
            MenuItem::CoinLevelB => {
                let _ = line1.push_str("МОНЕТЫ B");
                if self.editing {
                    write_i32(&mut line2, self.edit_value);
                } else {
                    write_i32(&mut line2, *state.data.coin_levels.get(1).unwrap_or(&0));
                }
            }
            MenuItem::HopperTest => {
                let _ = line1.push_str("ТЕСТ ХОППЕРОВ");
                let _ = line2.push_str("Нажми OK");
            }
            MenuItem::DisplayTest => {
                let _ = line1.push_str("ТЕСТ ДИСПЛЕЯ");
                let _ = line2.push_str("Нажми OK");
            }
            MenuItem::GsmSettings => {
                let _ = line1.push_str("НАСТРОЙКИ GSM");
                let _ = line2.push_str("Нажми OK");
            }
            MenuItem::CoinCalibration => {
                let _ = line1.push_str("КАЛИБРОВКА МОН");
                let _ = line2.push_str("Нажми OK");
            }
            MenuItem::Language => {
                let _ = line1.push_str("ЯЗЫК");
                match state.settings.user_language {
                    crate::state::Language::Latvian => {
                        let _ = line2.push_str("Latvija");
                    }
                    crate::state::Language::Russian => {
                        let _ = line2.push_str("Русский");
                    }
                }
            }
            MenuItem::MachineId => {
                let _ = line1.push_str("ID АВТОМАТА");
                write_i32(&mut line2, state.settings.machine_id);
            }
            MenuItem::CoinValue0 => {
                let _ = line1.push_str("НОМИНАЛ Ch0");
                write_i32(
                    &mut line2,
                    *state
                        .settings
                        .coin_acceptor
                        .coin_values
                        .first()
                        .unwrap_or(&0),
                );
            }
            MenuItem::HopperCoinValueA => {
                let _ = line1.push_str("ХОППЕР A VAL");
                write_i32(
                    &mut line2,
                    state
                        .settings
                        .coin_hoppers
                        .first()
                        .map(|h| h.coin_value)
                        .unwrap_or(0),
                );
            }
            MenuItem::ItemWarningLevel => {
                let _ = line1.push_str("ПРЕДУПР.ТОВАР");
                write_i32(&mut line2, state.settings.item_dispenser.warning_level);
            }
            MenuItem::PhoneNumber00 => {
                let _ = line1.push_str("ТЕЛЕФОН 0");
                let _ = line2.push_str("(edit)");
            }
            MenuItem::SendReportState => {
                let _ = line1.push_str("ОТЧЁТ СОСТ.");
                let _ = line2.push_str("Нажми OK");
            }
            MenuItem::SendReportAccounting => {
                let _ = line1.push_str("ОТЧЁТ БУХГ.");
                let _ = line2.push_str("Нажми OK");
            }
            MenuItem::SendReportErrors => {
                let _ = line1.push_str("ОТЧЁТ ОШИБ.");
                let _ = line2.push_str("Нажми OK");
            }
            MenuItem::ClearErrors => {
                let _ = line1.push_str("СБРОС ОШИБОК");
                let _ = line2.push_str("Нажми OK");
            }
            MenuItem::ResetKeys => {
                let _ = line1.push_str("СБРОС КЛЮЧЕЙ");
                let _ = line2.push_str("Нажми OK");
            }
            MenuItem::Version => {
                let _ = line1.push_str("ВЕРСИЯ");
                write_i32(&mut line2, config::FIRMWARE_VERSION as i32);
            }
            MenuItem::GsmInfo => {
                let _ = line1.push_str("GSM ИНФО");
                let _ = line2.push_str("Нажми OK");
            }
            MenuItem::Count => {
                let _ = line1.push_str("---");
            }
        }

        // В режиме редактирования — добавить маркер
        if self.editing {
            let _ = line2.push('◄');
        }

        (line1, line2)
    }
}

// ── Вспомогательные функции ──────────────────────────────────────────────

fn write_i32(s: &mut String<16>, v: i32) {
    let _ = write!(s, "{}", v);
}
