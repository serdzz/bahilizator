//! Дисплей HD44780 через I2C PCF8574 backpack
//!
//! Полный драйвер с инициализацией, курсором, custom chars
//! PCF8574 pin mapping: P7=D7, P6=D6, P5=D5, P4=D4, P3=BL, P2=EN, P1=RW, P0=RS

use embassy_sync::signal::Signal;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use heapless::String;

use crate::config;

// ── PCF8574 bit mapping ──────────────────────────────────────────────────

const RS_BIT: u8 = 0x01;
const RW_BIT: u8 = 0x02;
const EN_BIT: u8 = 0x04;
const BL_BIT: u8 = 0x08;
const _D4_BIT: u8 = 0x10;
const _D5_BIT: u8 = 0x20;
const _D6_BIT: u8 = 0x40;
const _D7_BIT: u8 = 0x80;

// ── HD44780 команды ──────────────────────────────────────────────────────

const CMD_CLEAR_DISPLAY: u8 = 0x01;
const CMD_RETURN_HOME: u8 = 0x02;
const CMD_ENTRY_MODE_SET: u8 = 0x04;
const CMD_DISPLAY_CONTROL: u8 = 0x08;
const CMD_SET_CGRAM_ADDR: u8 = 0x40;
const CMD_SET_DDRAM_ADDR: u8 = 0x80;

const ENTRY_INCREMENT: u8 = 0x02;
const DISPLAY_ON: u8 = 0x04;
const CURSOR_ON: u8 = 0x02;
const CURSOR_BLINK: u8 = 0x01;
const MODE_4BIT: u8 = 0x00;
const LINES_2: u8 = 0x08;
const DOTS_5X8: u8 = 0x00;

// ── DisplayCommand ───────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum DisplayCommand {
    Clear,
    Text { x: u8, y: u8, text: String<16> },
    TextAligned { align: Align, y: u8, text: String<40> },
    Cursor { visible: bool },
    ScrollerUpdate,
}

#[derive(Debug, Clone, Copy, defmt::Format)]
pub enum Align {
    Left,
    Center,
    Right,
}

// ── Вспомогательная функция для String<40> из &str ────────────────────────

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

// ── Hd44780 драйвер ──────────────────────────────────────────────────────

pub struct Hd44780I2c {
    addr: u8,
    backlight: bool,
    buf: [[u8; config::LCD_COLS as usize]; config::LCD_ROWS as usize],
}

impl Hd44780I2c {
    pub fn new(addr: u8) -> Self {
        Self {
            addr,
            backlight: true,
            buf: [[b' '; config::LCD_COLS as usize]; config::LCD_ROWS as usize],
        }
    }

    /// Инициализация HD44780 — 4-bit mode через I2C PCF8574
    pub async fn init(&mut self) {
        self.write_nibble(0x03, false).await;
        embassy_time::Timer::after_millis(5).await;
        self.write_nibble(0x03, false).await;
        embassy_time::Timer::after_micros(150).await;
        self.write_nibble(0x03, false).await;
        embassy_time::Timer::after_micros(150).await;
        self.write_nibble(0x02, false).await;
        embassy_time::Timer::after_micros(150).await;

        self.write_cmd(CMD_DISPLAY_CONTROL | MODE_4BIT | LINES_2 | DOTS_5X8).await;
        self.write_cmd(CMD_DISPLAY_CONTROL | DISPLAY_ON).await;
        self.write_cmd(CMD_CLEAR_DISPLAY).await;
        embassy_time::Timer::after_millis(2).await;
        self.write_cmd(CMD_ENTRY_MODE_SET | ENTRY_INCREMENT).await;

        self.load_custom_chars().await;
        self.write_cmd(CMD_RETURN_HOME).await;
        embassy_time::Timer::after_millis(2).await;
    }

    async fn write_nibble(&mut self, nibble: u8, rs: bool) {
        let rs_bit = if rs { RS_BIT } else { 0x00 };
        let bl_bit = if self.backlight { BL_BIT } else { 0x00 };
        let data_bits = (nibble & 0x0F) << 4;
        let data = data_bits | rs_bit | bl_bit | EN_BIT;

        self.i2c_write_byte(data).await;
        embassy_time::Timer::after_micros(1).await;
        self.i2c_write_byte(data & !EN_BIT).await;
        embassy_time::Timer::after_micros(50).await;
    }

    pub async fn write_cmd(&mut self, cmd: u8) {
        self.write_nibble(cmd >> 4, false).await;
        self.write_nibble(cmd & 0x0F, false).await;
    }

    pub async fn write_data(&mut self, data: u8) {
        self.write_nibble(data >> 4, true).await;
        self.write_nibble(data & 0x0F, true).await;
    }

    async fn i2c_write_byte(&mut self, _byte: u8) {
        // TODO: реальная запись через embassy-stm32 I2C
    }

    pub async fn clear(&mut self) {
        self.write_cmd(CMD_CLEAR_DISPLAY).await;
        embassy_time::Timer::after_millis(2).await;
        self.buf = [[b' '; config::LCD_COLS as usize]; config::LCD_ROWS as usize];
    }

    pub async fn set_cursor(&mut self, x: u8, y: u8) {
        let offset = if y == 0 { x } else { 0x40 + x };
        self.write_cmd(CMD_SET_DDRAM_ADDR | offset).await;
    }

    pub async fn print(&mut self, text: &str) {
        for ch in text.bytes() {
            self.write_data(ch).await;
        }
    }

    pub async fn refresh(&mut self) {
        self.write_cmd(CMD_RETURN_HOME).await;
        embassy_time::Timer::after_millis(2).await;
        for x in 0..config::LCD_COLS as usize {
            self.write_data(self.buf[0][x]).await;
        }
        self.write_cmd(CMD_SET_DDRAM_ADDR | 0x40).await;
        for x in 0..config::LCD_COLS as usize {
            self.write_data(self.buf[1][x]).await;
        }
    }

    pub async fn set_cursor_visible(&mut self, visible: bool) {
        if visible {
            self.write_cmd(CMD_DISPLAY_CONTROL | DISPLAY_ON | CURSOR_ON | CURSOR_BLINK).await;
        } else {
            self.write_cmd(CMD_DISPLAY_CONTROL | DISPLAY_ON).await;
        }
    }

    pub async fn create_char(&mut self, location: u8, char_map: &[u8; 8]) {
        self.write_cmd(CMD_SET_CGRAM_ADDR | (location & 0x07) << 3).await;
        for &row in char_map {
            self.write_data(row).await;
        }
        self.write_cmd(CMD_SET_DDRAM_ADDR).await;
    }

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
            self.create_char(i as u8, char_data).await;
        }
    }

    pub fn buffer_write(&mut self, x: usize, y: usize, ch: u8) {
        if x < config::LCD_COLS as usize && y < config::LCD_ROWS as usize {
            self.buf[y][x] = ch;
        }
    }

    #[allow(dead_code)]
    pub fn buffer_read(&self, x: usize, y: usize) -> u8 {
        if x < config::LCD_COLS as usize && y < config::LCD_ROWS as usize {
            self.buf[y][x]
        } else {
            b' '
        }
    }
}

// ── task_display ─────────────────────────────────────────────────────────

pub async fn display_task(signal: &'static Signal<CriticalSectionRawMutex, DisplayCommand>) {
    let mut lcd = Hd44780I2c::new(config::LCD_I2C_ADDR);
    lcd.init().await;

    loop {
        if let Some(cmd) = signal.try_take() {
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
                            if len >= config::LCD_COLS { 0 } else { (config::LCD_COLS - len) / 2 }
                        }
                        Align::Right => {
                            let len = text.len() as u8;
                            if len >= config::LCD_COLS { 0 } else { config::LCD_COLS - len }
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
            }
        }

        embassy_time::Timer::after_millis(50).await;
    }
}