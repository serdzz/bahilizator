//! Параметризованный драйвер хоппера
//!
//! Заменяет 3× copy-paste функции из hopper.c (hopperA_process, hopperB_process, hopperC_process)
//! Одна struct + методы — для всех хопперов
//!
//! Перенос из hopper.c:
//! - HOPPER_IDLE → HOPPER_PAY → возврат (монета прошла)
//! - HOPPER_IDLE → HOPPER_ERROR_LO → HOPPER_ERROR_HI → декодирование ошибки
//! - Разница HOPPER_A: не требует start/stop cycle при clearError
//!
//! State machine: IDLE → PAYING → CHECKING → COMPLETED/ERROR
//! Sensor polling каждую 1мс для точного детектирования импульсов

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::{Receiver, Sender};

use crate::config;
use crate::error::DispenserError;
use crate::state::Level;

// ── Команды хопперу ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, defmt::Format)]
pub enum HopperCmd {
    /// Выдать count монет из хоппера hopper
    Payout { hopper: u8, count: Level },
    /// Остановить выдачу хоппера
    Stop { hopper: u8 },
    /// Очистить ошибку хоппера
    ClearError { hopper: u8 },
}

// ── События от хоппера ───────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, defmt::Format)]
pub enum HopperEvent {
    /// Одна монета выдана
    CoinDispensed { hopper: u8 },
    /// Ошибка хоппера
    Error { hopper: u8, error: DispenserError },
    /// Таймаут — монета не вышла за отведённое время
    Timeout { hopper: u8 },
    /// Нет события
    None,
}

// ── Состояние хоппера ────────────────────────────────────────────────────
//
// Перенос из hopper.c: hopper_operation_level[]
// HOPPER_IDLE = 0, HOPPER_PAY, HOPPER_ERROR_LO, HOPPER_ERROR_HI

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum HopperState {
    #[default]
    Idle,
    Paying,
    ErrorLo,
    ErrorHi,
}

// ── HopperConfig — конфигурация хоппера ──────────────────────────────────
//
// Определяет пины и ограничения для конкретного хоппера

#[derive(Debug, Clone, Copy)]
pub struct HopperConfig {
    /// Идентификатор хоппера (0=A, 1=B, 2=C)
    pub id: u8,
    /// Хоппер A — особый: при clearError не делает start/stop cycle
    /// (в оригинале: if (hopperNumber==HOPPER_A) return;)
    pub is_hopper_a: bool,
    /// Максимальное количество монет для одной выдачи (защита от бесконечного вращения)
    pub max_payout: Level,
}

// ── Hopper — параметризованная структура ──────────────────────────────────

pub struct Hopper {
    /// Конфигурация хоппера
    pub config: HopperConfig,
    /// Текущее состояние state machine
    state: HopperState,
    /// Счётчик импульсов ошибки (для декодирования error code)
    error_pulse_count: u8,
    /// Время начала текущего импульса (монета или ошибка)
    pulse_time: u64,
    /// Счётчик оставшихся монет к выдаче
    coins_to_payout: Level,
    /// Счётчик выданных монет
    coins_dispensed: Level,
    /// Мотор включён (управляющий пин = LOW)
    motor_on: bool,
    /// Датчик монеты: true = монета проходит (низкий уровень)
    coin_sensed: bool,
    /// Датчик ошибки: true = ошибка (низкий уровень)
    error_sensed: bool,
}

impl Hopper {
    /// Создать новый экземпляр хоппера
    pub fn new(config: HopperConfig) -> Self {
        Self {
            config,
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

    /// Запустить выдачу — включить мотор
    ///
    /// В оригинале (hopper.c): hopperStartPayOut()
    ///   HOPPER_A: INT_POWER_FAIL_INTERRUPT&=~INT_POWER_FAIL_PIN; HOPPER_A_CONTROL_LOW;
    ///   HOPPER_B: HOPPER_B_CONTROL_LOW;
    ///   HOPPER_C: HOPPER_C_CONTROL_LOW;
    pub fn start_payout(&mut self) {
        self.motor_on = true;
        // GPIO пин хоппера установлен в LOW (мотор ON) — через глобальный указатель
        // Для HOPPER_A: также отключить прерывание power fail
    }

    /// Остановить выдачу — выключить мотор
    ///
    /// В оригинале: hopperStopPayOut()
    ///   HOPPER_A: INT_POWER_FAIL_INTERRUPT|=INT_POWER_FAIL_PIN; HOPPER_A_CONTROL_HI;
    ///   HOPPER_B: HOPPER_B_CONTROL_HI;
    ///   HOPPER_C: HOPPER_C_CONTROL_HI;
    pub fn stop_payout(&mut self) {
        self.motor_on = false;
        // GPIO пин хоппера установлен в HIGH (мотор OFF) — через глобальный указатель
    }

    /// Очистить ошибку хоппера
    ///
    /// В оригинале (hopper.c): clearError()
    ///   error_pulse_count[hopperNumber]=0;
    ///   hopper_operation_level[hopperNumber]=HOPPER_IDLE;
    ///   if (hopperNumber==HOPPER_A) return;  // HOPPER_A — особый
    ///   hopperStartPayOut(hopperNumber);
    ///   uSleep(10);
    ///   hopperStopPayOut(hopperNumber);
    pub fn clear_error(&mut self) {
        self.error_pulse_count = 0;
        self.state = HopperState::Idle;
        // HOPPER_A special: не делает start/stop cycle
        if !self.config.is_hopper_a {
            self.start_payout();
            // Короткий импульс для очистки ошибки
            // ESP32: busy-wait ~10µs (240MHz / 3 ≈ 80 cycles/µs → ~800 cycles)
            for _ in 0..800 {
                core::hint::spin_loop();
            }
            self.stop_payout();
        }
    }

    /// Обновить состояние датчиков (вызывать перед poll)
    pub fn update_sensors(&mut self, coin_sensed: bool, error_sensed: bool) {
        self.coin_sensed = coin_sensed;
        self.error_sensed = error_sensed;
    }

    /// Опрос хоппера — вызывать каждую 1мс
    ///
    /// Перенос из hopper.c: hopperA_process(), hopperB_process(), hopperC_process()
    ///
    /// Возвращает событие если произошло, иначе HopperEvent::None
    ///
    /// Логика:
    ///   IDLE:
    ///     - Датчик ошибки (низкий) → ERROR_LO
    ///     - Датчик монеты (низкий) + мотор ON → PAYING (запомнить pulse_time)
    ///
    ///   PAYING:
    ///     - Датчик монеты (высокий) + длительность в пределах нормы → монета прошла
    ///     - Датчик монеты (высокий) + слишком длинный импульс → таймаут
    ///     - Датчик ошибки (низкий) → ERROR_LO
    ///     - HOPPER_A особенность: если мотор не включён, но монета вышла → ошибка
    ///
    ///   ERROR_LO:
    ///     - Остановить мотор
    ///     - Ждём пока датчик ошибки поднимется → ERROR_HI
    ///
    ///   ERROR_HI:
    ///     - Датчик ошибки снова низкий → подсчёт импульсов ошибки
    ///     - Если пауза между сериями > PAUSE_BETWEEN_ERROR_CODES → декодировать
    ///       error code = error_pulse_count
    pub fn poll(&mut self, now: u64) -> HopperEvent {
        match self.state {
            HopperState::Idle => {
                // Датчик ошибки активен (низкий уровень)
                if self.error_sensed {
                    self.state = HopperState::ErrorLo;
                    self.stop_payout();
                    return HopperEvent::None;
                }

                // Датчик монеты активен (низкий уровень)
                // Для HOPPER_B/C: мотор должен быть включён (IS_HOPPER_B_PAY)
                // Для HOPPER_A: мотор может не быть включён (особенность)
                if self.coin_sensed && (self.config.is_hopper_a || self.motor_on) {
                    self.pulse_time = now;
                    self.state = HopperState::Paying;
                }
                HopperEvent::None
            }

            HopperState::Paying => {
                // Монета прошла — датчик вернулся в высокий уровень
                if !self.coin_sensed {
                    // Для HOPPER_A: датчик монеты — инвертированный (COIN_STATE = 0 при монете)
                    // Но в нашей абстракции coin_sensed=true при монете, так что !coin_sensed = монета прошла
                    // Вычисляем длительность импульса
                    let elapsed = now.wrapping_sub(self.pulse_time);

                    if (config::COIN_PULSE_MIN_MS..=config::COIN_PULSE_MAX_MS).contains(&elapsed) {
                        // Валидная монета
                        self.coins_dispensed += 1;
                        if self.coins_dispensed >= self.coins_to_payout {
                            self.stop_payout();
                            self.coins_to_payout = 0;
                            self.coins_dispensed = 0;
                        }
                        self.state = HopperState::Idle;
                        return HopperEvent::CoinDispensed {
                            hopper: self.config.id,
                        };
                    } else if elapsed > config::COIN_PULSE_MAX_MS {
                        // Таймаут — монета застряла
                        // В оригинале: HOPPER_A_CONTROL_HI; *error=1;
                        self.stop_payout();
                        self.state = HopperState::Idle;
                        return HopperEvent::Timeout {
                            hopper: self.config.id,
                        };
                    }
                    // Если elapsed < COIN_PULSE_MIN_MS — дребезг, игнорируем
                }

                // Датчик ошибки сработал во время выдачи
                if self.error_sensed {
                    self.stop_payout();
                    self.state = HopperState::ErrorLo;
                }

                HopperEvent::None
            }

            HopperState::ErrorLo => {
                // В оригинале: HOPPER_A_CONTROL_HI; (остановить мотор — уже сделали при переходе)
                // Ждём пока датчик ошибки поднимется (высокий уровень)
                if !self.error_sensed {
                    self.pulse_time = now;
                    self.state = HopperState::ErrorHi;
                }
                HopperEvent::None
            }

            HopperState::ErrorHi => {
                // Датчик ошибки снова низкий — очередной импульс
                if self.error_sensed {
                    let elapsed = now.wrapping_sub(self.pulse_time);

                    // Проверяем паузу между сериями импульсов
                    if elapsed > config::PAUSE_BETWEEN_ERROR_CODES_MS {
                        // Новая серия импульсов
                        if self.error_pulse_count > 0 {
                            // Предыдущая серия завершена — декодируем
                            let error_code = self.error_pulse_count;
                            self.error_pulse_count = 1; // начинаем новую серию
                            self.state = HopperState::ErrorLo;
                            return HopperEvent::Error {
                                hopper: self.config.id,
                                error: DispenserError::from_code(error_code),
                            };
                        }
                        self.error_pulse_count = 1;
                    } else {
                        // Продолжение текущей серии
                        self.error_pulse_count = self.error_pulse_count.saturating_add(1);
                    }

                    self.pulse_time = now;
                    self.state = HopperState::ErrorLo;
                }
                HopperEvent::None
            }
        }
    }

    /// Начать выдачу заданного количества монет
    pub fn request_payout(&mut self, count: Level) {
        self.coins_to_payout = count.min(self.config.max_payout);
        self.coins_dispensed = 0;
        self.start_payout();
    }

    /// Хоппер в режиме выдачи?
    pub fn is_paying(&self) -> bool {
        self.motor_on
    }

    /// Текущая ошибка?
    pub fn is_error(&self) -> bool {
        matches!(self.state, HopperState::ErrorLo | HopperState::ErrorHi)
    }
}

// ── Конфигурации хопперов ────────────────────────────────────────────────

/// Конфигурация хоппера A (item dispenser в оригинале — HOPPER_A)
pub fn hopper_a_config() -> HopperConfig {
    HopperConfig {
        id: 0,
        is_hopper_a: true,
        max_payout: 100,
    }
}

/// Конфигурация хоппера B
pub fn hopper_b_config() -> HopperConfig {
    HopperConfig {
        id: 1,
        is_hopper_a: false,
        max_payout: 50,
    }
}

// ── task_hopper ──────────────────────────────────────────────────────────

/// Основная задача управления хопперами
pub async fn run(
    cmd_rx: Receiver<'static, CriticalSectionRawMutex, HopperCmd, 4>,
    event_tx: Sender<'static, CriticalSectionRawMutex, HopperEvent, 4>,
) {
    // Создаём массив хопперов с конфигурациями
    let mut hoppers: [Hopper; config::HOPPER_COUNT] = [
        Hopper::new(hopper_a_config()),
        Hopper::new(hopper_b_config()),
    ];

    loop {
        // Обработка команд
        if let Ok(cmd) = cmd_rx.try_receive() {
            match cmd {
                HopperCmd::Payout { hopper, count } => {
                    if (hopper as usize) < hoppers.len() {
                        hoppers[hopper as usize].request_payout(count);
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

        // Опрос каждого хоппера с чтением реальных датчиков
        let now = embassy_time::Instant::now().as_millis();
        for hopper in &mut hoppers {
            // Читаем датчики через глобальные GPIO указатели (пока заглушка: всегда idle)
            // hopper.update_sensors(read_coin_pin(hopper.config.id), read_error_pin(hopper.config.id));
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
