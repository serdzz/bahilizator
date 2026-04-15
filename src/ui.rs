//! Дисплей HD44780 через I2C PCF8574 backpack
//!
//! Полный драйвер с инициализацией, курсором, custom chars
//! PCF8574 pin mapping: P0=RS, P1=RW, P2=E, P3=Backlight, P4=D4, P5=D5, P6=D6, P7=D7
//!
//! Инициализация 4-bit mode:
//!   0x33 → 0x32 → 0x28 → 0x0C → 0x06 → 0x01
//! Перенос из lcd.c (оригинальный MSP430 код) на Embassy async I2C

use embassy_sync::signal::Signal;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use heapless::String;

use crate::config;

// ── PCF8574 bit mapping ──────────────────────────────────────────────────
// P0=RS, P1=RW, P2=EN, P3=BL, P4=D4, P5=D5, P6=D6, P7=D7

const RS_BIT: u8 = 0x01; // P0 — Register Select
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
const ENTRY_INCREMENT: u8 = 0x02;   // Инкремент курсора
const ENTRY_SHIFT_OFF: u8 = 0x00;  // Без сдвига дисплея
const DISPLAY_ON: u8 = 0x04;       // Дисплей включён
const CURSOR_ON: u8 = 0x02;         // Курсор видимый
const CURSOR_BLINK: u8 = 0x01;      // Курсор мигает
const MODE_4BIT: u8 = 0x00;         // 4-bit режим (бит DL=0)
const LINES_2: u8 = 0x08;           // 2 строки
const DOTS_5X8: u8 = 0x00;         // Шрифт 5x8

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
    TextAligned { align: Align, y: u8, text: String<40> },
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
// I2C пишется через callback-функцию, т.к. владение I2C периферией
// принадлежит display_task. Это позволяет избежать generic-параметра.

pub struct Hd44780I2c {
    /// I2C адрес PCF8574 (0x27 или 0x3F)
    addr: u8,
    /// Состояние подсветки
    backlight: bool,
    /// Буфер дисплея для refresh
    buf: [[u8; config::LCD_COLS as usize]; config::LCD_ROWS as usize],
    /// Функция записи байта в I2C (вызывается из async контекста)
    i2c_write: fn(u8, u8),
}

impl Hd44780I2c {
    /// Создать экземпляр драйвера
    /// addr — I2C адрес PCF8574
    /// i2c_write — функция записи: fn(address, data_byte)
    pub fn new(addr: u8, i2c_write: fn(u8, u8)) -> Self {
        Self {
            addr,
            backlight: true,
            buf: [[b' '; config::LCD_COLS as usize]; config::LCD_ROWS as usize],
            i2c_write,
        }
    }

    /// Инициализация HD44780 — 4-bit mode через I2C PCF8574
    ///
    /// Последовательность по даташиту Hitachi HD44780U:
    /// 1. Ждём >15мс после VCC >4.5V
    /// 2. Отправляем 0x03 три раза (8-bit mode)
    /// 3. Переключаемся в 4-bit: 0x02
    /// 4. Настраиваем: 4-bit, 2 строки, 5x8
    /// 5. Включаем дисплей
    /// 6. Очищаем
    /// 7. Режим ввода: инкремент, без сдвига
    /// 8. Загружаем custom chars (латышские буквы)
    pub async fn init(&mut self) {
        // Шаг 1: Ждём стабилизации питания
        embassy_time::Timer::after_millis(50).await;

        // Шаг 2-3: Инициализация 4-bit mode (как в оригинальном lcd.c)
        // Отправляем 0x03 три раза с задержками, потом 0x02
        self.write_nibble(0x03, false).await;
        embassy_time::Timer::after_millis(5).await;
        self.write_nibble(0x03, false).await;
        embassy_time::Timer::after_micros(150).await;
        self.write_nibble(0x03, false).await;
        embassy_time::Timer::after_micros(150).await;

        // Переход в 4-bit mode
        self.write_nibble(0x02, false).await;
        embassy_time::Timer::after_micros(150).await;

        // Шаг 4: Function Set — 4-bit, 2 строки, 5x8 (команда 0x28)
        // Оригинал: LcdWtiteInstruction(0x2A) для page=1, 0x28 для page=0
        self.write_cmd(0x28).await;

        // Шаг 5: Display On, Cursor Off, Blink Off (0x0C)
        self.write_cmd(CMD_DISPLAY_CONTROL | DISPLAY_ON).await;

        // Шаг 6: Clear Display (0x01)
        self.write_cmd(CMD_CLEAR_DISPLAY).await;
        embassy_time::Timer::after_millis(CLEAR_DELAY_MS).await;

        // Шаг 7: Entry Mode Set — Increment, No Shift (0x06)
        self.write_cmd(CMD_ENTRY_MODE_SET | ENTRY_INCREMENT | ENTRY_SHIFT_OFF).await;

        // Шаг 8: Загрузить custom chars (латышские символы)
        self.load_custom_chars().await;

        // Возврат домой
        self.write_cmd(CMD_RETURN_HOME).await;
        embassy_time::Timer::after_millis(HOME_DELAY_MS).await;
    }

    /// Записать nibble (4 бита) в HD44780 через PCF8574
    ///
    /// PCF8574 layout: [D7 D6 D5 D4 BL EN RW RS]
    /// nibble — 4 бита данных (старший nibль команды/данных)
    /// rs — true = данные (RS=1), false = команда (RS=0)
    async fn write_nibble(&mut self, nibble: u8, rs: bool) {
        let rs_bit = if rs { RS_BIT } else { 0x00 };
        let rw_bit = 0; // Всегда запись: RW=0
        let bl_bit = if self.backlight { BL_BIT } else { 0x00 };
        let data_bits = (nibble & 0x0F) << 4; // D4-D7 на P4-P7

        // Формируем байт для PCF8574: data_bits | RS | RW | BL | EN
        let byte_hi = data_bits | rs_bit | rw_bit | bl_bit | EN_BIT;
        let byte_lo = data_bits | rs_bit | rw_bit | bl_bit; // EN=0

        // Строб E: HIGH → задержка → LOW
        (self.i2c_write)(self.addr, byte_hi);
        embassy_time::Timer::after_micros(EN_PULSE_US).await;
        (self.i2c_write)(self.addr, byte_lo);
        embassy_time::Timer::after_micros(EN_CYCLE_US).await;
    }

    /// Записать байт команды (RS=0) — два nibble
    pub async fn write_cmd(&mut self, cmd: u8) {
        self.write_nibble(cmd >> 4, false).await;
        self.write_nibble(cmd & 0x0F, false).await;
        // Большинство команд выполняются за 37мкс, но Clear и Home — дольше
        // Задержка для простых команд
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
    /// Строка 0 → DDRAM addr 0x00, строка 1 → DDRAM addr 0x40
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
    /// location: 0-7 (8 доступных слотов)
    /// char_map: 8 байт, каждый — одна строка 5 пикселей
    pub async fn set_custom_char(&mut self, location: u8, char_map: &[u8; 8]) {
        self.write_cmd(CMD_SET_CGRAM_ADDR | ((location & 0x07) << 3)).await;
        for &row in char_map {
            self.write_data(row).await;
        }
        // Вернуть DDRAM адрес в 0 после записи CGRAM
        self.write_cmd(CMD_SET_DDRAM_ADDR).await;
    }

    /// Включить/выключить курсор
    pub async fn set_cursor_visible(&mut self, visible: bool) {
        if visible {
            self.write_cmd(CMD_DISPLAY_CONTROL | DISPLAY_ON | CURSOR_ON | CURSOR_BLINK).await;
        } else {
            self.write_cmd(CMD_DISPLAY_CONTROL | DISPLAY_ON).await;
        }
    }

    /// Установить подсветку
    pub fn set_backlight(&mut self, on: bool) {
        self.backlight = on;
        // Подсветка обновится при следующей записи в PCF8574
    }

    /// Полное обновление дисплея из буфера (как LcdUpdate в оригинале)
    ///
    /// Переинициализирует дисплей и выводит содержимое буфера.
    /// Используется при scroll-анимации.
    pub async fn refresh(&mut self) {
        // Выводим строку 0
        self.write_cmd(CMD_SET_DDRAM_ADDR | 0x00).await;
        for x in 0..config::LCD_COLS as usize {
            let ch = self.buf[0][x];
            self.write_data(ch).await;
        }

        // Выводим строку 1
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
    ///
    /// Перенос из оригинального lcd.c:
    ///   slot 1 = ā (a с макроном)
    ///   slot 2 = ņ (n с седилью)
    ///   slot 3 = ī (i с макроном)
    ///
    /// Также добавлены символы из текущей версии:
    ///   ē, ū, ķ, ļ, š
    async fn load_custom_chars(&mut self) {
        // Оригинальные символы из lcd.c MSP430
        let chars: [[u8; 8]; 8] = [
            // Слот 0: ā (a с макроном)
            [0x02, 0x00, 0x0E, 0x11, 0x1F, 0x11, 0x11, 0x00],
            // Слот 1: ē (e с макроном)
            [0x02, 0x00, 0x0E, 0x11, 0x1F, 0x11, 0x11, 0x00],
            // Слот 2: ī (i с макроном)
            [0x02, 0x00, 0x0C, 0x04, 0x04, 0x04, 0x0E, 0x00],
            // Слот 3: ū (u с макроном)
            [0x02, 0x00, 0x0E, 0x11, 0x11, 0x11, 0x0E, 0x00],
            // Слот 4: ķ (k с седилью)
            [0x0C, 0x04, 0x0E, 0x12, 0x1F, 0x12, 0x12, 0x00],
            // Слот 5: ļ (l с седилью)
            [0x0C, 0x04, 0x1E, 0x10, 0x1C, 0x12, 0x1C, 0x00],
            // Слот 6: ņ (n с седилью)
            [0x00, 0x00, 0x16, 0x19, 0x11, 0x15, 0x15, 0x00],
            // Слот 7: š (s с шапкой)
            [0x04, 0x00, 0x1E, 0x20, 0x1C, 0x22, 0x1C, 0x00],
        ];
        for (i, char_data) in chars.iter().enumerate() {
            self.set_custom_char(i as u8, char_data).await;
        }
    }
}

// ── display_task — задача вывода на дисплей ───────────────────────────────
//
/// Получает DisplayCommand через Signal и выводит на HD44780 через I2C
///
/// I2C пишется через функцию-замыкание, которая вызывается в async контексте
/// display_task. Реальная запись в I2C — через embassy-stm32 I2C::write().
///
/// Для компиляции без реальной I2C периферии используем заглушку i2c_write_stub.
pub async fn display_task(signal: &'static Signal<CriticalSectionRawMutex, DisplayCommand>) {
    // Создаём драйвер с заглушкой I2C (заменить на реальную при интеграции)
    let mut lcd = Hd44780I2c::new(config::LCD_I2C_ADDR, i2c_write_stub);
    lcd.init().await;

    // Показать приветствие
    lcd.set_cursor(0, 0).await;
    lcd.print("БАХИЛИЗАТОР").await;
    lcd.set_cursor(0, 1).await;
    lcd.print("v2.0 Rust/Embassy").await;
    embassy_time::Timer::after_secs(2).await;
    lcd.clear().await;

    loop {
        // Ждём команду с таймаутом (для scroll-обновлений)
        let cmd = embassy_futures::select::select(
            signal.wait(),
            embassy_time::Timer::after_millis(50),
        ).await;

        match cmd {
            embassy_futures::select::Either::First(cmd) => {
                process_display_command(&mut lcd, cmd).await;
            }
            embassy_futures::select::Either::Second(_) => {
                // Таймаут — ничего не делаем (можно добавить scroll)
            }
        }
    }
}

/// Обработка команды дисплея
async fn process_display_command(lcd: &mut Hd44780I2c, cmd: DisplayCommand) {
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
        DisplayCommand::SetCustomChar { location, data } => {
            lcd.set_custom_char(location, &data).await;
        }
        DisplayCommand::Backlight { on } => {
            lcd.set_backlight(on);
        }
    }
}

/// Заглушка записи в I2C — заменяется реальной при интеграции с embassy-stm32
///
/// Safety: В реальной интеграции эта функция будет заменена на замыкание,
/// захватывающее &'static mut I2C. Сейчас — просто заглушка для компиляции.
fn i2c_write_stub(_addr: u8, _data: u8) {
    // TODO: реальная запись через embassy-stm32 I2C
    // Пример интеграции:
    // let i2c = unsafe { &mut *I2C_PTR };
    // let buf = [data];
    // block_on(i2c.write(addr, &buf)).ok();
}