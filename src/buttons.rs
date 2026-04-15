//! Кнопки и датчики дверей
//!
//! EXTI ISR → Channel для событий кнопок и дверей
//! Перенос из bah_io.c:
//!   - Кнопки: PREV, NEXT, OK, CANCEL (active-low, pull-up)
//!   - Двери: DOOR1, DOOR2 (active-low, normally-open)
//!   - Free item switch (OPTION_FREE_ITEMS_MODE)
//!
//! В оригинале (MSP430 IAR): GetButtonPressed() через getSwitchState()
//! В Rust/Embassy: EXTI interrupt → Channel → задача обрабатывает

use embassy_sync::channel::Sender;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;

use crate::config;

// ── Кнопки ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, defmt::Format)]
pub enum Button {
    Prev,   // Кнопка "<" — прокрутка назад
    Next,   // Кнопка ">" — прокрутка вперёд
    Ok,     // Кнопка подтверждения
    Cancel, // Кнопка отмена
}

// ── События кнопок и дверей ───────────────────────────────────────────────

#[derive(Debug, Clone, Copy, defmt::Format)]
pub enum ButtonEvent {
    /// Кнопка нажата (falling edge на EXTI)
    Pressed(Button),
    /// Состояние двери изменилось
    DoorChanged { door: u8, opened: bool },
    /// Переключатель бесплатного товара
    FreeItemSwitch { active: bool },
}

// ── Debounce ─────────────────────────────────────────────────────────────

/// Минимальный интервал между повторными срабатываниями кнопки (мс)
const DEBOUNCE_MS: u64 = 50;

// ── EXTI ISR → статические переменные ────────────────────────────────────
//
// EXTI ISR записывает сырые события в static, задача читает их.
// Это безопаснее, чем попытка отправить в Channel из ISR.

use core::sync::atomic::{AtomicU8, Ordering};

/// Сырые события от EXTI (битовая маска)
/// Бит 0: PREV, 1: NEXT, 2: OK, 3: CANCEL
/// Бит 4: DOOR1, 5: DOOR2
/// Бит 6: FREE_ITEM
static PENDING_BUTTONS: AtomicU8 = AtomicU8::new(0);
/// Бит = 1 для дверей: открыта (0 → 1 переход)
static PENDING_DOORS_OPEN: AtomicU8 = AtomicU8::new(0);
/// Бит = 1 для дверей: закрыта (1 → 0 переход)
static PENDING_DOORS_CLOSE: AtomicU8 = AtomicU8::new(0);

// ── ISR callbacks — вызываются из EXTI interrupt handler ──────────────────
//
/// Вызывается из EXTI ISR при нажатии кнопки
/// button_mask: бит 0=PREV, 1=NEXT, 2=OK, 3=CANCEL
pub fn exti_button_pressed(button_mask: u8) {
    PENDING_BUTTONS.fetch_or(button_mask, Ordering::Relaxed);
}

/// Вызывается из EXTI ISR при изменении состояния двери
/// door: 0 или 1
/// opened: true = дверь открылась, false = дверь закрылась
pub fn exti_door_changed(door: u8, opened: bool) {
    if opened {
        PENDING_DOORS_OPEN.fetch_or(1 << door, Ordering::Relaxed);
    } else {
        PENDING_DOORS_CLOSE.fetch_or(1 << door, Ordering::Relaxed);
    }
}

/// Вызывается из EXTI ISR при изменении переключателя бесплатного товара
pub fn exti_free_item_switch(_active: bool) {
    // TODO: реализовать при необходимости
}

// ── Задача обработки кнопок и дверей ──────────────────────────────────────

/// Основная задача кнопок и дверей
///
/// Читает события из EXTI ISR (через атомарные переменные),
/// устраняет дребезг, отправляет в Channel
pub async fn run(
    button_tx: Sender<'static, CriticalSectionRawMutex, ButtonEvent, 4>,
) {
    let mut last_button_time: [u64; 4] = [0; 4]; // Время последнего нажатия каждой кнопки

    loop {
        let now = embassy_time::Instant::now().as_millis();

        // ── Обработка кнопок ────────────────────────────────────────────
        let pending = PENDING_BUTTONS.swap(0, Ordering::Relaxed);
        if pending != 0 {
            for (bit, button) in [
                (0u8, Button::Prev),
                (1u8, Button::Next),
                (2u8, Button::Ok),
                (3u8, Button::Cancel),
            ] {
                if pending & (1 << bit) != 0 {
                    // Debounce: игнорируем если меньше DEBOUNCE_MS с прошлого нажатия
                    if now - last_button_time[bit as usize] >= DEBOUNCE_MS {
                        last_button_time[bit as usize] = now;
                        let _ = button_tx.try_send(ButtonEvent::Pressed(button));
                    }
                }
            }
        }

        // ── Обработка дверей ────────────────────────────────────────────
        let doors_open = PENDING_DOORS_OPEN.swap(0, Ordering::Relaxed);
        let doors_close = PENDING_DOORS_CLOSE.swap(0, Ordering::Relaxed);

        if doors_open != 0 {
            for door in 0..config::MAX_DOORS {
                if doors_open & (1 << door) != 0 {
                    let _ = button_tx.try_send(ButtonEvent::DoorChanged {
                        door: door as u8,
                        opened: true,
                    });
                }
            }
        }

        if doors_close != 0 {
            for door in 0..config::MAX_DOORS {
                if doors_close & (1 << door) != 0 {
                    let _ = button_tx.try_send(ButtonEvent::DoorChanged {
                        door: door as u8,
                        opened: false,
                    });
                }
            }
        }

        embassy_time::Timer::after_millis(10).await;
    }
}