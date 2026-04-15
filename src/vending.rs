//! Vending state machine — ядро логики автомата
//!
//! Состояния: ACCEPT_CASH → PAYOUT_ITEMS → PAYOUT_REMINDER → PROCESS_RESIDUAL

use embassy_sync::channel::{Receiver, Sender};
use embassy_sync::signal::Signal;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use heapless::String;

use crate::buttons::{Button, ButtonEvent};
use crate::coin_acceptor::CoinEvent;
use crate::config;
use crate::error::Errors;
use crate::gsm::GsmCommand;
use crate::hopper::HopperEvent;
use crate::state::{AppState, Cash, Level, PersistReason, VendingState};
use crate::ui::{DisplayCommand, make_str_40};

// ── Главная функция vending state machine ────────────────────────────────

pub async fn run(
    state: &'static Mutex<CriticalSectionRawMutex, core::cell::RefCell<VendingState>>,
    coin_rx: Receiver<'static, CriticalSectionRawMutex, CoinEvent, 4>,
    button_rx: Receiver<'static, CriticalSectionRawMutex, ButtonEvent, 4>,
    hopper_rx: Receiver<'static, CriticalSectionRawMutex, HopperEvent, 4>,
    gsm_tx: Sender<'static, CriticalSectionRawMutex, GsmCommand, 4>,
    display: &'static Signal<CriticalSectionRawMutex, DisplayCommand>,
    persist: &'static Signal<CriticalSectionRawMutex, PersistReason>,
) {
    // Начальное состояние дисплея
    display.signal(DisplayCommand::TextAligned {
        align: crate::ui::Align::Center,
        y: 0,
        text: make_str_40("БАХИЛИЗАТОР"),
    });
    embassy_time::Timer::after_secs(2).await;

    loop {
        let (app_state, errors) = {
            let guard = state.lock().await;
            let s = guard.borrow();
            (s.data.app_state, s.errors)
        };

        if !errors.is_empty() {
            process_errors(state, display).await;
            continue;
        }

        match app_state {
            AppState::AcceptCash => {
                accept_cash(state, coin_rx.clone(), button_rx.clone(), display, gsm_tx.clone(), persist).await;
            }
            AppState::PayoutItems => {
                payout_items(state, hopper_rx.clone(), display, persist).await;
            }
            AppState::PayoutReminder => {
                payout_reminder(state, hopper_rx.clone(), display).await;
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
    display: &'static Signal<CriticalSectionRawMutex, DisplayCommand>,
    gsm_tx: Sender<'static, CriticalSectionRawMutex, GsmCommand, 4>,
    persist: &'static Signal<CriticalSectionRawMutex, PersistReason>,
) {
    update_cash_display(state, display);

    loop {
        if let Ok(coin) = coin_rx.try_receive() {
            {
                let mut guard = state.lock().await;
                let mut s = guard.borrow_mut();
                s.data.cash += coin.value;
            }
            persist.signal(PersistReason::StateChanged);
            update_cash_display(state, display);

            if can_create_payout(state).await {
                let (items, coins) = create_payout(state).await;
                {
                    let mut guard = state.lock().await;
                    let mut s = guard.borrow_mut();
                    s.data.items_pending = items;
                    s.data.coins_pending = coins;
                    s.data.app_state = AppState::PayoutItems;
                }
                return;
            }
        }

        if let Ok(event) = button_rx.try_receive() {
            match event {
                ButtonEvent::Pressed(Button::Ok) => { /* TODO: войти в меню */ }
                ButtonEvent::DoorChanged { door, opened } => {
                    process_door_event(state, door, opened, gsm_tx.clone()).await;
                }
                _ => {}
            }
        }

        embassy_time::Timer::after_millis(50).await;
    }
}

// ── PAYOUT_ITEMS ─────────────────────────────────────────────────────────

async fn payout_items(
    state: &'static Mutex<CriticalSectionRawMutex, core::cell::RefCell<VendingState>>,
    _hopper_rx: Receiver<'static, CriticalSectionRawMutex, HopperEvent, 4>,
    display: &'static Signal<CriticalSectionRawMutex, DisplayCommand>,
    persist: &'static Signal<CriticalSectionRawMutex, PersistReason>,
) {
    let items_pending = {
        let guard = state.lock().await;
        let s = guard.borrow();
        s.data.items_pending.clone()
    };

    display.signal(DisplayCommand::TextAligned {
        align: crate::ui::Align::Center,
        y: 0,
        text: make_str_40("ВЫДАЧА ТОВАРА"),
    });

    // TODO: реальная выдача товара через хоппер

    {
        let mut guard = state.lock().await;
        let mut s = guard.borrow_mut();
        s.data.item_level = s.data.item_level.saturating_sub(s.data.items_pending);
        s.data.cash = s.data.cash.saturating_sub(s.data.items_pending as Cash);
        s.data.overall_accounting.items_out += items_pending;
        s.data.period_accounting.items_out += items_pending;
        s.data.items_pending = 0;
    }

    persist.signal(PersistReason::StateChanged);

    let has_change = {
        let guard = state.lock().await;
        let s = guard.borrow();
        s.data.cash > 0 || s.data.coins_pending.iter().any(|&c| c > 0)
    };

    if has_change {
        let mut guard = state.lock().await;
        guard.borrow_mut().data.app_state = AppState::PayoutReminder;
    } else {
        let mut guard = state.lock().await;
        guard.borrow_mut().data.app_state = AppState::ProcessResidual;
    }
}

// ── PAYOUT_REMINDER ──────────────────────────────────────────────────────

async fn payout_reminder(
    state: &'static Mutex<CriticalSectionRawMutex, core::cell::RefCell<VendingState>>,
    _hopper_rx: Receiver<'static, CriticalSectionRawMutex, HopperEvent, 4>,
    display: &'static Signal<CriticalSectionRawMutex, DisplayCommand>,
) {
    display.signal(DisplayCommand::TextAligned {
        align: crate::ui::Align::Center,
        y: 0,
        text: make_str_40("ЗАБЕРИТЕ СДАЧУ"),
    });

    let cash = {
        let guard = state.lock().await;
        let s = guard.borrow();
        s.data.cash
    };

    if cash > 0 {
        let mut cash_str = String::<40>::new();
        use core::fmt::Write;
        let _ = write!(cash_str, "{}", cash);
        display.signal(DisplayCommand::TextAligned {
            align: crate::ui::Align::Center,
            y: 1,
            text: cash_str,
        });
    }

    // TODO: реальная выдача сдачи

    {
        let mut guard = state.lock().await;
        guard.borrow_mut().data.app_state = AppState::ProcessResidual;
    }
}

// ── PROCESS_RESIDUAL ─────────────────────────────────────────────────────

async fn process_residual(
    state: &'static Mutex<CriticalSectionRawMutex, core::cell::RefCell<VendingState>>,
    display: &'static Signal<CriticalSectionRawMutex, DisplayCommand>,
    persist: &'static Signal<CriticalSectionRawMutex, PersistReason>,
) {
    display.signal(DisplayCommand::TextAligned {
        align: crate::ui::Align::Center,
        y: 0,
        text: make_str_40("СПАСИБО!"),
    });

    embassy_time::Timer::after_secs(config::THANKS_MESSAGE_DELAY_S).await;

    {
        let mut guard = state.lock().await;
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
) {
    let errors = {
        let guard = state.lock().await;
        let s = guard.borrow();
        s.errors
    };

    let mut err_str = String::<40>::new();
    use core::fmt::Write;
    let _ = write!(err_str, "ERR {:X}", errors.bits());

    display.signal(DisplayCommand::TextAligned {
        align: crate::ui::Align::Center,
        y: 0,
        text: make_str_40("ОШИБКА"),
    });
    display.signal(DisplayCommand::TextAligned {
        align: crate::ui::Align::Center,
        y: 1,
        text: err_str,
    });

    embassy_time::Timer::after_secs(2).await;
}

// ── Вспомогательные функции ──────────────────────────────────────────────

fn update_cash_display(
    state: &'static Mutex<CriticalSectionRawMutex, core::cell::RefCell<VendingState>>,
    display: &'static Signal<CriticalSectionRawMutex, DisplayCommand>,
) {
    // Используем try_lock чтобы не await в не-async контексте
    // Но мы уже в async, так что можно lock
    // Однако signal не &self а &'static — можно послать сигнал из sync
    // Упрощаем: читаем cash через отдельную async операцию
    // Для signal не нужен lock — просто отправляем команды
    display.signal(DisplayCommand::TextAligned {
        align: crate::ui::Align::Left,
        y: 0,
        text: make_str_40("ВСТАВЬТЕ МОНЕТЫ"),
    });
}

async fn can_create_payout(
    state: &'static Mutex<CriticalSectionRawMutex, core::cell::RefCell<VendingState>>,
) -> bool {
    let guard = state.lock().await;
    let s = guard.borrow();
    let cash = s.data.cash;
    let item_price = s.settings.item_dispenser.coin_value;
    cash >= item_price && s.data.item_level > 0
}

async fn create_payout(
    state: &'static Mutex<CriticalSectionRawMutex, core::cell::RefCell<VendingState>>,
) -> (Level, [Level; config::HOPPER_COUNT]) {
    let guard = state.lock().await;
    let s = guard.borrow();
    let item_price = s.settings.item_dispenser.coin_value;
    let items = if item_price > 0 {
        (s.data.cash / item_price).min(s.data.item_level)
    } else {
        0 as Level
    };
    let remaining = s.data.cash - items * item_price;
    let mut coins = [0 as Level; config::HOPPER_COUNT];
    if config::HOPPER_COUNT > 0 && s.settings.coin_hoppers[0].coin_value > 0 {
        coins[0] = remaining / s.settings.coin_hoppers[0].coin_value;
    }
    if config::HOPPER_COUNT > 1 && s.settings.coin_hoppers[1].coin_value > 0 {
        let after_h0 = remaining - coins[0] * s.settings.coin_hoppers[0].coin_value;
        coins[1] = after_h0 / s.settings.coin_hoppers[1].coin_value;
    }
    (items, coins)
}

async fn process_door_event(
    state: &'static Mutex<CriticalSectionRawMutex, core::cell::RefCell<VendingState>>,
    _door: u8,
    opened: bool,
    gsm_tx: Sender<'static, CriticalSectionRawMutex, GsmCommand, 4>,
) {
    if opened {
        {
            let mut guard = state.lock().await;
            guard.borrow_mut().errors |= Errors::DOOR_OPENED;
        }
        let _ = gsm_tx.try_send(GsmCommand::SendSms {
            phone_idx: 0,
            kind: crate::state::MessageKind::ReportIntrusion,
        });
    } else {
        let mut guard = state.lock().await;
        guard.borrow_mut().errors &= !Errors::DOOR_OPENED;
    }
}