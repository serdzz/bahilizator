//! Драйвер монетоприёмника
//!
//! Два режима работы:
//! - Normal: 6-канальный, каждый канал — свой пин, детекция по уровню
//! - Pulse: один пин, считает импульсы (1 импульс = 1 монета номинала)
//!
//! Перенос из coin_acceptor.c (MSP430):
//! - processNormalMode(): IDLE → PRE_ACCEPT → ACCEPT → возврат канала
//! - processPulseMode():   IDLE → ACCEPT → POST_ACCEPT → возврат pulse_count
//!
//! EXTI ISR отправляет сырые события в Channel, задача обрабатывает их

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Sender;
use embassy_sync::mutex::Mutex;

use crate::config;
use crate::state::{Cash, VendingState};

// ── CoinEvent ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, defmt::Format)]
pub struct CoinEvent {
    /// Канал монетоприёмника (0-5 для normal, 0 для pulse)
    pub channel: u8,
    /// Номинал монеты (из настроек)
    pub value: Cash,
}

// ── Режимы работы ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
enum CoinMode {
    /// Нормальный режим — 6 каналов, каждый на своём пине
    Normal,
    /// Пульсный режим — один пин, количество импульсов = номинал
    Pulse,
}

// ── Состояния обработки — Normal mode ────────────────────────────────────
//
// Перенос из coin_acceptor.c: processNormalMode()
// IDLE → PRE_ACCEPT → ACCEPT → возврат канала
// Логика: детектируем изменение уровня на каналах,
// ждём стабилизации (COIN_ACCEPTOR_PULSE_MIDLE), подтверждаем

#[derive(Debug, Clone, Copy, PartialEq)]
enum NormalStage {
    Idle,
    PreAccept,
    Accept,
}

// ── Состояния обработки — Pulse mode ─────────────────────────────────────
//
// Перенос из coin_acceptor.c: processPulseMode()
// IDLE → ACCEPT → POST_ACCEPT → возврат pulse_count
// Логика: детектируем импульс (низкий уровень), считаем количество,
// после паузы между импульсами > INTER_PULSE_MAX — выдаём результат

#[derive(Debug, Clone, Copy, PartialEq)]
enum PulseStage {
    Idle,
    Accept,
    PostAccept,
}

// ── Таймауты монетоприёмника ─────────────────────────────────────────────
// Из оригинального config.h MSP430

/// Минимальная длительность импульса монеты (normal mode: ~15мс)
const PULSE_MIDLE_MS: u64 = 15;
/// Максимальная длительность импульса монеты (normal mode: ~100мс)
const PULSE_MAX_MS: u64 = 100;
/// Минимальная длительность импульса (pulse mode: ~30мс)
const PULSE_MIN_MS: u64 = 30;
/// Максимальная пауза между импульсами (pulse mode: ~200мс)
const INTER_PULSE_MAX_MS: u64 = 200;

// ── Задача монетоприёмника ────────────────────────────────────────────────

pub async fn run(
    coin_tx: Sender<'static, CriticalSectionRawMutex, CoinEvent, 4>,
    state: &'static Mutex<CriticalSectionRawMutex, core::cell::RefCell<VendingState>>,
) {
    // Определяем режим из настроек
    let mode = {
        let guard = state.lock().await;
        if guard.borrow().settings.coin_acceptor.pulse_mode {
            CoinMode::Pulse
        } else {
            CoinMode::Normal
        }
    };

    match mode {
        CoinMode::Normal => run_normal_mode(coin_tx, state).await,
        CoinMode::Pulse => run_pulse_mode(coin_tx, state).await,
    }
}

// ── Normal Mode ──────────────────────────────────────────────────────────
//
// Перенос из coin_acceptor.c: processNormalMode()
//
// Логика:
//   IDLE: ждём, пока какой-нибудь канал не станет низким
//   PRE_ACCEPT: ждём стабилизации (PULSE_MIDLE_MS), запоминаем канал
//   ACCEPT: ждём, пока канал вернётся в высокий уровень — монета принята
//   Если таймаут — ошибка malfunction

async fn run_normal_mode(
    coin_tx: Sender<'static, CriticalSectionRawMutex, CoinEvent, 4>,
    state: &'static Mutex<CriticalSectionRawMutex, core::cell::RefCell<VendingState>>,
) {
    let mut stage = NormalStage::Idle;
    let mut active_channels: u8 = 0; // Битовая маска активных каналов
    let mut stage_start: u64 = 0;
    let mut error_flag = false;

    loop {
        // Читаем состояние каналов монетоприёмника (GPIO пины)
        // В оригинале: COIN_ACCEPTOR_CH1_STATE .. CH6_STATE
        // TODO: заменить на реальные GPIO
        let channel_state: u8 = read_coin_channels();
        let channel_mask = get_channel_mask(state).await;
        let now = embassy_time::Instant::now().as_millis();

        match stage {
            NormalStage::Idle => {
                // Какой-нибудь канал стал низким (монета проходит)
                if (channel_state & channel_mask) != channel_mask {
                    stage_start = now;
                    stage = NormalStage::PreAccept;
                    // Запоминаем, какие каналы активны (низкий уровень)
                    active_channels = channel_state & channel_mask;
                }
            }
            NormalStage::PreAccept => {
                // Ждём стабилизации импульса (PULSE_MIDLE_MS)
                if now - stage_start >= PULSE_MIDLE_MS {
                    // Фиксируем каналы, которые изменились
                    active_channels = channel_state & channel_mask;
                    stage_start = now;
                    stage = NormalStage::Accept;
                }
            }
            NormalStage::Accept => {
                // Канал вернулся в высокий уровень — монета прошла
                if (channel_state & channel_mask) == channel_mask {
                    // Определяем, какой канал сработал
                    let detected = channel_mask ^ active_channels;
                    if detected != 0 {
                        // Находим номер канала (первый установленный бит)
                        let ch = detected.trailing_zeros() as u8;
                        if ch < config::COIN_CHANNEL_COUNT as u8 {
                            let value = get_coin_value(state, ch as usize).await;
                            if value > 0 {
                                let _ = coin_tx.try_send(CoinEvent { channel: ch, value });
                            }
                        }
                    }
                    stage = NormalStage::Idle;
                    active_channels = 0;
                } else if now - stage_start > PULSE_MAX_MS {
                    // Таймаут — malfunction (как в оригинале)
                    error_flag = true;
                    stage = NormalStage::Idle;
                    active_channels = 0;
                    // TODO: установить ошибку ERROR_COIN_ACCEPTOR в state
                }
            }
        }

        if error_flag {
            // В оригинале: InsertElement(&aBah->errors, ERROR_COIN_ACCEPTOR_DEVICE)
            error_flag = false;
        }

        embassy_time::Timer::after_millis(1).await;
    }
}

// ── Pulse Mode ────────────────────────────────────────────────────────────
//
// Перенос из coin_acceptor.c: processPulseMode()
//
// Логика:
//   IDLE: ждём низкий уровень на пине монетоприёмника
//   ACCEPT: считаем импульсы (каждый низкий→высокий переход = 1 импульс)
//   POST_ACCEPT: после паузы > INTER_PULSE_MAX — выдаём результат
//   Количество импульсов = номер канала: 1 импульс → ch0, 2 импульса → ch1 и т.д.
//   (в оригинале: ret = 1<<(pulse_count-1))

async fn run_pulse_mode(
    coin_tx: Sender<'static, CriticalSectionRawMutex, CoinEvent, 4>,
    state: &'static Mutex<CriticalSectionRawMutex, core::cell::RefCell<VendingState>>,
) {
    let mut stage = PulseStage::Idle;
    let mut pulse_count: u8 = 0;
    let mut stage_start: u64 = 0;

    loop {
        // Читаем состояние пина монетоприёмника
        // В оригинале: COIN_ACCEPTOR_ANY_CH_LO_STATE != COIN_ACCEPTOR_CH_LO_MASK
        // Низкий уровень = монета проходит
        let coin_active = read_coin_pulse_pin(); // true = низкий уровень
        let now = embassy_time::Instant::now().as_millis();

        match stage {
            PulseStage::Idle => {
                // Детектируем начало импульса
                if coin_active {
                    stage_start = now;
                    stage = PulseStage::Accept;
                }
            }
            PulseStage::Accept => {
                // Импульс завершился (уровень стал высоким)
                if !coin_active {
                    let elapsed = now - stage_start;
                    if (PULSE_MIN_MS..PULSE_MAX_MS).contains(&elapsed) {
                        // Валидный импульс — считаем
                        pulse_count += 1;
                        stage_start = now;
                        stage = PulseStage::PostAccept;
                    } else {
                        // Невалидный импульс — malfunction
                        pulse_count = 0;
                        stage = PulseStage::Idle;
                        // TODO: ошибка ERROR_COIN_ACCEPTOR
                    }
                } else if now - stage_start > PULSE_MAX_MS {
                    // Слишком длинный импульс — malfunction
                    pulse_count = 0;
                    stage = PulseStage::Idle;
                }
            }
            PulseStage::PostAccept => {
                if coin_active {
                    // Новый импульс начался до таймаута — продолжаем считать
                    let elapsed = now - stage_start;
                    if elapsed < PULSE_MAX_MS {
                        stage = PulseStage::Accept;
                        stage_start = now;
                    } else {
                        // Слишком длинная пауза между импульсами
                        pulse_count = 0;
                        stage = PulseStage::Idle;
                    }
                } else if now - stage_start >= INTER_PULSE_MAX_MS {
                    // Серия импульсов завершилась
                    // В оригинале: ret = 1<<(pulse_count-1)
                    // pulse_count=1 → ch0, pulse_count=2 → ch1, ...
                    if pulse_count > 0 && pulse_count <= config::COIN_CHANNEL_COUNT as u8 {
                        let ch = (pulse_count - 1) as usize;
                        let value = get_coin_value(state, ch).await;
                        if value > 0 {
                            let _ = coin_tx.try_send(CoinEvent {
                                channel: ch as u8,
                                value,
                            });
                        }
                    }
                    pulse_count = 0;
                    stage = PulseStage::Idle;
                }
            }
        }

        embassy_time::Timer::after_millis(1).await;
    }
}

// ── Вспомогательные функции ───────────────────────────────────────────────

/// Прочитать значение монеты для канала из настроек
async fn get_coin_value(
    state: &'static Mutex<CriticalSectionRawMutex, core::cell::RefCell<VendingState>>,
    channel: usize,
) -> Cash {
    let guard = state.lock().await;
    let st = guard.borrow();
    if channel < config::COIN_CHANNEL_COUNT {
        st.settings.coin_acceptor.coin_values[channel]
    } else {
        0
    }
}

/// Прочитать маску разрешённых каналов из настроек
async fn get_channel_mask(
    state: &'static Mutex<CriticalSectionRawMutex, core::cell::RefCell<VendingState>>,
) -> u8 {
    let guard = state.lock().await;
    let st = guard.borrow();
    // Формируем маску из coin_enable
    let mut mask = 0u8;
    for i in 0..config::COIN_CHANNEL_COUNT {
        if st.settings.coin_acceptor.coin_enable[i] {
            mask |= 1 << i;
        }
    }
    mask
}

/// Прочитать состояние 6 каналов монетоприёмника (normal mode)
///
/// Возвращает битовую маску: 1 = высокий уровень (нет монеты),
/// 0 = низкий уровень (монета проходит)
///
/// В оригинале (MSP430): COIN_ACCEPTOR_CH1_STATE .. CH6_STATE
/// Транзистор инвертирует: NRI active low → NPN → HIGH на GPIO STM32
/// Поэтому HIGH на GPIO = монета обнаружена на линии
///
/// TODO: заменить на реальные GPIO через esp-hal
fn read_coin_channels() -> u8 {
    // Заглушка — нет активных линий
    0x00
}

/// Прочитать состояние пина монетоприёмника (pulse mode)
///
/// NRI G-13 output active low → NPN транзистор (BC547) → HIGH на GPIO
/// Поэтому HIGH на GPIO = импульс от монеты
///
/// TODO: заменить на реальное GPIO через esp-hal
fn read_coin_pulse_pin() -> bool {
    false // заглушка — нет монеты
}
