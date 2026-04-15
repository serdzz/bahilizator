//! Параметризованный драйвер хоппера
//!
//! Заменяет 3× copy-paste функции из hopper.c
//! Одна struct + методы — для всех хопперов

use embassy_sync::channel::{Channel, Receiver, Sender};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;

use crate::config;
use crate::error::DispenserError;
use crate::state::{Level, Ticks};

// ── Команды хопперу ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, defmt::Format)]
pub enum HopperCmd {
    Payout { hopper: u8, count: Level },
    Stop { hopper: u8 },
    ClearError { hopper: u8 },
}

// ── События от хоппера ───────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, defmt::Format)]
pub enum HopperEvent {
    CoinDispensed { hopper: u8 },
    Error { hopper: u8, error: DispenserError },
    Timeout { hopper: u8 },
    None,
}

// ── Состояние хоппера ────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
enum HopperState {
    Idle,
    Paying,
    ErrorLo,
    ErrorHi,
}

impl Default for HopperState {
    fn default() -> Self {
        HopperState::Idle
    }
}

// ── Hopper — параметризованная структура ──────────────────────────────────

pub struct Hopper {
    /// Идентификатор хоппера (0 = A, 1 = B, 2 = C)
    pub id: u8,
    /// Текущее состояние
    state: HopperState,
    /// Счётчик импульсов ошибки
    error_pulse_count: u8,
    /// Время начала текущего импульса
    pulse_time: Ticks,
    /// Счётчик оставшихся монет к выдаче
    coins_to_payout: Level,
    /// Счётчик выданных монет
    coins_dispensed: Level,
    /// Мотор включён
    motor_on: bool,
    /// Датчик монеты активен (low = монета)
    coin_sensed: bool,
    /// Датчик ошибки активен (low = ошибка)
    error_sensed: bool,
}

impl Hopper {
    /// Создать новый экземпляр хоппера
    pub fn new(id: u8) -> Self {
        Self {
            id,
            state: HopperState::Idle,
            error_pulse_count: 0,
            pulse_time: 0,
            coins_to_payout: 0,
            coins_dispensed: 0,
            motor_on: false,
            coin_sensed: false,
            error_sensed: false,
        }
    }

    /// Запустить выдачу
    pub fn start_payout(&mut self) {
        self.motor_on = true;
        // TODO: установить GPIO пин хоппера в HIGH
    }

    /// Остановить выдачу
    pub fn stop_payout(&mut self) {
        self.motor_on = false;
        // TODO: установить GPIO пин хоппера в LOW
    }

    /// Очистить ошибку
    pub fn clear_error(&mut self) {
        self.error_pulse_count = 0;
        self.state = HopperState::Idle;
        // HOPPER_A special: no stop/start cycle
        if self.id != 0 {
            self.start_payout();
            // Короткий импульс
            cortex_m::asm::delay(720); // ~10µs at 72MHz
            self.stop_payout();
        }
    }

    /// Обновить состояние датчиков (вызывать перед poll)
    pub fn update_sensors(&mut self, coin_sensed: bool, error_sensed: bool) {
        self.coin_sensed = coin_sensed;
        self.error_sensed = error_sensed;
    }

    /// Опрос хоппера — вызывать каждую 1мс
    /// Возвращает событие если произошло
    pub fn poll(&mut self, now: Ticks) -> HopperEvent {
        match self.state {
            HopperState::Idle => {
                if self.error_sensed {
                    self.state = HopperState::ErrorLo;
                    self.stop_payout();
                } else if self.coin_sensed && self.motor_on {
                    self.pulse_time = now;
                    self.state = HopperState::Paying;
                }
                HopperEvent::None
            }
            HopperState::Paying => {
                if !self.coin_sensed {
                    // Импульс монеты завершился
                    let elapsed = now.wrapping_sub(self.pulse_time);
                    if elapsed >= config::COIN_PULSE_MIN_MS
                        && elapsed <= config::COIN_PULSE_MAX_MS
                    {
                        self.coins_dispensed += 1;
                        if self.coins_dispensed >= self.coins_to_payout {
                            self.stop_payout();
                            self.coins_to_payout = 0;
                            self.coins_dispensed = 0;
                        }
                        self.state = HopperState::Idle;
                        return HopperEvent::CoinDispensed { hopper: self.id };
                    } else if elapsed > config::COIN_PULSE_MAX_MS {
                        self.stop_payout();
                        self.state = HopperState::Idle;
                        return HopperEvent::Timeout { hopper: self.id };
                    }
                } else if self.error_sensed {
                    self.stop_payout();
                    self.state = HopperState::ErrorLo;
                }
                HopperEvent::None
            }
            HopperState::ErrorLo => {
                self.stop_payout();
                if !self.error_sensed {
                    self.pulse_time = now;
                    self.state = HopperState::ErrorHi;
                }
                HopperEvent::None
            }
            HopperState::ErrorHi => {
                if self.error_sensed {
                    let elapsed = now.wrapping_sub(self.pulse_time);
                    if elapsed > config::PAUSE_BETWEEN_ERROR_CODES_MS {
                        // Новая серия импульсов ошибки
                        if self.error_pulse_count > 0 {
                            let error_code = self.error_pulse_count;
                            self.error_pulse_count = 0;
                            self.state = HopperState::Idle;
                            return HopperEvent::Error {
                                hopper: self.id,
                                error: DispenserError::from_code(error_code),
                            };
                        }
                        self.error_pulse_count = 1;
                    } else {
                        self.error_pulse_count += 1;
                    }
                    self.pulse_time = now;
                    self.state = HopperState::ErrorLo;
                }
                HopperEvent::None
            }
        }
    }
}

// ── task_hopper ──────────────────────────────────────────────────────────

/// Основная задача управления хопперами
pub async fn run(
    cmd_rx: Receiver<'static, CriticalSectionRawMutex, HopperCmd, 4>,
    event_tx: Sender<'static, CriticalSectionRawMutex, HopperEvent, 4>,
) {
    // Создаём массив хопперов
    let mut hoppers: [Hopper; config::HOPPER_COUNT] = [
        Hopper::new(0),
        Hopper::new(1),
    ];

    loop {
        // Обработка команд
        if let Ok(cmd) = cmd_rx.try_receive() {
            match cmd {
                HopperCmd::Payout { hopper, count } => {
                    if (hopper as usize) < hoppers.len() {
                        hoppers[hopper as usize].coins_to_payout = count;
                        hoppers[hopper as usize].coins_dispensed = 0;
                        hoppers[hopper as usize].start_payout();
                    }
                }
                HopperCmd::Stop { hopper } => {
                    if (hopper as usize) < hoppers.len() {
                        hoppers[hopper as usize].stop_payout();
                    }
                }
                HopperCmd::ClearError { hopper } => {
                    if (hopper as usize) < hoppers.len() {
                        hoppers[hopper as usize].clear_error();
                    }
                }
            }
        }

        // Опрос каждого хоппера
        let now = embassy_time::Instant::now().as_millis();
        for hopper in &mut hoppers {
            // TODO: прочитать реальные GPIO пины
            hopper.update_sensors(false, false);

            match hopper.poll(now) {
                HopperEvent::None => {}
                event => {
                    let _ = event_tx.try_send(event);
                }
            }
        }

        embassy_time::Timer::after_millis(1).await; // 1 kHz poll rate
    }
}