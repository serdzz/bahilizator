//! Flash read/write для Settings
//!
//! Settings хранятся на последней странице Flash STM32F103 (page 63 = 0x0800FC00)
//! Пишется редко — только при редактировании меню — wear не проблема

use crate::state::Settings;
use crate::error::FlashError;

// ── Адреса Flash ─────────────────────────────────────────────────────────

/// Последняя страница Flash для 64KB варианта
const SETTINGS_FLASH_ADDR: u32 = 0x0800_FC00;
/// Размер страницы Flash (STM32F103: 1024 байта)
const FLASH_PAGE_SIZE: usize = 1024;

// ── Публичные функции ─────────────────────────────────────────────────────

/// Загрузить настройки из Flash
/// Если CRC не совпадает — вернуть ошибку (вызывающий код подставит defaults)
pub fn load_settings() -> Result<Settings, FlashError> {
    // Читаем Settings прямо из адреса Flash (memory-mapped)
    // STM32F103 Flash читается как обычная RAM
    let settings = unsafe {
        let ptr = SETTINGS_FLASH_ADDR as *const Settings;
        // Проверяем что указатель выровнен
        if (ptr as usize) % core::mem::align_of::<Settings>() != 0 {
            return Err(FlashError::ReadFailed);
        }
        core::ptr::read(ptr)
    };

    // Проверить CRC
    if calc_settings_crc(&settings) != settings.crc {
        return Err(FlashError::CrcMismatch);
    }

    Ok(settings)
}

/// Сохранить настройки во Flash
/// Если настройки не изменились (CRC совпадает) — не пишем
pub fn save_settings(settings: &Settings) -> Result<(), FlashError> {
    // Проверить CRC — не писать если не изменился
    if let Ok(old) = load_settings() {
        if calc_settings_crc(&old) == calc_settings_crc(settings) {
            return Ok(()); // Нет изменений
        }
    }

    // TODO: реальная запись во Flash
    // 1. Разблокировать Flash (KEYR = 0x45670123, 0xCDEF89AB)
    // 2. Стереть страницу 63 (PER=1, FLASH_CR)
    // 3. Записать settings побайтово (PG=1)
    // 4. Заблокировать Flash
    // Это требует доступа к регистрам STM32F103 через embassy-stm32 или pac

    Ok(())
}

// ── CRC для Settings ─────────────────────────────────────────────────────

/// Вычислить CRC16 для Settings (все поля кроме самого crc)
pub fn calc_settings_crc(settings: &Settings) -> u16 {
    let bytes = unsafe {
        core::slice::from_raw_parts(
            settings as *const _ as *const u8,
            core::mem::size_of::<Settings>() - 2, // -2 для crc поля
        )
    };
    crc16(bytes)
}

/// Вычислить CRC16 (Modbus variant)
fn crc16(data: &[u8]) -> u16 {
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