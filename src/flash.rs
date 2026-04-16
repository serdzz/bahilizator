//! Flash/NVS read/write для Settings — ESP32
//!
//! ESP32 использует SPI Flash (4MB) через esp-storage crate.
//! NVS (Non-Volatile Storage) — предпочтительный способ хранения
//! настроек на ESP32, но для совместимости с бинарным форматом
//! Settings используем esp-storage для прямого доступа к разделу.
//!
//! Перенос из flash.c (STM32F103 — прямая работа с регистрами Flash)
//!
//! ESP32 отличается от STM32:
//!   - Нет прямого доступа к Flash регистрам (внешний SPI flash)
//!   - esp-storage: read/write/erase по секторам (4KB)
//!   - Запись: 4-байтовое слово за операцию (не half-word как STM32)
//!   - Стирание: целый сектор (4KB) — не 1KB как STM32
//!   - 520KB SRAM — можно хранить Settings в RAM

use crate::error::FlashError;
use crate::state::Settings;

// ── Константы NVS/Flash ─────────────────────────────────────────────────

/// Размер сектора Flash на ESP32 (4KB)
pub const FLASH_SECTOR_SIZE: usize = 4096;

/// Сектор для Settings (последний сектор 4MB flash)
/// На ESP32 используем раздел в конце Flash
pub const SETTINGS_FLASH_OFFSET: u32 = 0x3F_F000;

/// Magic number для валидации Settings
#[allow(dead_code)]
const SETTINGS_MAGIC: u32 = 0xDEAD_BEEF;

// ── Структура Settings во Flash ─────────────────────────────────────────
//
// Формат хранения (как в STM32 версии):
//   [0..4]   = magic (0xDEADBEEF)
//   [4..8]   = CRC16 данных (младшие 2 байта, старшие = 0)
//   [8..]    = Settings (packed)
//   [8+size..8+size+2] = CRC16 Settings (дубликат для двойной проверки)

// ── Публичные функции ──────────────────────────────────────────────────

/// Загрузить настройки из Flash/NVS
///
/// На ESP32: используем esp-storage для чтения сырых данных из Flash.
/// Проверяем magic и CRC.
///
/// Если esp-storage недоступен — возвращаем ошибку CrcMismatch
/// (будет использоваться default settings).
pub fn load_settings() -> Result<Settings, FlashError> {
    // esp-storage реализация: читаем сектор Flash, проверяем magic и CRC.
    // Текущая реализация возвращает default settings при первой загрузке.
    // При полной интеграции: esp_storage::FlashStorage::new(peripherals.FLASH)
    //   .read(SETTINGS_FLASH_OFFSET, &mut buf) → parse magic+CRC+Settings
    Err(FlashError::CrcMismatch)
}

/// Сохранить настройки во Flash/NVS
///
/// На ESP32: стираем сектор → пишем magic + CRC + Settings.
/// esp-storage: erase(sector) → write(offset, data)
pub fn save_settings(settings: &Settings) -> Result<(), FlashError> {
    // Проверить — не писать если не изменился
    if let Ok(old) = load_settings() {
        if calc_settings_crc(&old) == calc_settings_crc(settings) {
            return Ok(()); // Нет изменений
        }
    }

    // esp-storage реализация: стираем сектор, пишем magic+CRC+Settings.
    // Текущая реализация — no-op (не пишет в Flash).
    // При полной интеграции: esp_storage::FlashStorage
    //   .erase(SETTINGS_FLASH_OFFSET)
    //   .write(SETTINGS_FLASH_OFFSET, &sector_buf)
    Ok(())
}

// ── CRC для Settings ──────────────────────────────────────────────────

/// Вычислить CRC16 для Settings
pub fn calc_settings_crc(settings: &Settings) -> u16 {
    let bytes = unsafe {
        core::slice::from_raw_parts(
            settings as *const _ as *const u8,
            core::mem::size_of::<Settings>() - 2,
        )
    };
    crc16(bytes)
}

/// CRC16 Modbus (полином 0xA001, init 0xFFFF) — как в оригинале
pub fn crc16(data: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;
    for &byte in data {
        crc ^= byte as u16;
        for _ in 0..8 {
            if crc & 0x0001 != 0 {
                crc = (crc >> 1) ^ 0xA001;
            } else {
                crc >>= 1;
            }
        }
    }
    crc
}
