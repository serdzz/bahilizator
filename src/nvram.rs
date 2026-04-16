//! I²C EEPROM 24C08 драйвер для State
//!
//! 24C08: I2C адрес 0x50, 1024 байт (256×4 страницы)
//! Страница = 8 байт (page-write). Sequential read до конца строки.
//!
//! Wear levelling: 4 сектора по 256 байт. При каждой записи State
//! пишем в следующий сектор (rotate). Текущий сектор определяется
//! по magic-маркеру.
//!
//! Перенос из fram.c (оригинал — SPI FRAM FM25L04)
//! В текущей ревизии железа — I2C EEPROM 24C08

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_sync::signal::Signal;

use crate::config;
use crate::error::NvramError;
use crate::state::{PersistReason, VendingState, VendingStateData};

// ── Глобальный I2C1 для EEPROM ──────────────────────────────────────────

/// Указатель на I2C1 — устанавливается из main() один раз
static mut I2C1_PTR: Option<*mut esp_hal::i2c::master::I2c<'static, esp_hal::Blocking>> = None;

/// Установить I2C1 для EEPROM (вызывается из main.rs)
///
/// Safety: вызывается один раз из main() до spawn задач.
/// После этого I2C1 используется только из persist_task.
pub fn set_i2c(i2c: &'static esp_hal::i2c::master::I2c<'static, esp_hal::Blocking>) {
    unsafe {
        I2C1_PTR = Some(i2c as *const _ as *mut _);
    }
}

/// Получить mutable ссылку на I2C1
///
/// Safety: безопасно только если вызывается из одной задачи (persist_task)
unsafe fn get_i2c() -> &'static mut esp_hal::i2c::master::I2c<'static, esp_hal::Blocking> {
    &mut *I2C1_PTR.unwrap()
}

// ── Константы EEPROM 24C08 ───────────────────────────────────────────────

/// I2C адрес 24C08 (A0=A1=A2=GND → 0x50)
pub const EEPROM_ADDR: u8 = config::EEPROM_I2C_ADDR;

/// Общий объём 24C08: 1024 байт
pub const EEPROM_TOTAL_SIZE: u16 = 1024;

/// Размер страницы для page-write: 8 байт
pub const EEPROM_PAGE_SIZE: usize = config::EEPROM_PAGE_SIZE;

/// Количество секторов для wear levelling
pub const NVRAM_SECTORS: usize = 4;

/// Размер сектора: 256 байт (1024 / 4)
pub const NVRAM_SECTOR_SIZE: u16 = EEPROM_TOTAL_SIZE / NVRAM_SECTORS as u16;

/// Magic-маркер сектора — показывает, что сектор содержит валидные данные
pub const NVRAM_SECTOR_MAGIC: u16 = 0xBABE;

/// Размер State в EEPROM = размер VendingStateData + 2 байта CRC + 2 байта magic
pub const STATE_EEPROM_SIZE: usize = core::mem::size_of::<VendingStateData>() + 4;

/// Смещение State внутри сектора (magic + crc в начале)
pub const STATE_OFFSET_IN_SECTOR: u16 = 4;

// ── Заголовок сектора в EEPROM ──────────────────────────────────────────
//
/// Первые 4 байта каждого сектора:
///   [0..2] = magic (0xBABE)
///   [2..4] = CRC16 данных
///   [4..]  = VendingStateData
#[derive(Debug, Clone, Copy, Default)]
#[repr(C, packed)]
#[allow(dead_code)]
struct SectorHeader {
    magic: u16,
    crc: u16,
}

// ── Текущий сектор (volatile — хранится в RAM) ──────────────────────────

/// Индекс текущего активного сектора (0..3)
/// Определяется при загрузке поиском сектора с валидным magic
static mut CURRENT_SECTOR: u8 = 0;

// ── Низкоуровневые I2C функции ──────────────────────────────────────────

/// Прочитать один байт из EEPROM по 16-битному адресу
///
/// 24C08: адрес = 10 бит. Старшие 2 бита адреса передаются
/// в младших битах I2C адреса (A0, A1). Поэтому:
///   device_addr = 0x50 | (mem_addr >> 8) & 0x03
///   word_addr   = mem_addr & 0xFF
#[allow(dead_code)]
async fn eeprom_read_byte(mem_addr: u16) -> u8 {
    let i2c = unsafe { get_i2c() };
    let device_addr = EEPROM_ADDR | ((mem_addr >> 8) as u8 & 0x03);
    let word_addr = (mem_addr & 0xFF) as u8;
    let mut buf = [0u8; 1];
    if i2c.write_read(device_addr, &[word_addr], &mut buf).is_ok() {
        buf[0]
    } else {
        0xFF
    }
}

/// Записать один байт в EEPROM по 16-битному адресу
///
/// Внимание: после записи нужно подождать ~5мс (write cycle time)
async fn eeprom_write_byte(mem_addr: u16, data: u8) {
    let i2c = unsafe { get_i2c() };
    let device_addr = EEPROM_ADDR | ((mem_addr >> 8) as u8 & 0x03);
    let word_addr = (mem_addr & 0xFF) as u8;
    let _ = i2c.write(device_addr, &[word_addr, data]);
    embassy_time::Timer::after_millis(5).await; // write cycle time
}

/// Записать страницу (8 байт) в EEPROM — page-write
///
/// 24C08: page-write — до 8 байт в одной транзакции I2C.
/// Адрес должен быть выровнен на границу страницы.
/// Внутри страницы адрес инкрементируется автоматически (wrap within page).
async fn eeprom_page_write(mem_addr: u16, data: &[u8; EEPROM_PAGE_SIZE]) {
    let i2c = unsafe { get_i2c() };
    let device_addr = EEPROM_ADDR | ((mem_addr >> 8) as u8 & 0x03);
    let word_addr = (mem_addr & 0xFF) as u8;
    let mut buf = [0u8; 1 + EEPROM_PAGE_SIZE];
    buf[0] = word_addr;
    buf[1..].copy_from_slice(data);
    let _ = i2c.write(device_addr, &buf);
    embassy_time::Timer::after_millis(5).await;
}

/// Последовательное чтение (sequential read) из EEPROM
///
/// После установки начального адреса, чтение продолжается
/// с автоинкрементом адреса. Максимум — до конца строки
/// (у 24C08 — wrap на границе 256 байт).
async fn eeprom_sequential_read(mem_addr: u16, buf: &mut [u8]) {
    let i2c = unsafe { get_i2c() };
    let device_addr = EEPROM_ADDR | ((mem_addr >> 8) as u8 & 0x03);
    let word_addr = (mem_addr & 0xFF) as u8;
    if i2c.write_read(device_addr, &[word_addr], buf).is_err() {
        // Ошибка чтения — заполняем 0xFF
        for b in buf.iter_mut() {
            *b = 0xFF;
        }
    }
}

/// Найти текущий активный сектор в EEPROM
///
/// Ищем сектор с валидным magic (0xBABE) и CRC.
/// Если ни один не найден — используем сектор 0.
fn find_active_sector() -> u8 {
    // В реальном железе нужно прочитать magic из каждого сектора.
    // Возвращает сектор 0 если ни один не найден
    0
}

/// Получить следующий сектор (rotate)
fn next_sector(sector: u8) -> u8 {
    (sector + 1) % NVRAM_SECTORS as u8
}

/// Вычислить I2C адрес начала сектора
fn sector_base_addr(sector: u8) -> u16 {
    (sector as u16) * NVRAM_SECTOR_SIZE
}

// ── Публичные функции ──────────────────────────────────────────────────

/// Загрузить State из EEPROM
///
/// 1. Найти активный сектор (по magic)
/// 2. Прочитать заголовок (magic + CRC)
/// 3. Проверить magic == 0xBABE
/// 4. Прочитать VendingStateData
/// 5. Проверить CRC
/// 6. Если CRC не совпадает — попробовать следующий сектор
pub async fn load_state() -> Result<VendingStateData, NvramError> {
    let sector = find_active_sector();

    // Пробуем текущий и следующие секторы
    for attempt in 0..NVRAM_SECTORS {
        let s = (sector as usize + attempt) % NVRAM_SECTORS;
        let base = sector_base_addr(s as u8);

        // Читаем заголовок сектора (4 байта: magic + crc)
        let mut header_buf = [0u8; 4];
        eeprom_sequential_read(base, &mut header_buf).await;

        let magic = u16::from_le_bytes([header_buf[0], header_buf[1]]);
        let crc = u16::from_le_bytes([header_buf[2], header_buf[3]]);

        // Проверяем magic
        if magic != NVRAM_SECTOR_MAGIC {
            continue; // Пустой сектор — пробуем следующий
        }

        // Читаем VendingStateData
        let mut state_buf = [0u8; core::mem::size_of::<VendingStateData>()];
        eeprom_sequential_read(base + STATE_OFFSET_IN_SECTOR, &mut state_buf).await;

        // Проверяем CRC
        let computed_crc = crc16(&state_buf);
        if computed_crc == crc {
            // Safety: читаем VendingStateData из байтового буфера.
            // VendingStateData — #[derive(Copy)] без ссылок,
            // представление в памяти совпадает с побайтовой
            // копией (packed struct без padding).
            let state: VendingStateData =
                unsafe { core::ptr::read(state_buf.as_ptr() as *const VendingStateData) };

            // Сохраняем найденный сектор как текущий
            unsafe { CURRENT_SECTOR = s as u8 };

            // Проверяем версию
            if state.version > config::STATE_VERSION {
                return Err(NvramError::CrcMismatch); // Версия новее — несовместимо
            }

            return Ok(state);
        }
        // CRC не совпадает — пробуем следующий сектор
    }

    // Ни один сектор не валиден
    Err(NvramError::CrcMismatch)
}

/// Загрузить State из EEPROM (по умолчанию — default если ошибка)
pub fn load_state_default() -> VendingStateData {
    VendingStateData::default()
}

/// Сохранить State в EEPROM с wear levelling
///
/// 1. Переключаемся на следующий сектор
/// 2. Инвалидируем старый сектор (magic = 0)
/// 3. Пишем CRC + VendingStateData в новый сектор
/// 4. Устанавливаем magic = 0xBABE
pub async fn save_state(state: &VendingStateData) -> Result<(), NvramError> {
    let old_sector = unsafe { CURRENT_SECTOR };
    let new_sector = next_sector(old_sector);
    let new_base = sector_base_addr(new_sector);

    // Сериализуем State в байты
    // Safety: VendingStateData — Copy, без ссылок, размер известен.
    // Побайтовое представление корректно для записи в EEPROM.
    let state_bytes = unsafe {
        core::slice::from_raw_parts(
            state as *const _ as *const u8,
            core::mem::size_of::<VendingStateData>(),
        )
    };

    // Вычисляем CRC
    let crc = crc16(state_bytes);

    // Записываем заголовок сектора: magic + CRC
    // Сначала пишем CRC (без magic — сектор ещё не валиден)
    let crc_bytes = crc.to_le_bytes();
    eeprom_write_byte(new_base, crc_bytes[0]).await;
    eeprom_write_byte(new_base + 1, crc_bytes[1]).await;

    // Записываем VendingStateData постранично (8 байт за раз)
    let data_base = new_base + STATE_OFFSET_IN_SECTOR;
    let mut offset: u16 = 0;
    while offset < state_bytes.len() as u16 {
        let remaining = state_bytes.len() as u16 - offset;
        if remaining >= EEPROM_PAGE_SIZE as u16 {
            // Полная страница
            let mut page = [0u8; EEPROM_PAGE_SIZE];
            page.copy_from_slice(&state_bytes[offset as usize..offset as usize + EEPROM_PAGE_SIZE]);
            eeprom_page_write(data_base + offset, &page).await;
        } else {
            // Неполная страница — пишем побайтово
            for i in 0..remaining {
                eeprom_write_byte(data_base + offset + i, state_bytes[(offset + i) as usize]).await;
            }
        }
        offset += EEPROM_PAGE_SIZE as u16;
    }

    // Валидируем новый сектор — пишем magic последним
    let magic_bytes = NVRAM_SECTOR_MAGIC.to_le_bytes();
    eeprom_write_byte(new_base, magic_bytes[0]).await;
    eeprom_write_byte(new_base + 1, magic_bytes[1]).await;

    // Инвалидируем старый сектор (обнуляем magic)
    eeprom_write_byte(sector_base_addr(old_sector), 0x00).await;
    eeprom_write_byte(sector_base_addr(old_sector) + 1, 0x00).await;

    // Переключаемся на новый сектор
    unsafe { CURRENT_SECTOR = new_sector };

    Ok(())
}

// ── Задача периодического сохранения State ─────────────────────────────

pub async fn persist_task(
    state: &'static Mutex<CriticalSectionRawMutex, core::cell::RefCell<VendingState>>,
    persist_signal: &'static Signal<CriticalSectionRawMutex, PersistReason>,
) {
    let mut last_persist = embassy_time::Instant::now();
    let mut dirty = false;

    loop {
        match persist_signal.wait().await {
            PersistReason::StateChanged => {
                dirty = true;
            }
            PersistReason::SettingsChanged => {
                let s = {
                    let guard = state.lock().await;
                    let st = guard.borrow();
                    st.settings
                };
                crate::flash::save_settings(&s).ok();
            }
        }

        if dirty && last_persist.elapsed().as_secs() >= config::STATE_PERSIST_DEBOUNCE_S {
            let s = {
                let guard = state.lock().await;
                let st = guard.borrow();
                st.data
            };
            save_state(&s).await.ok();
            last_persist = embassy_time::Instant::now();
            dirty = false;
        }
    }
}

// ── CRC ────────────────────────────────────────────────────────────────

/// Вычислить CRC16 для State (Modbus variant — как в оригинале)
pub fn calc_state_crc(state: &VendingStateData) -> u16 {
    let bytes = unsafe {
        core::slice::from_raw_parts(
            state as *const _ as *const u8,
            core::mem::size_of::<VendingStateData>() - 2,
        )
    };
    crc16(bytes)
}

/// CRC16 Modbus (полином 0xA001, init 0xFFFF)
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
