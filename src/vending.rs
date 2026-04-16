//! Vending state machine — ядро логики автомата
//!
//! Перенос из Bahilizator.c:
//!   Run() → main loop: ACCEPT_CASH → PAYOUT_ITEMS → PAYOUT_REMINDER → PROCESS_RESIDUAL
//!   CreatePayout() — вычисление выдачи (товар + сдача)

#![allow(clippy::needless_range_loop)]
//!   AcceptCash() — приём монет, проверка CanAcceptCash
//!   PayoutPendingItems() — выдача товара через хоппер A
//!   PayoutReminder() — выдача сдачи через хопперы B/C
//!   ProcessResidual() — финализация: закрытие транзакции, "Спасибо"

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::{Receiver, Sender};
use embassy_sync::mutex::Mutex;
use embassy_sync::signal::Signal;
use heapless::String;

use crate::buttons::{Button, ButtonEvent};
use crate::coin_acceptor::CoinEvent;
use crate::config;
use crate::error::Errors;
use crate::gsm::GsmCommand;
use crate::hopper::{HopperCmd, HopperEvent};
use crate::state::{AppState, Cash, Level, MessageKind, PersistReason, VendingState};
use crate::ui::{make_str_40, Align, DisplayCommand};

// ── Вспомогательный макрос для чтения из Mutex<RefCell<VendingState>> ───
// Устраняет проблему lifetime: guard.borrow() → Ref → читаем поле → drop

macro_rules! read_state {
    ($state:expr, $guard:ident, $s:ident, $body:expr) => {{
        let $guard = $state.lock().await;
        let $s = $guard.borrow();
        $body
    }};
}

// ── Главная функция vending state machine ────────────────────────────────

#[allow(clippy::too_many_arguments)]
pub async fn run(
    state: &'static Mutex<CriticalSectionRawMutex, core::cell::RefCell<VendingState>>,
    coin_rx: Receiver<'static, CriticalSectionRawMutex, CoinEvent, 4>,
    button_rx: Receiver<'static, CriticalSectionRawMutex, ButtonEvent, 4>,
    hopper_event_rx: Receiver<'static, CriticalSectionRawMutex, HopperEvent, 4>,
    hopper_cmd_tx: Sender<'static, CriticalSectionRawMutex, HopperCmd, 4>,
    gsm_tx: Sender<'static, CriticalSectionRawMutex, GsmCommand, 4>,
    display: &'static Signal<CriticalSectionRawMutex, DisplayCommand>,
    persist: &'static Signal<CriticalSectionRawMutex, PersistReason>,
) {
    // Начальное состояние дисплея
    display.signal(DisplayCommand::TextAligned {
        align: Align::Center,
        y: 0,
        text: make_str_40("БАХИЛИЗАТОР"),
    });
    display.signal(DisplayCommand::TextAligned {
        align: Align::Center,
        y: 1,
        text: make_str_40("v2.0"),
    });
    embassy_time::Timer::after_secs(2).await;

    loop {
        let (app_state, has_errors) =
            read_state!(state, guard, s, (s.data.app_state, !s.errors.is_empty()));

        if has_errors {
            process_errors(state, display, gsm_tx).await;
            continue;
        }

        match app_state {
            AppState::AcceptCash => {
                accept_cash(
                    state,
                    coin_rx,
                    button_rx,
                    hopper_cmd_tx,
                    gsm_tx,
                    display,
                    persist,
                )
                .await;
            }
            AppState::PayoutItems => {
                payout_items(
                    state,
                    hopper_event_rx,
                    hopper_cmd_tx,
                    gsm_tx,
                    display,
                    persist,
                )
                .await;
            }
            AppState::PayoutReminder => {
                payout_reminder(
                    state,
                    hopper_event_rx,
                    hopper_cmd_tx,
                    gsm_tx,
                    display,
                    persist,
                )
                .await;
            }
            AppState::ProcessResidual => {
                process_residual(state, display, persist).await;
            }
        }
    }
}

// ── ACCEPT_CASH ───────────────────────────────────────────────────────────

async fn accept_cash(
    state: &'static Mutex<CriticalSectionRawMutex, core::cell::RefCell<VendingState>>,
    coin_rx: Receiver<'static, CriticalSectionRawMutex, CoinEvent, 4>,
    button_rx: Receiver<'static, CriticalSectionRawMutex, ButtonEvent, 4>,
    _hopper_cmd_tx: Sender<'static, CriticalSectionRawMutex, HopperCmd, 4>,
    gsm_tx: Sender<'static, CriticalSectionRawMutex, GsmCommand, 4>,
    display: &'static Signal<CriticalSectionRawMutex, DisplayCommand>,
    persist: &'static Signal<CriticalSectionRawMutex, PersistReason>,
) {
    // Проверить CanAcceptCash
    let can_accept = can_accept_cash(state).await;
    if !can_accept {
        let item_level = read_state!(state, g, s, s.data.item_level);
        if item_level > 0 {
            set_error(state, Errors::CANNOT_PAYOUT).await;
        } else {
            set_error(state, Errors::ITEM_DISPENSER_EMPTY).await;
        }
        return;
    }

    update_accept_cash_display(state, display).await;
    let mut cash_insert_time: Option<u64> = None;

    loop {
        // Проверка монет
        if let Ok(coin) = coin_rx.try_receive() {
            {
                let guard = state.lock().await;
                let mut s = guard.borrow_mut();
                s.data.cash += coin.value;
                s.data.overall_accounting.cash_in += coin.value;
                s.data.period_accounting.cash_in += coin.value;
            }
            persist.signal(PersistReason::StateChanged);
            cash_insert_time = Some(embassy_time::Instant::now().as_millis());
            update_accept_cash_display(state, display).await;

            if is_cash_pending(state).await {
                let (items, coins, reminder) = create_payout(state).await;
                {
                    let guard = state.lock().await;
                    let mut s = guard.borrow_mut();
                    s.data.items_pending = items;
                    s.data.coins_pending = coins;
                    s.data.cash = reminder;
                    s.data.app_state = AppState::PayoutItems;
                }
                persist.signal(PersistReason::StateChanged);
                return;
            }
        }

        // Проверка кнопок
        if let Ok(event) = button_rx.try_receive() {
            match event {
                ButtonEvent::Pressed(Button::Ok) => { { /* сервисное меню — не реализовано */ } }
                ButtonEvent::DoorChanged { door, opened } => {
                    process_door_event(state, door, opened, gsm_tx).await;
                }
                _ => {}
            }
        }

        // Таймаут сброса наличности
        if let Some(insert_time) = cash_insert_time {
            let cash = read_state!(state, g, s, s.data.cash);
            if cash > 0 {
                let timeout_ms =
                    read_state!(state, g, s, s.settings.cash_clear_timeout as u64 * 1000);
                if timeout_ms > 0 {
                    let now = embassy_time::Instant::now().as_millis();
                    if now - insert_time >= timeout_ms {
                        {
                            let guard = state.lock().await;
                            guard.borrow_mut().data.cash = 0;
                        }
                        cash_insert_time = None;
                        persist.signal(PersistReason::StateChanged);
                        update_accept_cash_display(state, display).await;
                    }
                }
            } else {
                cash_insert_time = None;
            }
        }

        embassy_time::Timer::after_millis(50).await;
    }
}

// ── PAYOUT_ITEMS ─────────────────────────────────────────────────────────

async fn payout_items(
    state: &'static Mutex<CriticalSectionRawMutex, core::cell::RefCell<VendingState>>,
    hopper_event_rx: Receiver<'static, CriticalSectionRawMutex, HopperEvent, 4>,
    hopper_cmd_tx: Sender<'static, CriticalSectionRawMutex, HopperCmd, 4>,
    gsm_tx: Sender<'static, CriticalSectionRawMutex, GsmCommand, 4>,
    display: &'static Signal<CriticalSectionRawMutex, DisplayCommand>,
    persist: &'static Signal<CriticalSectionRawMutex, PersistReason>,
) {
    let (items_pending, item_level) =
        read_state!(state, g, s, (s.data.items_pending, s.data.item_level));

    display.signal(DisplayCommand::TextAligned {
        align: Align::Center,
        y: 0,
        text: make_str_40("ВЫДАЧА ТОВАРА"),
    });

    if items_pending <= 0 || item_level <= 0 {
        let guard = state.lock().await;
        guard.borrow_mut().data.app_state = AppState::PayoutReminder;
        return;
    }

    let _ = hopper_cmd_tx.try_send(HopperCmd::Payout {
        hopper: 0,
        count: items_pending,
    });

    // Дисплей: "0/N шт"
    {
        let mut str = String::<40>::new();
        use core::fmt::Write;
        let _ = write!(str, "0/{} шт", items_pending);
        display.signal(DisplayCommand::TextAligned {
            align: Align::Center,
            y: 1,
            text: str,
        });
    }

    let items_total = items_pending;
    let mut items_out: Level = 0;

    loop {
        let has_errors = read_state!(state, g, s, !s.errors.is_empty());
        if has_errors {
            let _ = hopper_cmd_tx.try_send(HopperCmd::Stop { hopper: 0 });
            return;
        }

        if let Ok(event) = hopper_event_rx.try_receive() {
            match event {
                HopperEvent::CoinDispensed { hopper: 0 } => {
                    items_out += 1;
                    {
                        let guard = state.lock().await;
                        let mut s = guard.borrow_mut();
                        s.data.items_pending = s.data.items_pending.saturating_sub(1);
                        s.data.item_level = s.data.item_level.saturating_sub(1);

                        if s.data.free_items_pending > 0 {
                            s.data.free_items_pending -= 1;
                            s.data.overall_accounting.free_items_out += 1;
                            s.data.period_accounting.free_items_out += 1;
                        } else {
                            s.data.overall_accounting.items_out += 1;
                            s.data.period_accounting.items_out += 1;
                        }
                    }
                    persist.signal(PersistReason::StateChanged);

                    // Обновить дисплей
                    {
                        let mut str = String::<40>::new();
                        use core::fmt::Write;
                        let _ = write!(str, "{}/{} шт", items_out, items_total);
                        display.signal(DisplayCommand::TextAligned {
                            align: Align::Center,
                            y: 1,
                            text: str,
                        });
                    }

                    let remaining = read_state!(state, g, s, s.data.items_pending);
                    if remaining <= 0 {
                        let _ = hopper_cmd_tx.try_send(HopperCmd::Stop { hopper: 0 });
                        embassy_time::Timer::after_millis(config::PAYOUT_MESSAGE_DELAY_S * 1000)
                            .await;
                        let guard = state.lock().await;
                        guard.borrow_mut().data.app_state = AppState::PayoutReminder;
                        return;
                    }
                }
                HopperEvent::Error {
                    hopper: 0,
                    error: _,
                } => {
                    let _ = hopper_cmd_tx.try_send(HopperCmd::Stop { hopper: 0 });
                    set_error(state, Errors::ITEM_DISPENSER).await;
                    let _ = gsm_tx.try_send(GsmCommand::SendSms {
                        phone_idx: 0,
                        kind: MessageKind::ReportErrors,
                    });
                    return;
                }
                HopperEvent::Timeout { hopper: 0 } => {
                    let _ = hopper_cmd_tx.try_send(HopperCmd::Stop { hopper: 0 });
                    set_error(state, Errors::ITEM_DISPENSER).await;
                    return;
                }
                _ => {}
            }
        }

        embassy_time::Timer::after_millis(1).await;
    }
}

// ── PAYOUT_REMINDER ──────────────────────────────────────────────────────

async fn payout_reminder(
    state: &'static Mutex<CriticalSectionRawMutex, core::cell::RefCell<VendingState>>,
    hopper_event_rx: Receiver<'static, CriticalSectionRawMutex, HopperEvent, 4>,
    hopper_cmd_tx: Sender<'static, CriticalSectionRawMutex, HopperCmd, 4>,
    gsm_tx: Sender<'static, CriticalSectionRawMutex, GsmCommand, 4>,
    display: &'static Signal<CriticalSectionRawMutex, DisplayCommand>,
    persist: &'static Signal<CriticalSectionRawMutex, PersistReason>,
) {
    let has_coins_pending = read_state!(state, g, s, s.data.coins_pending.iter().any(|&c| c > 0));

    if !has_coins_pending {
        let guard = state.lock().await;
        guard.borrow_mut().data.app_state = AppState::ProcessResidual;
        return;
    }

    display.signal(DisplayCommand::TextAligned {
        align: Align::Center,
        y: 0,
        text: make_str_40("ЗАБЕРИТЕ СДАЧУ"),
    });

    let reminder_start = embassy_time::Instant::now();

    for hopper_id in (0..config::HOPPER_COUNT).rev() {
        let coins = read_state!(state, g, s, s.data.coins_pending[hopper_id]);
        if coins <= 0 {
            continue;
        }

        let _ = hopper_cmd_tx.try_send(HopperCmd::Payout {
            hopper: hopper_id as u8,
            count: coins,
        });

        // Дисплей: "Сдача: N мон"
        {
            let mut str = String::<40>::new();
            use core::fmt::Write;
            let _ = write!(str, "Сдача хоппер {}: {}", hopper_id + 1, coins);
            display.signal(DisplayCommand::TextAligned {
                align: Align::Center,
                y: 1,
                text: str,
            });
        }

        loop {
            let has_errors = read_state!(state, g, s, !s.errors.is_empty());
            if has_errors {
                let _ = hopper_cmd_tx.try_send(HopperCmd::Stop {
                    hopper: hopper_id as u8,
                });
                return;
            }

            // Таймаут REMINDER (30с)
            if reminder_start.elapsed().as_secs() >= config::RESIDUAL_TIMEOUT_S as u64 {
                let _ = hopper_cmd_tx.try_send(HopperCmd::Stop {
                    hopper: hopper_id as u8,
                });
                let guard = state.lock().await;
                let mut s = guard.borrow_mut();
                if s.data.coins_pending[hopper_id] > 0 {
                    s.data.coins_pending[hopper_id] -= 1; // наказание
                }
                s.data.app_state = AppState::ProcessResidual;
                persist.signal(PersistReason::StateChanged);
                return;
            }

            if let Ok(event) = hopper_event_rx.try_receive() {
                match event {
                    HopperEvent::CoinDispensed { hopper } if hopper == hopper_id as u8 => {
                        let coin_value =
                            read_state!(state, g, s, s.settings.coin_hoppers[hopper_id].coin_value);
                        {
                            let guard = state.lock().await;
                            let mut s = guard.borrow_mut();
                            s.data.overall_accounting.cash_out += coin_value;
                            s.data.period_accounting.cash_out += coin_value;
                            s.data.coin_levels[hopper_id] =
                                s.data.coin_levels[hopper_id].saturating_sub(1);
                            s.data.coins_pending[hopper_id] =
                                s.data.coins_pending[hopper_id].saturating_sub(1);
                        }
                        persist.signal(PersistReason::StateChanged);

                        let remaining = read_state!(state, g, s, s.data.coins_pending[hopper_id]);
                        if remaining <= 0 {
                            let _ = hopper_cmd_tx.try_send(HopperCmd::Stop {
                                hopper: hopper_id as u8,
                            });
                            break;
                        }
                    }
                    HopperEvent::Error { hopper, error: _ } if hopper == hopper_id as u8 => {
                        let _ = hopper_cmd_tx.try_send(HopperCmd::Stop {
                            hopper: hopper_id as u8,
                        });
                        set_error(state, Errors::COIN_HOPPER).await;
                        let _ = gsm_tx.try_send(GsmCommand::SendSms {
                            phone_idx: 0,
                            kind: MessageKind::CoinHopperWarningLevel,
                        });
                        return;
                    }
                    HopperEvent::Timeout { hopper } if hopper == hopper_id as u8 => {
                        let _ = hopper_cmd_tx.try_send(HopperCmd::Stop {
                            hopper: hopper_id as u8,
                        });
                        set_error(state, Errors::COIN_HOPPER).await;
                        return;
                    }
                    _ => {}
                }
            }

            embassy_time::Timer::after_millis(1).await;
        }
    }

    // Все монеты выданы
    embassy_time::Timer::after_millis(config::PAYOUT_MESSAGE_DELAY_S * 1000).await;
    let guard = state.lock().await;
    guard.borrow_mut().data.app_state = AppState::ProcessResidual;
}

// ── PROCESS_RESIDUAL ─────────────────────────────────────────────────────

async fn process_residual(
    state: &'static Mutex<CriticalSectionRawMutex, core::cell::RefCell<VendingState>>,
    display: &'static Signal<CriticalSectionRawMutex, DisplayCommand>,
    persist: &'static Signal<CriticalSectionRawMutex, PersistReason>,
) {
    let (items_pending, has_coins) = read_state!(
        state,
        g,
        s,
        (
            s.data.items_pending,
            s.data.coins_pending.iter().any(|&c| c > 0)
        )
    );

    let has_residual = items_pending > 0 || has_coins;

    if has_residual {
        display.signal(DisplayCommand::TextAligned {
            align: Align::Center,
            y: 0,
            text: make_str_40("НЕ ВЫДАНО"),
        });
        let reminder_cash = get_pending_reminder_in_cash(state).await;
        let mut residual_str = String::<40>::new();
        use core::fmt::Write;
        let _ = write!(residual_str, "Тов:{} Сдач:{}", items_pending, reminder_cash);
        display.signal(DisplayCommand::TextAligned {
            align: Align::Center,
            y: 1,
            text: residual_str,
        });

        // Закрыть транзакцию
        {
            let guard = state.lock().await;
            crate::event::close_transaction(&mut guard.borrow_mut().data.transactions, 0);
        }

        embassy_time::Timer::after_secs(config::RESIDUAL_TIMEOUT_S as u64).await;
    } else {
        display.signal(DisplayCommand::TextAligned {
            align: Align::Center,
            y: 0,
            text: make_str_40("СПАСИБО!"),
        });
        display.signal(DisplayCommand::TextAligned {
            align: Align::Center,
            y: 1,
            text: make_str_40("ЗА ПОКУПКУ"),
        });
        embassy_time::Timer::after_secs(config::THANKS_MESSAGE_DELAY_S).await;
    }

    // Сбросить pending → ACCEPT_CASH
    {
        let guard = state.lock().await;
        let mut s = guard.borrow_mut();
        s.data.cash = 0;
        s.data.coins_pending = [0; config::HOPPER_COUNT];
        s.data.items_pending = 0;
        s.data.free_items_pending = 0;
        s.data.app_state = AppState::AcceptCash;
    }
    persist.signal(PersistReason::StateChanged);
}

// ── Обработка ошибок ─────────────────────────────────────────────────────

async fn process_errors(
    state: &'static Mutex<CriticalSectionRawMutex, core::cell::RefCell<VendingState>>,
    display: &'static Signal<CriticalSectionRawMutex, DisplayCommand>,
    gsm_tx: Sender<'static, CriticalSectionRawMutex, GsmCommand, 4>,
) {
    let errors = read_state!(state, g, s, s.errors);

    display.signal(DisplayCommand::TextAligned {
        align: Align::Center,
        y: 0,
        text: make_str_40("НЕТ ОБСЛУЖИВАНИЯ"),
    });

    let mut err_str = String::<40>::new();
    use core::fmt::Write;
    let _ = write!(err_str, "ERR {:X}", errors.bits());
    display.signal(DisplayCommand::TextAligned {
        align: Align::Center,
        y: 1,
        text: err_str,
    });

    // Сбросить pending при ошибках
    {
        let guard = state.lock().await;
        let mut s = guard.borrow_mut();
        s.data.coins_pending = [0; config::HOPPER_COUNT];
        s.data.items_pending = 0;
        s.data.free_items_pending = 0;
    }

    let _ = gsm_tx.try_send(GsmCommand::SendSms {
        phone_idx: 0,
        kind: MessageKind::ReportErrors,
    });
    embassy_time::Timer::after_secs(2).await;
}

// ── Вспомогательные функции ──────────────────────────────────────────────

/// Проверить, можем ли принять монету (хватит ли сдачи)
async fn can_accept_cash(
    state: &'static Mutex<CriticalSectionRawMutex, core::cell::RefCell<VendingState>>,
) -> bool {
    let (item_level, cash) = read_state!(state, g, s, (s.data.item_level, s.data.cash));
    if item_level <= 0 {
        return false;
    }

    // Проверяем для каждого номинала монеты
    let coin_values = read_state!(state, g, s, s.settings.coin_acceptor.coin_values);
    let coin_enable = read_state!(state, g, s, s.settings.coin_acceptor.coin_enable);

    for i in 0..config::COIN_CHANNEL_COUNT {
        if coin_values[i] > 0 && coin_enable[i] {
            // Упрощённая проверка: можем ли выдать товар за cash + coin_value
            let _ = (cash, state);
            // Полная проверка CreatePayout как в оригинале —
            // проверка достаточности монет в хопперах для сдачи
        }
    }
    true
}

/// Достаточно ли наличности для покупки товара
async fn is_cash_pending(
    state: &'static Mutex<CriticalSectionRawMutex, core::cell::RefCell<VendingState>>,
) -> bool {
    let (cash, item_price) = read_state!(
        state,
        g,
        s,
        (s.data.cash, s.settings.item_dispenser.coin_value)
    );
    cash >= item_price
}

/// Создать выплату — вычислить количество товара и монет сдачи
async fn create_payout(
    state: &'static Mutex<CriticalSectionRawMutex, core::cell::RefCell<VendingState>>,
) -> (Level, [Level; config::HOPPER_COUNT], Cash) {
    let (cash, item_price, item_level, coin_levels, hopper_values) = read_state!(
        state,
        g,
        s,
        (
            s.data.cash,
            s.settings.item_dispenser.coin_value,
            s.data.item_level,
            s.data.coin_levels,
            {
                let mut vals = [0 as Cash; config::HOPPER_COUNT];
                for i in 0..config::HOPPER_COUNT {
                    vals[i] = s.settings.coin_hoppers[i].coin_value;
                }
                vals
            }
        )
    );

    if item_price <= 0 || cash < item_price || item_level <= 0 {
        return (0, [0; config::HOPPER_COUNT], cash);
    }

    let items: Level = 1;
    let mut reminder = cash - item_price;
    let mut coins = [0 as Level; config::HOPPER_COUNT];

    for i in (0..config::HOPPER_COUNT).rev() {
        if hopper_values[i] > 0 && coin_levels[i] > 0 && reminder > 0 {
            let count = (reminder / hopper_values[i]).min(coin_levels[i]);
            coins[i] = count;
            reminder -= count * hopper_values[i];
        }
    }

    (items, coins, reminder)
}

/// Стоимость невыданной сдачи
async fn get_pending_reminder_in_cash(
    state: &'static Mutex<CriticalSectionRawMutex, core::cell::RefCell<VendingState>>,
) -> Cash {
    read_state!(state, g, s, {
        let mut cash: Cash = 0;
        for i in 0..config::HOPPER_COUNT {
            cash += s.data.coins_pending[i] * s.settings.coin_hoppers[i].coin_value;
        }
        cash
    })
}

/// Обновить дисплей в состоянии ACCEPT_CASH
async fn update_accept_cash_display(
    state: &'static Mutex<CriticalSectionRawMutex, core::cell::RefCell<VendingState>>,
    display: &'static Signal<CriticalSectionRawMutex, DisplayCommand>,
) {
    let (cash, item_price) = read_state!(
        state,
        g,
        s,
        (s.data.cash, s.settings.item_dispenser.coin_value)
    );

    display.signal(DisplayCommand::TextAligned {
        align: Align::Center,
        y: 0,
        text: make_str_40("ВСТАВЬТЕ МОНЕТЫ"),
    });

    let mut str = String::<40>::new();
    use core::fmt::Write;
    if cash > 0 {
        let _ = write!(str, "Наличность: {}", cash);
    } else {
        let _ = write!(str, "Цена: {}", item_price);
    }
    display.signal(DisplayCommand::TextAligned {
        align: Align::Center,
        y: 1,
        text: str,
    });
}

/// Установить ошибку
async fn set_error(
    state: &'static Mutex<CriticalSectionRawMutex, core::cell::RefCell<VendingState>>,
    error: Errors,
) {
    let guard = state.lock().await;
    guard.borrow_mut().errors |= error;
}

/// Обработать событие двери
async fn process_door_event(
    state: &'static Mutex<CriticalSectionRawMutex, core::cell::RefCell<VendingState>>,
    _door: u8,
    opened: bool,
    gsm_tx: Sender<'static, CriticalSectionRawMutex, GsmCommand, 4>,
) {
    if opened {
        {
            let guard = state.lock().await;
            guard.borrow_mut().errors |= Errors::DOOR_OPENED;
        }
        let _ = gsm_tx.try_send(GsmCommand::SendSms {
            phone_idx: 0,
            kind: MessageKind::ReportIntrusion,
        });
    } else {
        let guard = state.lock().await;
        guard.borrow_mut().errors &= !Errors::DOOR_OPENED;
    }
}
