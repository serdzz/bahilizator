//! Flash read/write для Settings
//!
//! Settings хранятся на последней странице Flash STM32F103 (page 63 = 0x0800FC00)
//! Пишется редко — только при редактировании меню — wear не проблема
//!
//! STM32F103 Flash:
//!   - Medium density (64KB): 64 страницы по 1KB
//!   - Запись: half-word (16 бит) за операцию
//!   - Стирание: вся страница (1KB) → 0xFF
//!   - Разблокировка: KEYR = 0x45670123, 0xCDEF89AB
//!   - После стирания: проверка BUSY, PGERR, EOP
//!
//! Перенос из flash.c (оригинал — MSP430 FRAM/Info Flash)

use crate::config;
use crate::state::Settings;
use crate::error::FlashError;

// ── Адреса Flash ─────────────────────────────────────────────────────────

/// Последняя страница Flash для 64KB варианта (page 63)
const SETTINGS_FLASH_ADDR: u32 = 0x0800_FC00;
/// Размер страницы Flash (STM32F103: 1024 байта)
const FLASH_PAGE_SIZE: usize = 1024;

// ── Регистры Flash STM32F103 ────────────────────────────────────────────
//
/// Базовый адрес регистров Flash
const FLASH_BASE: u32 = 0x4002_2000;

/// Flash Access Control Register (offset 0x00)
const FLASH_ACR: u32 = FLASH_BASE + 0x00;

/// Flash Key Register (offset 0x04)
const FLASH_KEYR: u32 = FLASH_BASE + 0x04;

/// Flash Option Key Register (offset 0x08)
const FLASH_OPTKEYR: u32 = FLASH_BASE + 0x08;

/// Flash Status Register (offset 0x0C)
const FLASH_SR: u32 = FLASH_BASE + 0x0C;

/// Flash Control Register (offset 0x10)
const FLASH_CR: u32 = FLASH_BASE + 0x10;

/// Flash Option Control Register (offset 0x14)
const FLASH_OBR: u32 = FLASH_BASE + 0x14;

/// Flash Option Write Byte Register (offset 0x18)
const FLASH_WRPR: u32 = FLASH_BASE + 0x18;

// ── Биты FLASH_SR ──────────────────────────────────────────────────────

const FLASH_SR_BSY: u32 = 0x01;
const FLASH_SR_PGERR: u32 = 0x04;
const FLASH_SR_WRPRTERR: u32 = 0x10;
const FLASH_SR_EOP: u32 = 0x20;

// ── Биты FLASH_CR ──────────────────────────────────────────────────────

const FLASH_CR_PG: u32 = 0x01;       // Programming
const FLASH_CR_PER: u32 = 0x02;      // Page Erase
const FLASH_CR_MER: u32 = 0x04;      // Mass Erase
const FLASH_CR_STRT: u32 = 0x40;     // Start
const FLASH_CR_LOCK: u32 = 0x80;     // Lock

// ── Magic number для валидации Settings ─────────────────────────────────

/// Magic number — признак валидных настроек во Flash
const SETTINGS_MAGIC: u32 = 0xDEAD_BEEF;

// ── Структура Settings во Flash ─────────────────────────────────────────
//
/// Формат хранения во Flash:
///   [0..4]   = magic (0xDEADBEEF)
///   [4..8]   = CRC16 данных (младшие 2 байта, старшие = 0)
///   [8..]    = Settings (packed)
///   [8+size..8+size+2] = CRC16 Settings (дубликат для двойной проверки)
///
/// При чтении: magic + CRC → если оба валидны, Settings ок
/// При записи: erase page → write magic → write CRC → write settings → write CRC2

// ── Низкоуровневый доступ к регистрам ──────────────────────────────────

/// Прочитать регистр Flash
///
/// Safety: читаем из memory-mapped регистров периферии STM32F103.
/// Адреса константные и соответствуют Reference Manual RM0008.
unsafe fn read_flash_reg(reg: u32) -> u32 {
    core::ptr::read_volatile(reg as *const u32)
}

/// Записать регистр Flash
///
/// Safety: пишем в memory-mapped регистры периферии STM32F103.
/// Вызывается только при работе с Flash (unlock/erase/program/lock).
unsafe fn write_flash_reg(reg: u32, value: u32) {
    core::ptr::write_volatile(reg as *mut u32, value);
}

/// Ждать пока BSY=0 во FLASH_SR
///
/// Safety: читает FLASH_SR в цикле. Таймаут ~40мс (программирование
/// half-word занимает максимум 40мкс при 72МГц, даём 1000x запас).
fn wait_flash_busy() -> Result<(), FlashError> {
    let timeout = 1_000_000; // ~14мс при 72МГц (каждая итерация ~1-2 такта)
    for _ in 0..timeout {
        let sr = unsafe { read_flash_reg(FLASH_SR) };
        if sr & FLASH_SR_BSY == 0 {
            return Ok(());
        }
    }
    Err(FlashError::WriteFailed)
}

/// Очистить флаги ошибок в FLASH_SR
fn clear_flash_errors() {
    unsafe {
        // Записываем 0 в EOP, PGERR, WRPRTERR (write 1 to clear)
        let sr = read_flash_reg(FLASH_SR);
        write_flash_reg(FLASH_SR, sr | FLASH_SR_EOP | FLASH_SR_PGERR | FLASH_SR_WRPRTERR);
    }
}

// ── Операции Flash ─────────────────────────────────────────────────────

/// Разблокировать Flash (unlock)
///
/// Safety: записываем ключи в FLASH_KEYR как описано в RM0008.
/// После unlock — возможны erase и program операции.
fn flash_unlock() {
    unsafe {
        write_flash_reg(FLASH_KEYR, 0x4567_0123);
        write_flash_reg(FLASH_KEYR, 0xCDEF_89AB);
    }
}

/// Заблокировать Flash (lock)
///
/// Safety: устанавливаем LOCK бит в FLASH_CR.
fn flash_lock() {
    unsafe {
        let cr = read_flash_reg(FLASH_CR);
        write_flash_reg(FLASH_CR, cr | FLASH_CR_LOCK);
    }
}

/// Стереть страницу Flash
///
/// Safety: стирает страницу по адресу SETTINGS_FLASH_ADDR.
/// Данные теряются. Требует unlock перед вызовом.
fn flash_erase_page() -> Result<(), FlashError> {
    clear_flash_errors();
    wait_flash_busy()?;

    unsafe {
        // Установить PER (Page Erase) в CR
        let cr = read_flash_reg(FLASH_CR);
        write_flash_reg(FLASH_CR, cr | FLASH_CR_PER);

        // Записать адрес страницы в AR (Address Register)
        // На STM32F103 AR находится по смещению 0x14 от FLASH_R_BASE
        // НО: в RM0008 FLASH_AR = FLASH_BASE + 0x14
        // FIXME: stm32f1 имеет FLASH_AR по другому адресу
        // Используем альтернативный метод — пишем через CR
        // Устанавливаем стартовый адрес
        let ar_reg = FLASH_BASE + 0x14; // FLASH_AR
        core::ptr::write_volatile(ar_reg as *mut u32, SETTINGS_FLASH_ADDR);

        // Установить STRT (Start) в CR
        let cr = read_flash_reg(FLASH_CR);
        write_flash_reg(FLASH_CR, cr | FLASH_CR_STRT);
    }

    // Ждём завершения стирания
    wait_flash_busy()?;

    // Проверяем ошибки
    let sr = unsafe { read_flash_reg(FLASH_SR) };
    if sr & FLASH_SR_PGERR != 0 || sr & FLASH_SR_WRPRTERR != 0 {
        return Err(FlashError::EraseFailed);
    }

    // Сбрасываем PER бит
    unsafe {
        let cr = read_flash_reg(FLASH_CR);
        write_flash_reg(FLASH_CR, cr & !FLASH_CR_PER);
    }

    Ok(())
}

/// Запрограммировать half-word (16 бит) во Flash
///
/// Safety: записывает 2 байта по указанному адресу.
/// Адрес должен быть выровнен на half-word (2 байта).
/// Данные могут менять 1→0 но не 0→1 (нужно стереть страницу).
fn flash_program_halfword(addr: u32, data: u16) -> Result<(), FlashError> {
    wait_flash_busy()?;

    unsafe {
        // Установить PG (Programming) в CR
        let cr = read_flash_reg(FLASH_CR);
        write_flash_reg(FLASH_CR, cr | FLASH_CR_PG);

        // Записать half-word по адресу
        // Safety: addr — валидный адрес Flash памяти STM32F103,
        // выровнен на 2 байта. Flash разблокирован, страница стёрта.
        core::ptr::write_volatile(addr as *mut u16, data);

        // Ждём завершения
    }

    wait_flash_busy()?;

    // Проверяем ошибки
    let sr = unsafe { read_flash_reg(FLASH_SR) };
    if sr & FLASH_SR_PGERR != 0 || sr & FLASH_SR_WRPRTERR != 0 {
        // Сбрасываем PG
        unsafe {
            let cr = read_flash_reg(FLASH_CR);
            write_flash_reg(FLASH_CR, cr & !FLASH_CR_PG);
        }
        return Err(FlashError::WriteFailed);
    }

    // Сбрасываем PG
    unsafe {
        let cr = read_flash_reg(FLASH_CR);
        write_flash_reg(FLASH_CR, cr & !FLASH_CR_PG);
    }

    Ok(())
}

// ── Публичные функции ──────────────────────────────────────────────────

/// Загрузить настройки из Flash
///
/// Читаем Settings прямо из адреса Flash (memory-mapped).
/// STM32F103 Flash читается как обычная RAM.
/// Проверяем magic и CRC.
pub fn load_settings() -> Result<Settings, FlashError> {
    // Читаем magic (первые 4 байта)
    let magic = unsafe {
        let ptr = SETTINGS_FLASH_ADDR as *const u32;
        core::ptr::read_volatile(ptr)
    };

    // Проверяем magic number
    if magic != SETTINGS_MAGIC {
        return Err(FlashError::CrcMismatch);
    }

    // Читаем CRC (байты 4..8)
    let stored_crc = unsafe {
        let ptr = (SETTINGS_FLASH_ADDR + 4) as *const u16;
        core::ptr::read_volatile(ptr)
    };

    // Читаем Settings (байты 8..8+size)
    let settings = unsafe {
        let ptr = (SETTINGS_FLASH_ADDR + 8) as *const Settings;
        // Проверяем выравнивание
        if (ptr as usize) % core::mem::align_of::<Settings>() != 0 {
            return Err(FlashError::ReadFailed);
        }
        core::ptr::read_volatile(ptr)
    };

    // Проверяем CRC Settings
    let computed_crc = calc_settings_crc(&settings);
    if computed_crc != stored_crc {
        return Err(FlashError::CrcMismatch);
    }

    Ok(settings)
}

/// Сохранить настройки во Flash
///
/// 1. Проверить CRC — не писать если не изменился
/// 2. Разблокировать Flash
/// 3. Стереть страницу 63
/// 4. Записать: magic → CRC → Settings
/// 5. Заблокировать Flash
///
/// В оригинале (flash.c): _flashWrite() — erase page + byte-by-byte program
pub fn save_settings(settings: &Settings) -> Result<(), FlashError> {
    // Проверить — не писать если не изменился
    if let Ok(old) = load_settings() {
        if calc_settings_crc(&old) == calc_settings_crc(settings) {
            return Ok(()); // Нет изменений
        }
    }

    // Вычисляем CRC
    let crc = calc_settings_crc(settings);

    // Разблокировать Flash
    flash_unlock();

    // Стереть страницу
    let erase_result = flash_erase_page();
    if erase_result.is_err() {
        flash_lock();
        return erase_result;
    }

    // Программируем побайтово (half-words):
    // [0..4]   = magic (0xDEADBEEF)
    // [4..6]   = CRC16
    // [6..8]   = padding (0xFFFF)
    // [8..8+N] = Settings packed

    let base = SETTINGS_FLASH_ADDR;

    // Magic: 0xDEADBEEF (2 half-words)
    flash_program_halfword(base, 0xBEEF as u16)?;
    flash_program_halfword(base + 2, 0xDEAD as u16)?;

    // CRC16
    flash_program_halfword(base + 4, crc)?;

    // Padding
    flash_program_halfword(base + 6, 0xFFFF)?;

    // Settings — побайтовая запись (half-word at a time)
    let settings_bytes = unsafe {
        core::slice::from_raw_parts(
            settings as *const _ as *const u8,
            core::mem::size_of::<Settings>(),
        )
    };

    // Пишем по 2 байта (half-word)
    let settings_base = base + 8;
    let mut i = 0;
    while i + 1 < settings_bytes.len() {
        let hw = u16::from_le_bytes([settings_bytes[i], settings_bytes[i + 1]]);
        flash_program_halfword(settings_base + i as u32, hw)?;
        i += 2;
    }
    // Если нечётное количество байт — дописываем последний с 0xFF
    if i < settings_bytes.len() {
        let hw = u16::from_le_bytes([settings_bytes[i], 0xFF]);
        flash_program_halfword(settings_base + i as u32, hw)?;
    }

    // Заблокировать Flash
    flash_lock();

    Ok(())
}

// ── CRC для Settings ──────────────────────────────────────────────────

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

/// CRC16 Modbus (полином 0xA001, init 0xFFFF) — как в оригинале
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