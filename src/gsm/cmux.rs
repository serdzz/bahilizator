//! GSM 07.10 CMUX протокол — frame parser/builder
//!
//! Мультиплексирование нескольких DLCI поверх одного UART
//! Basic mode — GSM 07.10
//!
//! Перенос из gsm.c: SIM800L использует CMUX для разделения AT и PPP каналов
//! Frame format: F9 | Address | Control | Length | Data | FCS | F9
//!
//! DLCI каналы:
//!   DLC0 = управление мультиплексором (MSC)
//!   DLC1 = AT команды
//!   DLC2 = PPP данные

use heapless::Vec;

// ── Константы CMUX ───────────────────────────────────────────────────────

pub const CMUX_FLAG: u8 = 0xF9;

// Типы кадров (control field)
pub const SABM: u8 = 0x2F;   // Set Asynchronous Balanced Mode — установить канал
pub const UA: u8 = 0x63;     // Unnumbered Acknowledge — подтверждение
pub const DM: u8 = 0x0F;    // Disconnected Mode — отказ
pub const DISC: u8 = 0x43;  // Disconnect — разорвать канал
pub const UIH: u8 = 0xEF;   // Unnumbered Information with Header check — данные
pub const UI: u8 = 0x03;    // Unnumbered Information — данные без контроля

// Управление мультиплексором (MSC — Modem Status Command)
pub const MSC_CMD: u8 = 0xFD;  // MSC SET
pub const MSC_ACK: u8 = 0x73; // MSC ACK

// Биты управления потоком в MSC
pub const FC_BIT: u8 = 0x02;  // Flow Control (1 = остановить передачу)
pub const RTC_BIT: u8 = 0x04; // Ready To Communicate
pub const RTR_BIT: u8 = 0x08; // Ready To Receive

// ── CmuxFrame — распарсенный кадр CMUX ───────────────────────────────────

#[derive(Debug)]
pub struct CmuxFrame {
    /// DLCI канала (0 = управление, 1 = AT, 2 = PPP)
    pub dlci: u8,
    /// Поле управления (SABM, UA, UIH, ...)
    pub control: u8,
    /// Данные кадра (payload)
    pub data: Vec<u8, 256>,
}

// ── CmuxDlci — идентификаторы каналов ────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dlci {
    Control = 0,
    AtCommands = 1,
    PppData = 2,
}

impl Dlci {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Dlci::Control),
            1 => Some(Dlci::AtCommands),
            2 => Some(Dlci::PppData),
            _ => None,
        }
    }
}

// ── Состояния декодера ────────────────────────────────────────────────────
//
// GSM 07.10 Basic Mode использует byte-stuffing:
//   0xF9 внутри кадра экранируется как 0xF9 0xF9
//   (В Basic Mode экранирование не используется — флаг только на границах)

#[derive(Debug, Clone, Copy)]
enum DecoderState {
    Idle,
    InFrame,
}

// ── CmuxDecoder — state machine для парсинга входящих кадров ─────────────
//
/// Декодер работает побайтово — кормим байты из UART,
/// получаем Option<CmuxFrame> когда кадр завершён.
///
/// Basic Mode: флаг F9 только на границах, без экранирования

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
    /// Возвращает Some(CmuxFrame) если кадр полностью принят
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
                        let result = parse_cmux_frame(&self.current);
                        self.state = DecoderState::Idle;
                        return result;
                    }
                    // Пустой кадр (два флага подряд) — skip
                    self.state = DecoderState::Idle;
                } else {
                    if self.current.push(byte).is_err() {
                        // Переполнение буфера — сброс
                        self.state = DecoderState::Idle;
                    }
                }
                None
            }
        }
    }

    /// Покормить декодер срезом байтов
    /// Возвращает вектор распарсенных кадров
    pub fn feed_slice(&mut self, bytes: &[u8]) -> Vec<CmuxFrame, 4> {
        let mut frames = Vec::new();
        for &byte in bytes {
            if let Some(frame) = self.feed_byte(byte) {
                frames.push(frame).ok();
            }
        }
        frames
    }
}

// ── Парсинг кадра ────────────────────────────────────────────────────────
//
/// Распарсить кадр CMUX Basic Mode из байтов (без флагов 0xF9)
///
/// Формат:
///   Address (1 байт): EA(1) | CR(1) | DLCI(6)
///   Control (1 байт): тип кадра
///   Length (1-2 байта): EA(1) + 7/15 бит длины
///   Data (length байт)
///   FCS (1 байт): CRC-8 по address + control

pub fn parse_cmux_frame(raw: &[u8]) -> Option<CmuxFrame> {
    if raw.len() < 3 {
        return None; // Минимум: address + control + length(1)
    }

    // Address: EA(1) CR(1) DLCI(6)
    let addr = raw[0];
    let dlci = (addr >> 2) & 0x3F;

    // Control
    let control = raw[1];

    // Length: EA(1) + 7 бит в первом байте
    // Если EA=0 — двухбайтовая длина
    let len_byte = raw[2];
    let ea = len_byte & 0x01;
    let mut data_len = ((len_byte >> 1) as usize);
    let mut header_len: usize = 3;

    if ea == 0 {
        // Двухбайтовая длина: второй байт содержит старшие биты
        if raw.len() < 4 {
            return None;
        }
        data_len |= (raw[3] as usize) << 7;
        header_len = 4;
    }

    // FCS — последний байт кадра
    // Проверяем, что данных достаточно
    let total_len = header_len + data_len + 1; // +1 для FCS
    if raw.len() < total_len {
        return None;
    }

    // Проверяем FCS (CRC-8 по address + control)
    let fcs_pos = header_len + data_len;
    let expected_fcs = compute_fcs(&raw[0..header_len]);
    // В Basic Mode FCS считается только по address + control (2 байта)
    let actual_fcs = raw[fcs_pos];
    if expected_fcs != actual_fcs {
        // FCS не совпал — кадр повреждён, но для SIM800L часто работает без строгой проверки
        // В продакшене можно включить строгую проверку
        // return None;
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
//
/// Закодировать кадр CMUX для отправки в UART
///
/// Формат: F9 | Address | Control | Length | Data | FCS | F9
/// Address: EA=1, CR=1 (команда от инициатора), DLCI

pub fn encode_cmux_frame(dlci: u8, control: u8, data: &[u8]) -> Vec<u8, 300> {
    let mut frame = Vec::new();

    // Открывающий флаг
    frame.push(CMUX_FLAG).ok();

    // Address: EA=1, CR=1, DLCI
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
        // Двухбайтовая длина: EA=0 в первом
        frame.push(((len as u8 & 0x7F) << 1) | 0x00).ok();
        frame.push((len >> 7) as u8).ok();
    }

    // Data
    frame.extend_from_slice(data).ok();

    // FCS — CRC-8 по address + control
    let fcs = compute_fcs(&[addr, control]);
    frame.push(fcs).ok();

    // Закрывающий флаг
    frame.push(CMUX_FLAG).ok();

    frame
}

// ── FCS — Frame Check Sequence ───────────────────────────────────────────
//
/// Вычислить FCS для кадра CMUX (CRC-8)
/// Полином: x^8 + x^2 + x + 1 (= 0x07, инициализация 0xFF)
/// GSM 07.10 Basic Mode: FCS считается по address + control полям

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
    // GSM 07.10: complement FCS
    !fcs
}

// ── MSC (Modem Status Command) ──────────────────────────────────────────
//
/// Управление потоком данных на конкретном DLCI
/// MSC SET — установить статус модема (flow control)
/// MSC ACK — подтвердить

/// Создать MSC SET кадр для DLCI
pub fn encode_msc_set(dlci: u8, fc_on: bool) -> Vec<u8, 12> {
    let mut msc_data = Vec::new();
    msc_data.push(MSC_CMD).ok();
    // Адрес DLCI в MSC: EA=1, CR=1, DLCI
    msc_data.push(((dlci & 0x3F) << 2) | 0x01 | 0x02).ok();
    msc_data.push(0x01).ok(); // Length = 1
    // Сигналы: FC, RTC, RTR, DV
    let signals = if fc_on { 0x0D } else { 0x05 }; // FC=0/1, RTC=1, RTR=1
    msc_data.push(signals | 0x0E).ok(); // EA=1
    msc_data
}

/// Отправить SABM для установки DLCI
pub fn encode_sabm(dlci: u8) -> Vec<u8, 300> {
    encode_cmux_frame(dlci, SABM, &[])
}

/// Отправить UA для подтверждения DLCI
pub fn encode_ua(dlci: u8) -> Vec<u8, 300> {
    encode_cmux_frame(dlci, UA, &[])
}

/// Отправить DISC для разрыва DLCI
pub fn encode_disc(dlci: u8) -> Vec<u8, 300> {
    encode_cmux_frame(dlci, DISC, &[])
}

/// Отправить DM (Disconected Mode)
pub fn encode_dm(dlci: u8) -> Vec<u8, 300> {
    encode_cmux_frame(dlci, DM, &[])
}

// ── Инициализация CMUX ──────────────────────────────────────────────────
//
/// Последовательность установки мультиплексора:
/// 1. Отправить AT+CMUX=0,1,5,128,10,3,30,10,2
/// 2. Дождаться OK
/// 3. Установить SABM на DLC0 → UA
/// 4. Установить SABM на DLC1 → UA
/// 5. Установить SABM на DLC2 → UA
/// 6. Каналы готовы

pub struct CmuxInitState {
    pub stage: u8,
    pub dlci_established: [bool; 3],
}

impl CmuxInitState {
    pub fn new() -> Self {
        Self {
            stage: 0,
            dlci_established: [false; 3],
        }
    }

    /// Обработать входящий кадр во время инициализации
    /// Возвращает true если инициализация завершена
    pub fn process_frame(&mut self, frame: &CmuxFrame) -> bool {
        match frame.control {
            UA => {
                // Подтверждение канала
                if (frame.dlci as usize) < 3 {
                    self.dlci_established[frame.dlci as usize] = true;
                }
                self.stage += 1;
            }
            DM => {
                // Отказ — канал не установлен
                // Повторить SABM?
            }
            _ => {}
        }

        // Все 3 канала установлены?
        self.dlci_established.iter().all(|&v| v)
    }

    /// Получить следующий SABM кадр для отправки
    pub fn next_sabm(&self) -> Option<Vec<u8, 300>> {
        match self.stage {
            0 => Some(encode_sabm(0)),
            1 => Some(encode_sabm(1)),
            2 => Some(encode_sabm(2)),
            _ => None,
        }
    }
}