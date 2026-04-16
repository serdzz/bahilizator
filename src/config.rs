//! Конфигурация пинов и констант — перенос из config.h
//!
//! Пин-мап для STM32F103C8T6 Bluepill
//! Пины указаны как строки для документации; реальное использование — через embassy-stm32 PAC

#![allow(dead_code)]

// ── I2C (дисплей + EEPROM) ────────────────────────────────────────────────

/// I2C1 SDA — PB7
pub const I2C_SDA_PIN: &str = "PB7";
/// I2C1 SCL — PB6
pub const I2C_SCL_PIN: &str = "PB6";
pub const I2C_FREQ_HZ: u32 = 100_000; // 100 kHz

// ── Дисплей HD44780 через PCF8574 ────────────────────────────────────────

pub const LCD_I2C_ADDR: u8 = 0x27; // или 0x3F
pub const LCD_COLS: u8 = 16;
pub const LCD_ROWS: u8 = 2;

// ── EEPROM 24C08 ─────────────────────────────────────────────────────────

pub const EEPROM_I2C_ADDR: u8 = 0x50;
pub const EEPROM_PAGE_SIZE: usize = 8;

// ── GSM SIM800L (USART1) ────────────────────────────────────────────────

/// USART1 TX — PA9
pub const GSM_UART_TX_PIN: &str = "PA9";
/// USART1 RX — PA10
pub const GSM_UART_RX_PIN: &str = "PA10";
pub const GSM_UART_BAUD: u32 = 115_200;
/// PWRKEY — PA0
pub const GSM_PWRKEY_PIN: &str = "PA0";
/// STATUS — PA1
pub const GSM_STATUS_PIN: &str = "PA1";
/// DTR — PA2
pub const GSM_DTR_PIN: &str = "PA2";

// ── Монетоприёмник NRI G-13.6000 ──────────────────────────────────────────
// Питание: +12V DC (pin 1=GND, pin 2=+12V)
// Выходы: 6 линий, active low (pin 3-4, 7-10)
// Подключение через NPN транзистор (BC547/2N2222):
//   NRI output → 10kΩ → Base, Emitter → GND,
//   Collector → STM32 GPIO + pull-up 10kΩ → +3.3V
// Транзистор инвертирует: NRI low → GPIO HIGH (монета обнаружена)

pub const COIN_CHANNEL_COUNT: usize = 6;
/// Coin channel GPIO pins (PB8-PB13, через NPN транзистор)
pub const COIN_CH1_PIN: &str = "PB8";
pub const COIN_CH2_PIN: &str = "PB9";
pub const COIN_CH3_PIN: &str = "PB10";
pub const COIN_CH4_PIN: &str = "PB11";
pub const COIN_CH5_PIN: &str = "PB12";
pub const COIN_CH6_PIN: &str = "PB13";
/// Total blocking pin (NRI pin 6, active HIGH — без инверсии)
pub const COIN_BLOCK_PIN: &str = "PB14";

// ── Хопперы ──────────────────────────────────────────────────────────────

pub const HOPPER_COUNT: usize = 2;

pub const COIN_PULSE_MIN_MS: u64 = 30;
pub const COIN_PULSE_MAX_MS: u64 = 300;
pub const PAUSE_BETWEEN_ERROR_CODES_MS: u64 = 400;

// ── Кнопки (PA3-PA6) ────────────────────────────────────────────────────

pub const BTN_PREV_PIN: &str = "PA3";
pub const BTN_NEXT_PIN: &str = "PA4";
pub const BTN_OK_PIN: &str = "PA5";
pub const BTN_CANCEL_PIN: &str = "PA6";

// ── Двери (PA7, PA8) ────────────────────────────────────────────────────

pub const DOOR_1_PIN: &str = "PA7";
pub const DOOR_2_PIN: &str = "PA8";
pub const MAX_DOORS: usize = 2;

// ── 1-Wire iButton (PA11) ──────────────────────────────────────────────

pub const IBUTTON_PIN: &str = "PA11";

// ── LED ──────────────────────────────────────────────────────────────────

pub const LED_RED_PIN: &str = "PC15";
pub const LED_GREEN_PIN: &str = "PB0";

// ── Power Fail (PB1) ────────────────────────────────────────────────────

pub const POWER_FAIL_PIN: &str = "PB1";

// ── Таймауты (в секундах) ────────────────────────────────────────────────

pub const RESIDUAL_TIMEOUT_S: u8 = 15;
pub const MENU_EXIT_TIMEOUT_S: u8 = 30;
pub const CASH_CLEAR_TIMEOUT_S: u8 = 10;
pub const THANKS_MESSAGE_DELAY_S: u64 = 3;
pub const PAYOUT_MESSAGE_DELAY_S: u64 = 2;
pub const STATE_PERSIST_DEBOUNCE_S: u64 = 5;

// ── Версия прошивки ──────────────────────────────────────────────────────

pub const FIRMWARE_VERSION: u8 = 2;
pub const SETTINGS_VERSION: u8 = 1;
pub const STATE_VERSION: u8 = 1;