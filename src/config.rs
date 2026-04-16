//! Конфигурация пинов и констант — LilyGo T-Call SIM800 v20190610
//!
//! ESP32 Xtensa LX6 dual-core, 520KB SRAM, 4MB Flash
//! SIM800L на плате, IP5306 power management (I2C 0x75)
//!
//! Пин-мап LilyGo T-Call (IP5306 version):
//!   I2C SDA  = GPIO21, SCL = GPIO22 (IP5306 + внешний дисплей/EEPROM)
//!   UART0 (USB) = GPIO1 TX, GPIO3 RX — debug/лог
//!   UART2 (GSM) = GPIO26 TX → SIM800L RXD, GPIO27 RX ← SIM800L TXD
//!   MODEM PWRKEY = GPIO4
//!   MODEM RST   = GPIO5
//!   MODEM POWER = GPIO23 (включение питания модема через ключ)
//!   LED = GPIO13 (v1.4) или нет (v1.3)
//!
//! Внешние устройства (на свободных GPIO):
//!   iButton 1-Wire = GPIO32 (input-only, но можно Flex)
//!   Coin CH1-4 = GPIO12-15 (через NPN транзистор)
//!   Coin CH5 = GPIO16 (если нет PSRAM)
//!   Coin CH6 = GPIO17 (если нет PSRAM)
//!   Coin BLOCK = GPIO33
//!   Hopper A Enable = GPIO18, Sensor = GPIO25
//!   Hopper B Enable = GPIO19, Sensor = GPIO14 (занят под CH3!)
//!   Buttons = GPIO34,35,36,39 (input-only pins)
//!   Door 1 = GPIO36, Door 2 = GPIO39

#![allow(dead_code)]

// ── Плата ──────────────────────────────────────────────────────────────────

/// Версия платы — LilyGo T-Call SIM800 IP5306
pub const BOARD_NAME: &str = "LilyGo T-Call SIM800 IP5306";

// ── I2C (IP5306 + внешний дисплей + EEPROM) ────────────────────────────────

/// I2C SDA — GPIO21 (общая шина с IP5306)
pub const I2C_SDA_PIN: u8 = 21;
/// I2C SCL — GPIO22 (общая шина с IP5306)
pub const I2C_SCL_PIN: u8 = 22;
pub const I2C_FREQ_HZ: u32 = 100_000; // 100 kHz

// ── IP5306 Power Management ────────────────────────────────────────────

/// I2C адрес IP5306
pub const IP5306_I2C_ADDR: u8 = 0x75;

// ── Дисплей HD44780 через PCF8574 ────────────────────────────────────────

pub const LCD_I2C_ADDR: u8 = 0x27; // или 0x3F
pub const LCD_COLS: u8 = 16;
pub const LCD_ROWS: u8 = 2;

// ── EEPROM 24C08 ─────────────────────────────────────────────────────────

pub const EEPROM_I2C_ADDR: u8 = 0x50;
pub const EEPROM_PAGE_SIZE: usize = 8;

// ── GSM SIM800L (на плате LilyGo T-Call) ────────────────────────────────
//
// SIM800L уже установлен на плате. UART2 подключён аппаратно.
// Управление: PWRKEY (GPIO4), RST (GPIO5), POWER enable (GPIO23)

/// UART2 TX → SIM800L RXD — GPIO26
pub const GSM_UART_TX_PIN: u8 = 26;
/// UART2 RX ← SIM800L TXD — GPIO27
pub const GSM_UART_RX_PIN: u8 = 27;
pub const GSM_UART_BAUD: u32 = 115_200;
/// PWRKEY — GPIO4 (на плате LilyGo T-Call)
pub const GSM_PWRKEY_PIN: u8 = 4;
/// RST — GPIO5 (hard reset модема)
pub const GSM_RST_PIN: u8 = 5;
/// POWER enable — GPIO23 (включение питания модема через MOSFET ключ)
pub const GSM_POWER_PIN: u8 = 23;

// ── Монетоприёмник NRI G-13.6000 ──────────────────────────────────────────
// Питание: +12V DC
// Выходы: 6 линий, active low (pin 3-4, 7-10)
// Подключение через NPN транзистор (BC547/2N2222):
//   NRI output → 10kΩ → Base, Emitter → GND,
//   Collector → ESP32 GPIO + pull-up 10kΩ → +3.3V
//
// ВНИМАНИЕ: GPIO12 и GPIO15 — strapping pins!
// GPIO12 (MTDI): при загрузке определяет voltage of internal flash (0=3.3V, 1=1.8V)
//   Если при подаче питания на GPIO12 HIGH — ESP32 может не запуститься!
//   Решение: внешний pull-down 10kΩ на GPIO12.
// GPIO15 (MTDO): при загрузке — silences boot messages (0=quiet, 1=verbose)
//   Менее критично, но тоже需要注意

pub const COIN_CHANNEL_COUNT: usize = 6;
/// Coin CH1 — GPIO12 (strapping pin! Нужен внешний pull-down!)
pub const COIN_CH1_PIN: u8 = 12;
/// Coin CH2 — GPIO13
pub const COIN_CH2_PIN: u8 = 13;
/// Coin CH3 — GPIO14
pub const COIN_CH3_PIN: u8 = 14;
/// Coin CH4 — GPIO15 (strapping pin! boot messages)
pub const COIN_CH4_PIN: u8 = 15;
/// Coin CH5 — GPIO16 (только если нет PSRAM)
pub const COIN_CH5_PIN: u8 = 16;
/// Coin CH6 — GPIO17 (только если нет PSRAM)
pub const COIN_CH6_PIN: u8 = 17;
/// Total blocking pin — GPIO33 (input-only, но можно использовать как output на ESP32)
pub const COIN_BLOCK_PIN: u8 = 33;

// ── Хопперы ──────────────────────────────────────────────────────────────

pub const HOPPER_COUNT: usize = 2;

pub const COIN_PULSE_MIN_MS: u64 = 30;
pub const COIN_PULSE_MAX_MS: u64 = 300;
pub const PAUSE_BETWEEN_ERROR_CODES_MS: u64 = 400;

// ── Hopper пины ──────────────────────────────────────────────────────────
//
// GPIO26/27 заняты под UART2 (SIM800L), поэтому хопперы на других пинах

/// Hopper A Enable — GPIO18
pub const HOPPER_A_ENABLE_PIN: u8 = 18;
/// Hopper A Sensor — GPIO25
pub const HOPPER_A_SENSOR_PIN: u8 = 25;
/// Hopper B Enable — GPIO19
pub const HOPPER_B_ENABLE_PIN: u8 = 19;
/// Hopper B Sensor — GPIO34 (input-only!)
pub const HOPPER_B_SENSOR_PIN: u8 = 34;

// ── Кнопки (input-only pins на ESP32!) ────────────────────────────────────
//
// GPIO32 был бы идеален для кнопок, но на T-Call v1.4 он занят под MODEM DTR
// Используем оставшиеся input-only пины

/// Button PREV — GPIO35 (input-only)
pub const BTN_PREV_PIN: u8 = 35;
/// Button NEXT — GPIO36 (input-only)
pub const BTN_NEXT_PIN: u8 = 36;
/// Button OK — GPIO39 (input-only)
pub const BTN_OK_PIN: u8 = 39;
/// Button CANCEL — нет свободного input-only пина!
/// Используем GPIO2 как обычный input с pull-up (не input-only, но работает)
pub const BTN_CANCEL_PIN: u8 = 2;

// ── Двери (input-only!) ────────────────────────────────────────────────
//
// На T-Call input-only пины сильно ограничены:
// GPIO34,35,36,39 — все input-only
// GPIO34 — уже используется под Hopper B Sensor
// GPIO35 — кнопка PREV
// GPIO36 — кнопка NEXT
// GPIO39 — кнопка OK
// Проблема: двери придётся ставить на не-input-only пины или отказаться от двух дверей

/// Door 1 — GPIO32 (на v1.4 занят под DTR; на v1.3 свободен)
pub const DOOR_1_PIN: u8 = 32;
/// Door 2 — нет свободного input-only пина. Используем обычный GPIO.
/// GPIO0 — boot button (не рекомендуется), ставим None
pub const DOOR_2_PIN: u8 = 255; // Нет второго пина для двери
pub const MAX_DOORS: usize = 1; // Только одна дверь

// ── 1-Wire iButton ──────────────────────────────────────────────────────
//
// GPIO4 занят под SIM800L PWRKEY на T-Call!
// Переносим на GPIO33 (input-only, но Flex/OpenDrain работает)

/// iButton 1-Wire — GPIO33 (input-only на ESP32, используем Flex)
/// Альтернатива: GPIO32 если не нужен Door 1
pub const IBUTTON_PIN: u8 = 33;

// ── LED ──────────────────────────────────────────────────────────────────
//
// На T-Call v1.4 — GPIO13 (User LED)
// На T-Call v1.3 — нет встроенного LED

/// LED — GPIO13 (на T-Call v1.4)
pub const LED_PIN: u8 = 13;

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
/// Flash: 4MB
pub const FLASH_SIZE: usize = 4096 * 1024;