//! Дисплей HD44780 через I2C PCF8574 backpack
//!
//! Полный драйвер с инициализацией, курсором, custom chars
//! PCF8574 pin mapping: P0=RS, P1=RW, P2=E, P3=Backlight, P4=D4, P5=D5, P6=D6, P7=D7
//!
//! Инициализация 4-bit mode:
//!   0x33 → 0x32 → 0x28 → 0x0C → 0x06 → 0x01
//! Перенос из lcd.c (оригинальный MSP430 код) на Embassy async I2C
//!
//! Поддерживает два режима работы:
//!   1. С реальным I2C (display_task_with_i2c) — запись через esp-hal I2C
//!   2. Без I2C (display_task) — заглушка, драйвер работает но не пишет на шину

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use heapless::String;

use crate::config;

// ── PCF8574 bit mapping ──────────────────────────────────────────────────
// P0=RS, P1=RW, P2=EN, P3=BL, P4=D4, P5=D5, P6=D6, P7=D7

const RS_BIT: u8 = 0x01; // P0 — Register Select
#[allow(dead_code)]
const RW_BIT: u8 = 0x02; // P1 — Read/Write (всегда 0 = запись)
const EN_BIT: u8 = 0x04; // P2 — Enable strobe
const BL_BIT: u8 = 0x08; // P3 — Backlight
                         // D4..D7 на P4..P7: старший nibble данных = (data & 0x0F) << 4

// ── HD44780 команды ──────────────────────────────────────────────────────

const CMD_CLEAR_DISPLAY: u8 = 0x01;
const CMD_RETURN_HOME: u8 = 0x02;
const CMD_ENTRY_MODE_SET: u8 = 0x04;
const CMD_DISPLAY_CONTROL: u8 = 0x08;
const CMD_SET_CGRAM_ADDR: u8 = 0x40;
const CMD_SET_DDRAM_ADDR: u8 = 0x80;

// Параметры команд
const ENTRY_INCREMENT: u8 = 0x02; // Инкремент курсора
const ENTRY_SHIFT_OFF: u8 = 0x00; // Без сдвига дисплея
const DISPLAY_ON: u8 = 0x04; // Дисплей включён
const CURSOR_ON: u8 = 0x02; // Курсор видимый
const CURSOR_BLINK: u8 = 0x01; // Курсор мигает
#[allow(dead_code)]
const MODE_4BIT: u8 = 0x00; // 4-bit режим (бит DL=0)
#[allow(dead_code)]
const LINES_2: u8 = 0x08; // 2 строки
#[allow(dead_code)]
const DOTS_5X8: u8 = 0x00; // Шрифт 5x8

// ── Таймауты HD44780 ─────────────────────────────────────────────────────

/// Время удержания EN высоким (минимум 450нс по даташиту)
const EN_PULSE_US: u64 = 1;
/// Время между циклами включения EN (минимум 500нс)
const EN_CYCLE_US: u64 = 50;
/// Время очистки дисплея (~1.52мс по даташиту, берём с запасом)
const CLEAR_DELAY_MS: u64 = 2;
/// Время возврата домой (~1.52мс по даташиту)
const HOME_DELAY_MS: u64 = 2;
/// Задержка между командами (большинство команд < 40мкс)
const CMD_DELAY_US: u64 = 100;

// ── DisplayCommand — команды для display_task ────────────────────────────

#[derive(Debug, Clone)]
pub enum DisplayCommand {
    /// Очистить дисплей
    Clear,
    /// Вывести текст в позицию (x, y)
    Text { x: u8, y: u8, text: String<16> },
    /// Вывести текст с выравниванием
    TextAligned {
        align: Align,
        y: u8,
        text: String<40>,
    },
    /// Показать/скрыть курсор
    Cursor { visible: bool },
    /// Обновить дисплей из буфера (scroll)
    ScrollerUpdate,
    /// Установить custom-символ
    SetCustomChar { location: u8, data: [u8; 8] },
    /// Включить/выключить подсветку
    Backlight { on: bool },
}

#[derive(Debug, Clone, Copy, defmt::Format)]
pub enum Align {
    Left,
    Center,
    Right,
}

// ── Вспомогательные функции для String ────────────────────────────────────

pub fn make_str_40(s: &str) -> String<40> {
    let mut result = String::new();
    let _ = result.push_str(s);
    result
}

pub fn make_str_16(s: &str) -> String<16> {
    let mut result = String::new();
    let _ = result.push_str(s);
    result
}

// ── Hd44780I2c — драйвер HD44780 через PCF8574 ──────────────────────────
//
// Абстракция над I2C: пишет байт на шину через trait-подобный интерфейс.
// Реальная запись — через замыкание, которое захватывает &mut I2C.

/// Trait для записи байта в I2C — позволяет использовать разные бэкенды
pub trait I2cWriter {
    /// Записать один байт по I2C адресу
    fn write_byte(&mut self, addr: u8, data: u8);
}

/// Заглушка I2C — ничего не пишет, но драйвер работает
pub struct I2cNoop;

impl I2cWriter for I2cNoop {
    fn write_byte(&mut self, _addr: u8, _data: u8) {
        // Заглушка — I2C не подключён
    }
}

/// Реальный I2C через esp-hal — владеет I2C периферией
pub struct I2cEspHal {
    i2c: esp_hal::i2c::master::I2c<'static, esp_hal::Blocking>,
}

impl I2cEspHal {
    pub fn new(i2c: esp_hal::i2c::master::I2c<'static, esp_hal::Blocking>) -> Self {
        Self { i2c }
    }
}

impl I2cWriter for I2cEspHal {
    fn write_byte(&mut self, addr: u8, data: u8) {
        self.i2c.write(addr, &[data]).ok();
    }
}

// ── Hd44780I2c — параметризованный по I2C writer ────────────────────────

pub struct Hd44780I2c<W: I2cWriter> {
    addr: u8,
    backlight: bool,
    buf: [[u8; config::LCD_COLS as usize]; config::LCD_ROWS as usize],
    writer: W,
}

impl<W: I2cWriter> Hd44780I2c<W> {
    /// Создать экземпляр драйвера с любым I2C writer
    pub fn new(addr: u8, writer: W) -> Self {
        Self {
            addr,
            backlight: true,
            buf: [[b' '; config::LCD_COLS as usize]; config::LCD_ROWS as usize],
            writer,
        }
    }

    /// Инициализация HD44780 — 4-bit mode через I2C PCF8574
    pub async fn init(&mut self) {
        embassy_time::Timer::after_millis(50).await;

        self.write_nibble(0x03, false).await;
        embassy_time::Timer::after_millis(5).await;
        self.write_nibble(0x03, false).await;
        embassy_time::Timer::after_micros(150).await;
        self.write_nibble(0x03, false).await;
        embassy_time::Timer::after_micros(150).await;

        self.write_nibble(0x02, false).await;
        embassy_time::Timer::after_micros(150).await;

        self.write_cmd(0x28).await;
        self.write_cmd(CMD_DISPLAY_CONTROL | DISPLAY_ON).await;
        self.write_cmd(CMD_CLEAR_DISPLAY).await;
        embassy_time::Timer::after_millis(CLEAR_DELAY_MS).await;
        self.write_cmd(CMD_ENTRY_MODE_SET | ENTRY_INCREMENT | ENTRY_SHIFT_OFF)
            .await;

        self.load_custom_chars().await;

        self.write_cmd(CMD_RETURN_HOME).await;
        embassy_time::Timer::after_millis(HOME_DELAY_MS).await;
    }

    /// Записать nibble (4 бита) в HD44780 через PCF8574
    async fn write_nibble(&mut self, nibble: u8, rs: bool) {
        let rs_bit = if rs { RS_BIT } else { 0x00 };
        let rw_bit = 0;
        let bl_bit = if self.backlight { BL_BIT } else { 0x00 };
        let data_bits = (nibble & 0x0F) << 4;

        let byte_hi = data_bits | rs_bit | rw_bit | bl_bit | EN_BIT;
        let byte_lo = data_bits | rs_bit | rw_bit | bl_bit;

        self.writer.write_byte(self.addr, byte_hi);
        embassy_time::Timer::after_micros(EN_PULSE_US).await;
        self.writer.write_byte(self.addr, byte_lo);
        embassy_time::Timer::after_micros(EN_CYCLE_US).await;
    }

    /// Записать байт команды (RS=0) — два nibble
    pub async fn write_cmd(&mut self, cmd: u8) {
        self.write_nibble(cmd >> 4, false).await;
        self.write_nibble(cmd & 0x0F, false).await;
        embassy_time::Timer::after_micros(CMD_DELAY_US).await;
    }

    /// Записать байт данных (RS=1) — два nibble
    pub async fn write_data(&mut self, data: u8) {
        self.write_nibble(data >> 4, true).await;
        self.write_nibble(data & 0x0F, true).await;
        embassy_time::Timer::after_micros(CMD_DELAY_US).await;
    }

    /// Очистить дисплей
    pub async fn clear(&mut self) {
        self.write_cmd(CMD_CLEAR_DISPLAY).await;
        embassy_time::Timer::after_millis(CLEAR_DELAY_MS).await;
        self.buf = [[b' '; config::LCD_COLS as usize]; config::LCD_ROWS as usize];
    }

    /// Установить курсор в позицию (x, y)
    pub async fn set_cursor(&mut self, x: u8, y: u8) {
        let row_offset: [u8; 2] = [0x00, 0x40];
        let offset = row_offset[y as usize % config::LCD_ROWS as usize] + (x % config::LCD_COLS);
        self.write_cmd(CMD_SET_DDRAM_ADDR | offset).await;
    }

    /// Вывести строку символов в текущую позицию курсора
    pub async fn print(&mut self, text: &str) {
        for ch in text.bytes() {
            self.write_data(ch).await;
        }
    }

    /// Вывести один символ в текущую позицию
    pub async fn write_char(&mut self, ch: u8) {
        self.write_data(ch).await;
    }

    /// Установить custom-символ в CGRAM
    pub async fn set_custom_char(&mut self, location: u8, char_map: &[u8; 8]) {
        self.write_cmd(CMD_SET_CGRAM_ADDR | ((location & 0x07) << 3))
            .await;
        for &row in char_map {
            self.write_data(row).await;
        }
        self.write_cmd(CMD_SET_DDRAM_ADDR).await;
    }

    /// Включить/выключить курсор
    pub async fn set_cursor_visible(&mut self, visible: bool) {
        if visible {
            self.write_cmd(CMD_DISPLAY_CONTROL | DISPLAY_ON | CURSOR_ON | CURSOR_BLINK)
                .await;
        } else {
            self.write_cmd(CMD_DISPLAY_CONTROL | DISPLAY_ON).await;
        }
    }

    /// Установить подсветку
    pub fn set_backlight(&mut self, on: bool) {
        self.backlight = on;
    }

    /// Полное обновление дисплея из буфера
    pub async fn refresh(&mut self) {
        self.write_cmd(CMD_SET_DDRAM_ADDR).await;
        for x in 0..config::LCD_COLS as usize {
            let ch = self.buf[0][x];
            self.write_data(ch).await;
        }

        self.write_cmd(CMD_SET_DDRAM_ADDR | 0x40).await;
        for x in 0..config::LCD_COLS as usize {
            let ch = self.buf[1][x];
            self.write_data(ch).await;
        }
    }

    /// Записать символ в буфер (без вывода на дисплей)
    pub fn buffer_write(&mut self, x: usize, y: usize, ch: u8) {
        if x < config::LCD_COLS as usize && y < config::LCD_ROWS as usize {
            self.buf[y][x] = ch;
        }
    }

    /// Прочитать символ из буфера
    #[allow(dead_code)]
    pub fn buffer_read(&self, x: usize, y: usize) -> u8 {
        if x < config::LCD_COLS as usize && y < config::LCD_ROWS as usize {
            self.buf[y][x]
        } else {
            b' '
        }
    }

    /// Очистить буфер (без вывода на дисплей)
    pub fn buffer_clear(&mut self) {
        self.buf = [[b' '; config::LCD_COLS as usize]; config::LCD_ROWS as usize];
    }

    /// Загрузить custom-символы (латышские буквы)
    async fn load_custom_chars(&mut self) {
        let chars: [[u8; 8]; 8] = [
            [0x02, 0x00, 0x0E, 0x11, 0x1F, 0x11, 0x11, 0x00], // ā
            [0x02, 0x00, 0x0E, 0x11, 0x1F, 0x11, 0x11, 0x00], // ē
            [0x02, 0x00, 0x0C, 0x04, 0x04, 0x04, 0x0E, 0x00], // ī
            [0x02, 0x00, 0x0E, 0x11, 0x11, 0x11, 0x0E, 0x00], // ū
            [0x0C, 0x04, 0x0E, 0x12, 0x1F, 0x12, 0x12, 0x00], // ķ
            [0x0C, 0x04, 0x1E, 0x10, 0x1C, 0x12, 0x1C, 0x00], // ļ
            [0x00, 0x00, 0x16, 0x19, 0x11, 0x15, 0x15, 0x00], // ņ
            [0x04, 0x00, 0x1E, 0x20, 0x1C, 0x22, 0x1C, 0x00], // š
        ];
        for (i, char_data) in chars.iter().enumerate() {
            self.set_custom_char(i as u8, char_data).await;
        }
    }
}

// ── display_task — с заглушкой I2C (без реальной периферии) ────────────

pub async fn display_task(signal: &'static Signal<CriticalSectionRawMutex, DisplayCommand>) {
    let mut lcd = Hd44780I2c::new(config::LCD_I2C_ADDR, I2cNoop);
    lcd.init().await;

    lcd.set_cursor(0, 0).await;
    lcd.print("БАХИЛИЗАТОР").await;
    lcd.set_cursor(0, 1).await;
    lcd.print("v2.0 Rust/Embassy").await;
    embassy_time::Timer::after_secs(2).await;
    lcd.clear().await;

    loop {
        let cmd =
            embassy_futures::select::select(signal.wait(), embassy_time::Timer::after_millis(50))
                .await;

        match cmd {
            embassy_futures::select::Either::First(cmd) => {
                process_display_command(&mut lcd, cmd).await;
            }
            embassy_futures::select::Either::Second(_) => {}
        }
    }
}

// ── display_task_with_i2c — с реальным esp-hal I2C ────────────────────

pub async fn display_task_with_i2c(
    signal: &'static Signal<CriticalSectionRawMutex, DisplayCommand>,
    i2c: esp_hal::i2c::master::I2c<'static, esp_hal::Blocking>,
) {
    let writer = I2cEspHal::new(i2c);
    let mut lcd = Hd44780I2c::new(config::LCD_I2C_ADDR, writer);
    lcd.init().await;

    lcd.set_cursor(0, 0).await;
    lcd.print("БАХИЛИЗАТОР").await;
    lcd.set_cursor(0, 1).await;
    lcd.print("v2.0 LilyGo").await;
    embassy_time::Timer::after_secs(2).await;
    lcd.clear().await;

    loop {
        let cmd =
            embassy_futures::select::select(signal.wait(), embassy_time::Timer::after_millis(50))
                .await;

        match cmd {
            embassy_futures::select::Either::First(cmd) => {
                process_display_command(&mut lcd, cmd).await;
            }
            embassy_futures::select::Either::Second(_) => {}
        }
    }
}

/// Обработка команды дисплея
async fn process_display_command<W: I2cWriter>(lcd: &mut Hd44780I2c<W>, cmd: DisplayCommand) {
    match cmd {
        DisplayCommand::Clear => {
            lcd.clear().await;
        }
        DisplayCommand::Text { x, y, ref text } => {
            lcd.set_cursor(x, y).await;
            lcd.print(text.as_str()).await;
        }
        DisplayCommand::TextAligned { align, y, ref text } => {
            let x = match align {
                Align::Left => 0,
                Align::Center => {
                    let len = text.len() as u8;
                    if len >= config::LCD_COLS {
                        0
                    } else {
                        (config::LCD_COLS - len) / 2
                    }
                }
                Align::Right => {
                    let len = text.len() as u8;
                    config::LCD_COLS.saturating_sub(len)
                }
            };
            lcd.set_cursor(x, y).await;
            lcd.print(text.as_str()).await;
        }
        DisplayCommand::Cursor { visible } => {
            lcd.set_cursor_visible(visible).await;
        }
        DisplayCommand::ScrollerUpdate => {
            lcd.refresh().await;
        }
        DisplayCommand::SetCustomChar { location, data } => {
            lcd.set_custom_char(location, &data).await;
        }
        DisplayCommand::Backlight { on } => {
            lcd.set_backlight(on);
        }
    }
}