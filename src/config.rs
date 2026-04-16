//! Конфигурация пинов и констант — ESP32 DevKit V1 (DOIT)
//!
//! ESP32 Xtensa LX6 dual-core, 520KB SRAM, 4MB Flash
//! 30-pin DevKit: GPIO 0-39 (не все доступны)
//!
//! Пин-мап:
//!   I2C SDA  = GPIO21, SCL = GPIO22
//!   UART0 (USB) = GPIO1 TX, GPIO3 RX — debug/лог
//!   UART2 (GSM) = GPIO17 TX, GPIO16 RX — SIM800L
//!   Coin CH1-6 = GPIO13-18 (через NPN)
//!   Coin BLOCK  = GPIO19
//!   Hopper A Enable = GPIO25, Sensor = GPIO26
//!   Hopper B Enable = GPIO27, Sensor = GPIO14
//!   Buttons = GPIO32-35 (input-only pins!)
//!   Doors = GPIO36, GPIO39 (input-only!)
//!   iButton 1-Wire = GPIO4
//!   GSM PWRKEY = GPIO5, STATUS = GPIO33 (input-only)
//!   LED = GPIO2 (встроенный синий на DevKit)

#![allow(dead_code)]

// ── I2C (дисплей + EEPROM) ────────────────────────────────────────────────

/// I2C SDA — GPIO21
pub const I2C_SDA_PIN: u8 = 21;
/// I2C SCL — GPIO22
pub const I2C_SCL_PIN: u8 = 22;
pub const I2C_FREQ_HZ: u32 = 100_000; // 100 kHz

// ── Дисплей HD44780 через PCF8574 ────────────────────────────────────────

pub const LCD_I2C_ADDR: u8 = 0x27; // или 0x3F
pub const LCD_COLS: u8 = 16;
pub const LCD_ROWS: u8 = 2;

// ── EEPROM 24C08 ─────────────────────────────────────────────────────────

pub const EEPROM_I2C_ADDR: u8 = 0x50;
pub const EEPROM_PAGE_SIZE: usize = 8;

// ── GSM SIM800L (UART2) ────────────────────────────────────────────────

/// UART2 TX — GPIO17
pub const GSM_UART_TX_PIN: u8 = 17;
/// UART2 RX — GPIO16
pub const GSM_UART_RX_PIN: u8 = 16;
pub const GSM_UART_BAUD: u32 = 115_200;
/// PWRKEY — GPIO5
pub const GSM_PWRKEY_PIN: u8 = 5;
/// STATUS — GPIO33 (input-only!)
pub const GSM_STATUS_PIN: u8 = 33;
/// DTR — GPIO23
pub const GSM_DTR_PIN: u8 = 23;

// ── Монетоприёмник NRI G-13.6000 ──────────────────────────────────────────
// Питание: +12V DC
// Выходы: 6 линий, active low (pin 3-4, 7-10)
// Подключение через NPN транзистор (BC547/2N2222):
//   NRI output → 10kΩ → Base, Emitter → GND,
//   Collector → ESP32 GPIO + pull-up 10kΩ → +3.3V

pub const COIN_CHANNEL_COUNT: usize = 6;
/// Coin channel GPIO pins (GPIO13-18, через NPN транзистор)
pub const COIN_CH1_PIN: u8 = 13;
pub const COIN_CH2_PIN: u8 = 14;
pub const COIN_CH3_PIN: u8 = 15;
pub const COIN_CH4_PIN: u8 = 16;
pub const COIN_CH5_PIN: u8 = 17;
pub const COIN_CH6_PIN: u8 = 18;
/// Total blocking pin (GPIO19, active HIGH)
pub const COIN_BLOCK_PIN: u8 = 19;

// ── Хопперы ──────────────────────────────────────────────────────────────

pub const HOPPER_COUNT: usize = 2;

pub const COIN_PULSE_MIN_MS: u64 = 30;
pub const COIN_PULSE_MAX_MS: u64 = 300;
pub const PAUSE_BETWEEN_ERROR_CODES_MS: u64 = 400;

// ── Кнопки (GPIO32-35 — input-only pins на ESP32!) ───────────────────────

pub const BTN_PREV_PIN: u8 = 32;
pub const BTN_NEXT_PIN: u8 = 33;
pub const BTN_OK_PIN: u8 = 34;
pub const BTN_CANCEL_PIN: u8 = 35;

// ── Двери (GPIO36, GPIO39 — input-only!) ────────────────────────────────

pub const DOOR_1_PIN: u8 = 36;
pub const DOOR_2_PIN: u8 = 39;
pub const MAX_DOORS: usize = 2;

// ── 1-Wire iButton (GPIO4) ──────────────────────────────────────────────

pub const IBUTTON_PIN: u8 = 4;

// ── LED (встроенный синий на DevKit) ──────────────────────────────────────

pub const LED_PIN: u8 = 2;

// ── Hopper пины ──────────────────────────────────────────────────────────

/// Hopper A Enable — GPIO25
pub const HOPPER_A_ENABLE_PIN: u8 = 25;
/// Hopper A Sensor — GPIO26
pub const HOPPER_A_SENSOR_PIN: u8 = 26;
/// Hopper B Enable — GPIO27
pub const HOPPER_B_ENABLE_PIN: u8 = 27;
/// Hopper B Sensor — GPIO14
pub const HOPPER_B_SENSOR_PIN: u8 = 14;

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

// ── ESP32 характеристики ─────────────────────────────────────────────────

/// SRAM: 520KB (в 26 раз больше чем STM32F103!)
pub const SRAM_SIZE: usize = 520 * 1024;
/// Flash: 4MB (в 64 раза больше!)
pub const FLASH_SIZE: usize = 4096 * 1024;
