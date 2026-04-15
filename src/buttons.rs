//! Кнопки и датчики дверей
//!
//! EXTI ISR → Channel для событий кнопок и дверей
//! Перенос из io.c

use embassy_sync::channel::{Channel, Sender};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;

// ── Кнопки ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, defmt::Format)]
pub enum Button {
    Prev,
    Next,
    Ok,
    Cancel,
}

// ── События кнопок и дверей ───────────────────────────────────────────────

#[derive(Debug, Clone, Copy, defmt::Format)]
pub enum ButtonEvent {
    Pressed(Button),
    DoorChanged { door: u8, opened: bool },
}

// ── Задача обработки кнопок ──────────────────────────────────────────────

/// Основная задача кнопок и дверей
pub async fn run(
    button_tx: Sender<'static, CriticalSectionRawMutex, ButtonEvent, 4>,
) {
    // На STM32F103: EXTI на каждом пине кнопки
    // При нажатии (falling edge, pull-up) → отправляем событие

    loop {
        // TODO: подключить реальные EXTI interrupt handlers
        // Пока — заглушка
        embassy_time::Timer::after_millis(10).await;
    }
}