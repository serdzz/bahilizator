//! GSM 07.10 CMUX протокол — frame parser/builder
//!
//! Мультиплексирование нескольких DLCI поверх одного UART
//! Basic mode — GSM 07.10

use heapless::Vec;

// ── Константы CMUX ───────────────────────────────────────────────────────

pub const CMUX_FLAG: u8 = 0xF9;

// Типы кадров (control field)
pub const SABM: u8 = 0x2F;   // Set Asynchronous Balanced Mode
pub const UA: u8 = 0x63;     // Unnumbered Acknowledge
pub const DM: u8 = 0x0F;    // Disconnected Mode
pub const DISC: u8 = 0x43;  // Disconnect
pub const UIH: u8 = 0xEF;   // Unnumbered Information with Header check
pub const UI: u8 = 0x03;    // Unnumbered Information

// Управление мультиплексором (MSC)
pub const MSC_SET: u8 = 0xFD;
pub const MSC_ACK: u8 = 0x73;

// ── CmuxFrame — распарсенный кадр CMUX ───────────────────────────────────

#[derive(Debug)]
pub struct CmuxFrame {
    /// DLCI канала (0 = управление, 1 = AT, 2 = PPP)
    pub dlci: u8,
    /// Поле управления (SABM, UA, UIH, ...)
    pub control: u8,
    /// Данные кадра
    pub data: Vec<u8, 256>,
}

// ── Состояния декодера ────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
enum DecoderState {
    Idle,
    InFrame,
    Escape,
}

// ── CmuxDecoder — state machine для парсинга входящих кадров ─────────────

pub struct CmuxDecoder {
    state: DecoderState,
    current: Vec<u8, 300>,
}

impl CmuxDecoder {
    /// Создать новый декодер
    pub fn new() -> Self {
        Self {
            state: DecoderState::Idle,
            current: Vec::new(),
        }
    }

    /// Покормить декодер одним байтом из UART
    pub fn feed_byte(&mut self, byte: u8) -> Option<CmuxFrame> {
        match self.state {
            DecoderState::Idle => {
                if byte == CMUX_FLAG {
                    self.state = DecoderState::InFrame;
                    self.current.clear();
                }
                None
            }
            DecoderState::InFrame => {
                if byte == CMUX_FLAG {
                    if !self.current.is_empty() {
                        let result = self.process_complete_frame();
                        self.state = DecoderState::Idle;
                        return result;
                    }
                    // Пустой кадр — skip
                    self.state = DecoderState::Idle;
                } else if byte == 0xF9 {
                    // Escape sequence — не стандарт, но для безопасности
                    self.current.push(byte).ok();
                } else {
                    self.current.push(byte).ok();
                    if self.current.len() >= 300 {
                        self.state = DecoderState::Idle; // overflow, drop
                    }
                }
                None
            }
            DecoderState::Escape => {
                self.current.push(byte).ok();
                self.state = DecoderState::InFrame;
                None
            }
        }
    }

    /// Обработка завершённого кадра
    fn process_complete_frame(&mut self) -> Option<CmuxFrame> {
        parse_cmux_frame(&self.current)
    }
}

// ── Парсинг кадра ────────────────────────────────────────────────────────

/// Распарсить кадр CMUX из байтов (без флагов 0xF9)
pub fn parse_cmux_frame(raw: &[u8]) -> Option<CmuxFrame> {
    if raw.len() < 3 {
        return None; // Минимум: address + control + length(1)
    }

    // Address: EA(1) CR(1) DLCI(6)
    let addr = raw[0];
    let _ea = addr & 0x01;
    let _cr = (addr >> 1) & 0x01;
    let dlci = (addr >> 2) & 0x3F;

    // Control
    let control = raw[1];

    // Length: первый байт — EA(1) + 7 бит длины, второй байт — старшие биты если EA=0
    let len_byte = raw[2];
    let ea = len_byte & 0x01;
    let mut data_len = (len_byte >> 1) as usize;
    let mut header_len = 3;

    if ea == 0 {
        if raw.len() < 4 {
            return None;
        }
        data_len |= (raw[3] as usize) << 7;
        header_len = 4;
    }

    // Проверяем FCS (последний байт перед концом)
    let total_len = header_len + data_len + 1; // +1 для FCS
    if raw.len() < total_len {
        return None;
    }

    // Извлечь данные
    let mut data = Vec::new();
    if data_len > 0 {
        let start = header_len;
        let end = start + data_len;
        if end <= raw.len() {
            data.extend_from_slice(&raw[start..end]).ok();
        }
    }

    Some(CmuxFrame {
        dlci,
        control,
        data,
    })
}

// ── Кодирование кадра ────────────────────────────────────────────────────

/// Закодировать кадр CMUX для отправки в UART
pub fn encode_cmux_frame(dlci: u8, control: u8, data: &[u8]) -> Vec<u8, 300> {
    let mut frame = Vec::new();

    // Flag
    frame.push(CMUX_FLAG).ok();

    // Address: EA=1, CR=1 (command from initiator), DLCI
    let addr = 0x01 | 0x02 | ((dlci & 0x3F) << 2);
    frame.push(addr).ok();

    // Control
    frame.push(control).ok();

    // Length
    let len = data.len();
    if len <= 0x7F {
        // Однобайтовая длина: EA=1
        frame.push(((len as u8) << 1) | 0x01).ok();
    } else {
        // Двухбайтовая длина: EA=0 в первом, второй — старшие биты
        frame.push(((len as u8 & 0x7F) << 1) | 0x00).ok(); // EA=0
        frame.push((len >> 7) as u8).ok();
    }

    // Data
    frame.extend_from_slice(data).ok();

    // FCS — CRC для address + control (упрощённый)
    let fcs = compute_fcs(&frame[1..frame.len()]); // skip flag
    frame.push(fcs).ok();

    // Flag
    frame.push(CMUX_FLAG).ok();

    frame
}

// ── FCS — Frame Check Sequence ───────────────────────────────────────────

/// Вычислить FCS для кадра CMUX (CRC-8)
/// Полином: x^8 + x^2 + x + 1 (= 0x07, инициализация 0xFF)
pub fn compute_fcs(data: &[u8]) -> u8 {
    let mut fcs: u8 = 0xFF;
    for &byte in data {
        fcs ^= byte;
        for _ in 0..8 {
            if fcs & 0x01 != 0 {
                fcs = (fcs >> 1) ^ 0x8C; // 0x8C = reflected polynomial
            } else {
                fcs >>= 1;
            }
        }
    }
    fcs
}

// ── MSC (Modem Status Command) — управление потоком ─────────────────────

/// Создать MSC SET кадр для DLCI
pub fn encode_msc_set(dlci: u8, fc_on: bool) -> Vec<u8, 12> {
    let mut msc_data = Vec::new();
    msc_data.push(MSC_SET).ok();
    msc_data.push(((dlci & 0x3F) << 2) | 0x01 | 0x02).ok(); // EA=1, CR=1
    msc_data.push(0x01).ok(); // length = 1
    let signals = if fc_on { 0x0D } else { 0x05 }; // FC=0/1, RTC=1, RTR=1
    msc_data.push(signals | 0x0E).ok(); // EA=1
    msc_data
}

/// Отправить SABM для установки DLCI
pub fn encode_sabm(dlci: u8) -> Vec<u8, 300> {
    encode_cmux_frame(dlci, SABM, &[])
}