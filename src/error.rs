//! Набор ошибок автомата — ErrorSet через bitflags

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, Default)]
    pub struct Errors: u64 {
        const NONE                = 0;
        const FIRMWARE            = 1 << 0;
        const SETTINGS_VERSION    = 1 << 1;
        const SETTINGS_CRC       = 1 << 2;
        const STATE_VERSION       = 1 << 3;
        const STATE_CRC           = 1 << 4;
        const BATTERY_LOW         = 1 << 5;
        const FLASH_CORRUPTED     = 1 << 6;
        const NVRAM_CORRUPTED     = 1 << 7;
        const GSM_MODULE          = 1 << 8;
        const SIM_CARD            = 1 << 9;
        const COIN_ACCEPTOR       = 1 << 10;
        const ITEM_DISPENSER      = 1 << 11;
        const COIN_HOPPER         = 1 << 12;
        const COIN_ACCEPTOR_OFF   = 1 << 13;
        const ITEM_DISPENSER_EMPTY = 1 << 14;
        const COIN_HOPPER_EMPTY   = 1 << 15;
        const CANNOT_PAYOUT       = 1 << 16;
        const DOOR_OPENED         = 1 << 17;
    }
}

// Ручной impl defmt::Format для Errors (bitflags InternalBitFlags не поддерживает derive)
impl defmt::Format for Errors {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Errors({=u64:X})", self.bits())
    }
}

// ── Ошибки диспенсера ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Default, defmt::Format)]
pub enum DispenserError {
    #[default]
    None = 0,
    MotorCoil = 1,
    OpticSensor = 2,
    Jam = 3,
    Unknown = 99,
}

impl DispenserError {
    pub fn from_code(code: u8) -> Self {
        match code {
            0 => DispenserError::None,
            1 => DispenserError::MotorCoil,
            2 => DispenserError::OpticSensor,
            3 => DispenserError::Jam,
            _ => DispenserError::Unknown,
        }
    }
}

// ── Ошибки NVRAM ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, defmt::Format)]
pub enum NvramError {
    WriteFailed,
    ReadFailed,
    CrcMismatch,
    I2cError,
}

// ── Ошибки Flash ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, defmt::Format)]
pub enum FlashError {
    EraseFailed,
    WriteFailed,
    ReadFailed,
    CrcMismatch,
    Locked,
}

// ── Ошибки GSM ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, defmt::Format)]
pub enum GsmError {
    Timeout,
    NoResponse,
    CmeError(u16),
    CmsError(u16),
    UartError,
    CmuxError,
    PppError,
}

// ── Ошибки PPP ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, defmt::Format)]
pub enum PppError {
    LcpTimeout,
    PapAuthFailed,
    IpcpTimeout,
    NotConnected,
    WriteFailed,
}