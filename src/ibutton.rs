//! 1-Wire iButton драйвер (bit-bang async)
//!
//! Перенос из 1-wire.c — асинхронный bit-bang на GPIO
//! DS1990A: Reset pulse → Read ROM (0x33) → 8 байт (family+serial+CRC)
//!
//! Тайминги (из оригинала на MSP430):
//!   Reset: 480µs low → 70µs wait → read presence → 410µs
//!   Write 1: 6µs low → release → 64µs
//!   Write 0: 60µs low → release → 10µs
//!   Read:    10µs low → release → 9µs wait → read → 55µs
//!
//! CRC-8: полином x^8+x^5+x^4+1 (= 0x31, reversed 0x8C)

use embassy_sync::channel::Sender;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;

use crate::state::{IBUTTON_LEN, Settings, VendingState};

// ── 1-Wire команды ────────────────────────────────────────────────────────

const CMD_READ_ROM: u8 = 0x33;
#[allow(dead_code)]
const CMD_MATCH_ROM: u8 = 0x55;
#[allow(dead_code)]
const CMD_SKIP_ROM: u8 = 0xCC;

// ── IbuttonEvent — событие от iButton ─────────────────────────────────────

#[derive(Debug, Clone, defmt::Format)]
pub struct IbuttonEvent {
    /// 8-байтовый ключ iButton
    pub key: [u8; IBUTTON_LEN],
    /// Ключ принят (true) или отклонён (false)
    pub accepted: bool,
}

// ── IbuttonDriver — драйвер 1-Wire шины ─────────────────────────────────
//
/// Инкапсулирует GPIO пин и timing для bit-bang 1-Wire
/// Пин конфигурируется как OpenDrain: output low для передачи,
/// input (с pull-up) для чтения/освобождения шины
///
/// В оригинале (1-wire.c):
///   ONE_WIRE_OUT = output mode
///   ONE_WIRE_IN  = input mode (с pull-up)
///   ONE_WIRE_SET_LO = установить low
///   ONE_WIRE_GET_BIT = прочитать уровень

pub struct IbuttonDriver {
    /// Пин 1-Wire (PA11 на STM32F103)
    /// В реальной интеграции: AnyPin или PA11<Output<OpenDrain>>
    _pin_marker: (),
}

impl IbuttonDriver {
    /// Создать драйвер (заглушка — без реального пина)
    pub fn new() -> Self {
        Self { _pin_marker: () }
    }

    /// Инициализация шины: пин в input с pull-up, выход low
    ///
    /// В оригинале: initOneWire()
    ///   ONE_WIRE_REN &= ~ONE_WIRE_PIN;  // отключить pull-up/pull-down
    ///   ONE_WIRE_SET_LO;                // установить low в регистре выхода
    ///   ONE_WIRE_IN;                    // переключить в input (высокий импеданс)
    pub async fn init(&mut self) {
        // В реальном железе:
        // let pin = PA11.into_floating_input();
        // Или: PA11.into_output_open_drain()
        //   .with_internal_pull_up()
        //   .set_as_input();
        self.release_bus().await;
    }

    // ── Reset pulse ──────────────────────────────────────────────────
    //
    /// В оригинале: oneWireReset()
    ///   ONE_WIRE_OUT;       // output mode
    ///   delay_mks(480);     // hold low 480µs
    ///   ONE_WIRE_IN;        // release (input)
    ///   delay_mks(70);      // wait 70µs
    ///   present = ONE_WIRE_GET_BIT; // read presence
    ///   delay_mks(410);     // wait 410µs
    ///   return (present > 0);
    ///
    /// Возвращает true если устройство присутствует (presence pulse)

    pub async fn reset_pulse(&mut self) -> bool {
        // Pull bus low for 480µs
        self.pull_low().await;
        embassy_time::Timer::after_micros(480).await;

        // Release bus
        self.release_bus().await;

        // Wait 70µs then read presence
        embassy_time::Timer::after_micros(70).await;
        let present = self.read_bus();

        // Wait remaining 410µs
        embassy_time::Timer::after_micros(410).await;

        // В оригинале: return (present > 0)
        // present=0 (bus low) = устройство есть
        !present
    }

    // ── Write bit ────────────────────────────────────────────────────
    //
    /// В оригинале: oneWireTxBit(bit)
    ///   bit=1: OUT → 6µs → IN → 64µs
    ///   bit=0: OUT → 60µs → IN → 10µs

    pub async fn write_bit(&mut self, bit: u8) {
        self.pull_low().await;

        if bit != 0 {
            // Write 1: short pulse
            embassy_time::Timer::after_micros(6).await;
            self.release_bus().await;
            embassy_time::Timer::after_micros(64).await;
        } else {
            // Write 0: long pulse
            embassy_time::Timer::after_micros(60).await;
            self.release_bus().await;
            embassy_time::Timer::after_micros(10).await;
        }
    }

    // ── Read bit ─────────────────────────────────────────────────────
    //
    /// В оригинале: oneWireRxBit()
    ///   ONE_WIRE_OUT;       // pull low
    ///   delay_mks(10);      // 10µs (у оригинала, не 6µs — видимо с запасом)
    ///   ONE_WIRE_IN;        // release
    ///   delay_mks(9);       // sample point
    ///   data = ONE_WIRE_GET_BIT;
    ///   delay_mks(55);      // rest of time slot

    pub async fn read_bit(&mut self) -> u8 {
        self.pull_low().await;
        embassy_time::Timer::after_micros(10).await;

        self.release_bus().await;
        embassy_time::Timer::after_micros(9).await;

        let data = if self.read_bus() { 1 } else { 0 };

        embassy_time::Timer::after_micros(55).await;

        data
    }

    // ── Write byte ───────────────────────────────────────────────────
    //
    /// В оригинале: oneWirePutc(byte)
    ///   LSB first: byte & 0x01 → byte >>= 1

    pub async fn write_byte(&mut self, byte: u8) {
        let mut b = byte;
        for _ in 0..8 {
            self.write_bit(b & 0x01).await;
            b >>= 1;
        }
    }

    // ── Read byte ────────────────────────────────────────────────────
    //
    /// В оригинале: oneWiteGetc() [sic — опечатка в оригинале]
    ///   LSB first: bit << c → ret |= bit<<c

    pub async fn read_byte(&mut self) -> u8 {
        let mut result: u8 = 0;
        for i in 0..8 {
            result |= self.read_bit().await << i;
        }
        result
    }

    // ── Read ROM ──────────────────────────────────────────────────────
    //
    /// Полная операция чтения ключа:
    /// 1. Reset pulse
    /// 2. Read ROM command (0x33)
    /// 3. Прочитать 8 байт
    /// 4. Проверить CRC
    ///
    /// Возвращает Some(key) если ключ прочитан и CRC верный

    pub async fn read_rom(&mut self) -> Option<[u8; IBUTTON_LEN]> {
        // Reset pulse
        if !self.reset_pulse().await {
            return None; // Нет устройства на шине
        }

        // Команда Read ROM
        self.write_byte(CMD_READ_ROM).await;

        // Читаем 8 байт
        let mut key = [0u8; IBUTTON_LEN];
        for byte in key.iter_mut() {
            *byte = self.read_byte().await;
        }

        // Проверяем CRC
        if check_crc(&key) {
            Some(key)
        } else {
            None
        }
    }

    // ── GPIO заглушки ────────────────────────────────────────────────
    //
    /// Эти методы заменяются на реальные GPIO операции при интеграции

    /// Подтянуть шину к земле (output low)
    async fn pull_low(&mut self) {
        // В реальном железе:
        // pin.set_low().ok();
        // или для OpenDrain: pin.set_as_output()
        let _ = self;
    }

    /// Освободить шину (input с pull-up)
    async fn release_bus(&mut self) {
        // В реальном железе:
        // pin.set_as_input(); // OpenDrain: high-Z = released
        let _ = self;
    }

    /// Прочитать уровень шины (true = high, false = low)
    fn read_bus(&self) -> bool {
        // В реальном железе:
        // pin.is_high()
        // Safety: заглушка — шина отпущена = high
        true
    }
}

// ── CRC-8 Dallas/Maxim ───────────────────────────────────────────────────
//
/// CRC-8 для iButton (DS1990A): полином x^8+x^5+x^4+1
/// reflected polynomial = 0x8C
/// В оригинале: _crc_ibutton_update()

pub fn check_crc(key: &[u8; IBUTTON_LEN]) -> bool {
    compute_crc(&key[..7]) == key[7]
}

/// Вычислить CRC-8 по Dallas/Maxim
pub fn compute_crc(data: &[u8]) -> u8 {
    let mut crc: u8 = 0;
    for &byte in data {
        crc ^= byte;
        for _ in 0..8 {
            if crc & 0x01 != 0 {
                crc = (crc >> 1) ^ 0x8C;
            } else {
                crc >>= 1;
            }
        }
    }
    crc
}

// ── Проверка ключа по whitelist ──────────────────────────────────────────
//
/// Сравнить прочитанный ROM с whitelist в Settings
/// Settings.keys = [[IbuttonKey; 2]; 3] — до 6 ключей
/// Возвращает уровень доступа (0=сервисный, 1=технический) или None

#[derive(Debug, Clone, Copy, defmt::Format, PartialEq)]
pub enum KeyAccess {
    /// Сервисный ключ (полный доступ)
    Service = 0,
    /// Технический ключ (ограниченный доступ)
    Technical = 1,
}

pub fn check_key(settings: &Settings, key: &[u8; IBUTTON_LEN]) -> Option<KeyAccess> {
    // Все нули — пустой слот, пропускаем
    let is_empty = key.iter().all(|&b| b == 0);
    if is_empty {
        return None;
    }

    // Проверяем каждый уровень доступа
    for level in 0..settings.keys.len() {
        for slot in 0..settings.keys[level].len() {
            let stored = &settings.keys[level][slot];
            let stored_empty = stored.iter().all(|&b| b == 0);
            if !stored_empty && stored == key {
                return match level {
                    0 => Some(KeyAccess::Service),
                    _ => Some(KeyAccess::Technical),
                };
            }
        }
    }

    None
}

// ── Задача iButton ───────────────────────────────────────────────────────
//
/// Основная задача опроса iButton
/// Опрашивает шину каждые 200мс, при обнаружении ключа —
/// проверяет по whitelist и отправляет событие

pub async fn run(
    ibutton_tx: Sender<'static, CriticalSectionRawMutex, IbuttonEvent, 1>,
    state: &'static embassy_sync::mutex::Mutex<
        embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
        core::cell::RefCell<VendingState>,
    >,
) {
    let mut driver = IbuttonDriver::new();
    driver.init().await;

    loop {
        // Шаг 1: Попытка чтения ключа
        if let Some(key) = driver.read_rom().await {
            // Шаг 2: Проверка по whitelist
            let accepted = {
                let guard = state.lock().await;
                let s = guard.borrow();
                check_key(&s.settings, &key).is_some()
            };

            // Шаг 3: Отправить событие
            let _ = ibutton_tx.try_send(IbuttonEvent {
                key,
                accepted,
            });
        }

        // Опрос раз в 200мс
        embassy_time::Timer::after_millis(200).await;
    }
}