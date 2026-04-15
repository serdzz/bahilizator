//! 1-Wire iButton драйвер (bit-bang async)
//!
//! Перенос из 1-wire.c — асинхронный bit-bang на GPIO

use embassy_sync::channel::Sender;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;

use crate::state::IBUTTON_LEN;

// ── IbuttonEvent — событие от iButton ─────────────────────────────────────

#[derive(Debug, Clone, defmt::Format)]
pub struct IbuttonEvent {
    /// 8-байтовый ключ iButton
    pub key: [u8; IBUTTON_LEN],
    /// Ключ принят (true) или отклонён (false)
    pub accepted: bool,
}

// ── 1-Wire команды ────────────────────────────────────────────────────────

const CMD_READ_ROM: u8 = 0x33;
const CMD_MATCH_ROM: u8 = 0x55;
const CMD_SKIP_ROM: u8 = 0xCC;

// ── Задача iButton ───────────────────────────────────────────────────────

/// Основная задача чтения iButton
pub async fn run(
    ibutton_tx: Sender<'static, CriticalSectionRawMutex, IbuttonEvent, 1>,
) {
    loop {
        // Шаг 1: Reset pulse
        if reset_pulse().await {
            // Шаг 2: Присутствует устройство — читаем ROM
            let mut key = [0u8; IBUTTON_LEN];
            write_byte(CMD_READ_ROM).await;
            for byte in key.iter_mut() {
                *byte = read_byte().await;
            }

            // Шаг 3: Проверить CRC (8-й байт = CRC первых 7)
            if check_crc(&key) {
                let _ = ibutton_tx.try_send(IbuttonEvent {
                    key,
                    accepted: true,
                });
            }
        }

        // Опрос раз в 200мс
        embassy_time::Timer::after_millis(200).await;
    }
}

// ── 1-Wire примитивы (bit-bang) ──────────────────────────────────────────

/// Reset pulse — возвращает true если устройство присутствует
async fn reset_pulse() -> bool {
    // TODO: реальный GPIO bit-bang
    // 1. Pull low for 480µs
    // 2. Release and wait 70µs
    // 3. Read: low = presence
    // 4. Wait 410µs
    false // заглушка
}

/// Записать байт на 1-Wire шину
async fn write_byte(_byte: u8) {
    // TODO: реальный GPIO bit-bang
    // for i in 0..8 { write_bit((byte >> i) & 1) }
}

/// Прочитать байт с 1-Wire шины
async fn read_byte() -> u8 {
    // TODO: реальный GPIO bit-bang
    // let mut byte = 0u8;
    // for i in 0..8 { byte |= (read_bit() as u8) << i; }
    0 // заглушка
}

/// Записать один бит
#[allow(dead_code)]
async fn write_bit(_bit: u8) {
    // TODO: реальный GPIO bit-bang
    // bit=1: pull low 6µs, release, wait 64µs
    // bit=0: pull low 60µs, release, wait 10µs
}

/// Прочитать один бит
#[allow(dead_code)]
async fn read_bit() -> u8 {
    // TODO: реальный GPIO bit-bang
    // Pull low 6µs, release, wait 9µs, read, wait 55µs
    0
}

/// Проверить CRC ключа iButton (DS1990A: CRC-8 из первых 7 байт)
fn check_crc(key: &[u8; IBUTTON_LEN]) -> bool {
    // CRC-8 с полиномом x^8 + x^5 + x^4 + 1 (= 0x31)
    let mut crc: u8 = 0;
    for &byte in &key[..7] {
        crc ^= byte;
        for _ in 0..8 {
            if crc & 0x01 != 0 {
                crc = (crc >> 1) ^ 0x8C; // 0x8C = reverse of 0x31
            } else {
                crc >>= 1;
            }
        }
    }
    crc == key[7]
}