//! 1-Wire iButton драйвер — на базе crate `one-wire-bus`
//!
//! DS1990A: Reset pulse → Read ROM (0x33) → 8 байт (family+serial+CRC)
//!
//! ESP32 DevKit V1: GPIO4 для 1-Wire шины
//! esp-hal::gpio::OutputOpenDrain реализует InputPin + OutputPin
//! из embedded-hal 0.2 — подходит для one-wire-bus.
//!
//! Для ESP32 используем busy-wait delay вместо cortex_m::asm::delay().

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Sender;

use crate::state::{Settings, VendingState, IBUTTON_LEN};

use one_wire_bus::{Address, OneWire};

// ── 1-Wire команды ────────────────────────────────────────────────────────

/// Read ROM — чтение 64-битного адреса единственного устройства на шине
const CMD_READ_ROM: u8 = 0x33;

// ── IbuttonEvent ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, defmt::Format)]
pub struct IbuttonEvent {
    /// 8-байтовый ключ iButton
    pub key: [u8; IBUTTON_LEN],
    /// Ключ принят (true) или отклонён (false)
    pub accepted: bool,
}

// ── EspDelay — busy-wait для 1-Wire на ESP32 (240 MHz) ────────────────────

pub struct EspDelay {
    clock_mhz: u32,
}

impl EspDelay {
    pub fn new() -> Self {
        Self { clock_mhz: 240 }
    }
}

impl Default for EspDelay {
    fn default() -> Self {
        Self::new()
    }
}

impl embedded_hal::blocking::delay::DelayUs<u16> for EspDelay {
    fn delay_us(&mut self, us: u16) {
        let cycles_per_us = self.clock_mhz / 3;
        let total_cycles = us as u32 * cycles_per_us;
        for _ in 0..total_cycles {
            core::hint::spin_loop();
        }
    }
}

// ── EspHalPin — заглушка GPIO пина для one-wire-bus ──────────────────────
//
// TODO: Заменить на esp_hal::gpio::OutputOpenDrain<'static> из main.rs.
// esp-hal OutputOpenDrain реализует embedded-hal 0.2 InputPin + OutputPin.
// Сейчас — заглушка для компиляции.

pub struct EspHalPin;

impl embedded_hal::digital::v2::OutputPin for EspHalPin {
    type Error = core::convert::Infallible;

    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl embedded_hal::digital::v2::InputPin for EspHalPin {
    type Error = core::convert::Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(true)
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(false)
    }
}

// ── IbuttonDriver ────────────────────────────────────────────────────────

/// Драйвер 1-Wire шины для чтения iButton ключей DS1990A.
pub struct IbuttonDriver {
    bus: OneWire<EspHalPin>,
    delay: EspDelay,
}

impl IbuttonDriver {
    /// Создать драйвер. В реальном коде pin = esp_hal OutputOpenDrain.
    /// Сейчас — заглушка, используем EspHalPin.
    pub fn new(pin: EspHalPin) -> Self {
        let bus = OneWire::new(pin).unwrap_or_else(|_| {
            // TODO: реальный пин всегда инициализируется успешно
            panic!("iButton: OneWire init failed");
        });
        Self {
            bus,
            delay: EspDelay::new(),
        }
    }

    /// Отправить reset pulse и проверить presence
    pub fn reset(&mut self) -> bool {
        self.bus.reset(&mut self.delay).unwrap_or(false)
    }

    /// Чтение ключа DS1990A: Reset → Read ROM (0x33) → 8 байт → CRC check
    pub fn read_rom(&mut self) -> Option<[u8; IBUTTON_LEN]> {
        if !self.reset() {
            return None;
        }

        self.bus.write_byte(CMD_READ_ROM, &mut self.delay).ok()?;

        let mut key = [0u8; IBUTTON_LEN];
        self.bus.read_bytes(&mut key, &mut self.delay).ok()?;

        if one_wire_bus::crc::crc8(&key) == 0 {
            Some(key)
        } else {
            defmt::trace!("iButton CRC mismatch");
            None
        }
    }

    /// Поиск всех устройств на шине (SEARCH ROM = 0xF0)
    pub fn find_devices(&mut self) -> heapless::Vec<Address, 4> {
        let mut found = heapless::Vec::new();
        for result in self.bus.devices(false, &mut self.delay) {
            match result {
                Ok(addr) => {
                    found.push(addr).ok();
                }
                Err(_) => break,
            }
        }
        found
    }
}

// ── Проверка ключа по whitelist ──────────────────────────────────────────

#[derive(Debug, Clone, Copy, defmt::Format, PartialEq)]
pub enum KeyAccess {
    Service = 0,
    Technical = 1,
}

pub fn check_key(settings: &Settings, key: &[u8; IBUTTON_LEN]) -> Option<KeyAccess> {
    if key.iter().all(|&b| b == 0) {
        return None;
    }

    for level in 0..settings.keys.len() {
        for slot in 0..settings.keys[level].len() {
            let stored = &settings.keys[level][slot];
            if !stored.iter().all(|&b| b == 0) && stored == key {
                return match level {
                    0 => Some(KeyAccess::Service),
                    _ => Some(KeyAccess::Technical),
                };
            }
        }
    }

    None
}

// ── Утилиты ──────────────────────────────────────────────────────────────

pub fn address_to_bytes(addr: &Address) -> [u8; IBUTTON_LEN] {
    addr.0.to_le_bytes()
}

pub fn compute_crc(data: &[u8]) -> u8 {
    one_wire_bus::crc::crc8(data)
}

pub fn check_crc(key: &[u8; IBUTTON_LEN]) -> bool {
    one_wire_bus::crc::crc8(key) == 0
}

// ── Задача iButton ────────────────────────────────────────────────────────

pub async fn run(
    driver: Option<IbuttonDriver>,
    ibutton_tx: Sender<'static, CriticalSectionRawMutex, IbuttonEvent, 1>,
    state: &'static embassy_sync::mutex::Mutex<
        CriticalSectionRawMutex,
        core::cell::RefCell<VendingState>,
    >,
) {
    let mut driver = match driver {
        Some(d) => d,
        None => {
            defmt::warn!("iButton: драйвер не инициализирован (нет пина GPIO4)");
            loop {
                embassy_time::Timer::after_secs(5).await;
            }
        }
    };

    loop {
        if let Some(key) = driver.read_rom() {
            let accepted = {
                let guard = state.lock().await;
                let s = guard.borrow();
                check_key(&s.settings, &key).is_some()
            };

            let _ = ibutton_tx.try_send(IbuttonEvent { key, accepted });
            defmt::debug!("iButton: ключ обнаружен, accepted={}", accepted);
        }

        embassy_time::Timer::after_millis(200).await;
    }
}
