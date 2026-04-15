//! Драйвер монетоприёмника
//!
//! EXTI ISR → Channel для нормального режима
//! Pulse counting для пульсного режима

use embassy_sync::channel::{Channel, Sender};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;

use crate::state::{Cash, VendingState};
use crate::config;

// ── CoinEvent ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, defmt::Format)]
pub struct CoinEvent {
    pub channel: u8,
    pub value: Cash,
}

// ── Режим работы ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
enum CoinMode {
    Normal,
    Pulse,
}

// ── Задача монетоприёмника ────────────────────────────────────────────────

pub async fn run(
    coin_tx: Sender<'static, CriticalSectionRawMutex, CoinEvent, 4>,
    state: &'static Mutex<CriticalSectionRawMutex, core::cell::RefCell<VendingState>>,
) {
    let mode = {
        let guard = state.lock().await;
        if guard.borrow().settings.coin_acceptor.pulse_mode {
            CoinMode::Pulse
        } else {
            CoinMode::Normal
        }
    };

    let mut pulse_count: u16 = 0;
    let mut last_pulse_ms: u64 = 0;

    loop {
        match mode {
            CoinMode::Normal => {
                // TODO: подключить EXTI interrupt handler
                embassy_time::Timer::after_millis(10).await;
            }
            CoinMode::Pulse => {
                let now = embassy_time::Instant::now().as_millis();

                if pulse_count > 0 && now - last_pulse_ms > 200 {
                    let channel = (pulse_count as usize).min(config::COIN_CHANNEL_COUNT - 1);
                    let value = {
                        let guard = state.lock().await;
                        let st = guard.borrow();
                        st.settings.coin_acceptor.coin_values[channel]
                    };

                    if value > 0 {
                        let _ = coin_tx.try_send(CoinEvent {
                            channel: channel as u8,
                            value,
                        });
                    }
                    pulse_count = 0;
                }

                embassy_time::Timer::after_millis(1).await;
            }
        }
    }
}