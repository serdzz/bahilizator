//! GSM менеджер — инициализация, SMS, питание

pub mod at_channel;
pub mod cmux;
pub mod ppp_channel;
pub mod sms;

use embassy_sync::channel::{Receiver, Sender};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;

use crate::state::VendingState;

// ── GsmCommand ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, defmt::Format)]
pub enum GsmCommand {
    SendSms { phone_idx: u8, kind: crate::state::MessageKind },
    GetTime,
    PowerDown,
}

// ── SmsEvent ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct SmsEvent {
    pub sender: heapless::String<17>,
    pub message: heapless::String<160>,
}

// ── Задача GSM ───────────────────────────────────────────────────────────

pub async fn run(
    gsm_cmd_rx: Receiver<'static, CriticalSectionRawMutex, GsmCommand, 4>,
    _sms_event_tx: Sender<'static, CriticalSectionRawMutex, SmsEvent, 4>,
    _state: &'static Mutex<CriticalSectionRawMutex, core::cell::RefCell<VendingState>>,
) {
    // Подать питание на модем
    gsm_power_on().await;
    embassy_time::Timer::after_secs(3).await;

    // Инициализация AT команд
    // TODO: реальная инициализация

    loop {
        match gsm_cmd_rx.try_receive() {
            Ok(GsmCommand::SendSms { phone_idx, kind }) => {
                let _ = (phone_idx, kind);
                // TODO: отправить SMS
            }
            Ok(GsmCommand::GetTime) => {
                // TODO: запросить CCLK?
            }
            Ok(GsmCommand::PowerDown) => {
                // TODO: AT+CPOWD=1
            }
            Err(_) => {}
        }

        embassy_time::Timer::after_millis(10).await;
    }
}

async fn gsm_power_on() {
    // TODO: PWRKEY pulse
}