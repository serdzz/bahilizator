//! I²C EEPROM драйвер для State (24C08)
//!
//! Внешний EEPROM на том же I2C bus, что и дисплей
//! Wear-safe: пишем только если CRC изменился

use embassy_sync::mutex::Mutex;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;

use crate::config;
use crate::error::NvramError;
use crate::state::{PersistReason, VendingState, VendingStateData};

// ── Константы EEPROM ─────────────────────────────────────────────────────

pub const EEPROM_ADDR: u8 = config::EEPROM_I2C_ADDR;
pub const STATE_EEPROM_OFFSET: u16 = 0x00;
pub const STATE_SIZE: usize = core::mem::size_of::<VendingStateData>();
pub const STATE_PAGE_SIZE: usize = config::EEPROM_PAGE_SIZE;

// ── Публичные функции ─────────────────────────────────────────────────────

/// Загрузить State из EEPROM (по умолчанию — default)
pub fn load_state_default() -> VendingStateData {
    VendingStateData::default()
}

/// Сохранить State в EEPROM (page-by-page, с проверкой CRC)
pub async fn save_state(_state: &VendingStateData) -> Result<(), NvramError> {
    // TODO: реальная запись через I2C
    Ok(())
}

/// Загрузить State из EEPROM через I2C
pub async fn load_state() -> Result<VendingStateData, NvramError> {
    // TODO: реальное чтение из I2C EEPROM
    Ok(VendingStateData::default())
}

// ── Задача периодического сохранения State ───────────────────────────────

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
                st.data.clone()
            };
            save_state(&s).await.ok();
            last_persist = embassy_time::Instant::now();
            dirty = false;
        }
    }
}

// ── CRC для State ────────────────────────────────────────────────────────

pub fn calc_state_crc(state: &VendingStateData) -> u16 {
    let bytes = unsafe {
        core::slice::from_raw_parts(
            state as *const _ as *const u8,
            core::mem::size_of::<VendingStateData>() - 2,
        )
    };
    crc16(bytes)
}

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