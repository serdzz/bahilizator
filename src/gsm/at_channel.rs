//! AT command channel поверх CMUX DLC1
//!
//! Обёртка CMUX DLC1 → Read + Write для atat crate
//! Ручное формирование AT команд для SIM800L
//!
//! SIM800L AT команды:
//!   AT+CGATT — GPRS attach
//!   AT+CSTT — APN settings
//!   AT+CIICR — GPRS connect
//!   AT+CIFSR — Get IP
//!   AT+CIPSTART — TCP connection
//!   AT+CIPSEND — Send data
//!   AT+CMGF — SMS format
//!   AT+CMGS — Send SMS
//!
//! URC (Unsolicited Result Code):
//!   +CMT — incoming SMS
//!   +CGEV — GPRS events

use heapless::String;

use crate::gsm::cmux;

// ── AT ответ ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, defmt::Format)]
pub enum AtResponse {
    Ok,
    Error,
    Data,
    Prompt, // ">" — готов принять SMS текст
    Timeout,
    CmeError(u16),
    CmsError(u16),
}

// ── URC — Unsolicited Result Codes ───────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Urc {
    /// Входящее SMS: +CMT: <sender>,<timestamp>\n<text>
    IncomingSms {
        sender: String<20>,
        text: String<160>,
    },
    /// GPRS событие: +CGEV: ...
    GprsEvent(String<40>),
    /// Call Ready — модем готов
    CallReady,
    /// Power Down
    PowerDown,
    /// Voltage Warning
    VoltageWarning,
    /// SMS отправлен: +CMGS: <id>
    SmsSent(u16),
    /// Неизвестный URC
    Unknown(String<80>),
}

// ── CmuxAtChannel — AT канал через CMUX DLC1 ─────────────────────────────
//
/// Обёртка для отправки AT команд через CMUX DLC1 и
/// парсинга ответов из DLC1
pub struct CmuxAtChannel {
    /// DLCI для AT команд (обычно 1)
    pub dlci: u8,
    /// Буфер приёма (данные из DLC1)
    rx_buffer: Vec<u8, 512>,
}

use heapless::Vec;

impl CmuxAtChannel {
    /// Создать новый AT канал для указанного DLCI
    pub fn new(dlci: u8) -> Self {
        Self {
            dlci,
            rx_buffer: Vec::new(),
        }
    }

    /// Добавить полученные данные в буфер
    pub fn feed_rx_data(&mut self, data: &[u8]) {
        for &byte in data {
            self.rx_buffer.push(byte).ok();
        }
    }

    /// Прочитать и разобрать ответ из буфера
    /// Ищет "OK", "ERROR", "+CME ERROR", "+CMS ERROR", ">"
    pub fn parse_response(&mut self) -> Option<AtResponse> {
        // Ищем маркеры ответа в буфере
        let buf_str = core::str::from_utf8(&self.rx_buffer).unwrap_or("");

        if buf_str.contains("OK") {
            self.rx_buffer.clear();
            return Some(AtResponse::Ok);
        }
        if buf_str.contains("ERROR") {
            // Проверяем тип ошибки
            if let Some(pos) = buf_str.find("+CME ERROR:") {
                let code = parse_error_code(&buf_str[pos + 11..]);
                self.rx_buffer.clear();
                return Some(AtResponse::CmeError(code));
            }
            if let Some(pos) = buf_str.find("+CMS ERROR:") {
                let code = parse_error_code(&buf_str[pos + 11..]);
                self.rx_buffer.clear();
                return Some(AtResponse::CmsError(code));
            }
            self.rx_buffer.clear();
            return Some(AtResponse::Error);
        }
        if buf_str.contains(">") {
            self.rx_buffer.clear();
            return Some(AtResponse::Prompt);
        }

        None // Ответ ещё не полный
    }

    /// Разобрать URC из буфера
    pub fn parse_urc(&mut self) -> Option<Urc> {
        let buf_str = core::str::from_utf8(&self.rx_buffer).unwrap_or("");

        if buf_str.contains("+CMT:") {
            // Входящее SMS
            // Формат: +CMT: "<sender>","<timestamp>"\n<text>
            let sender = extract_between_quotes(buf_str, 0).unwrap_or_default();
            // Текст SMS — после второго \n
            let text = if let Some(pos) = buf_str.find('\n') {
                let rest = &buf_str[pos + 1..];
                let trimmed = rest.trim_end_matches('\r');
                let mut t = String::new();
                t.push_str(trimmed).ok();
                t
            } else {
                String::new()
            };
            self.rx_buffer.clear();
            return Some(Urc::IncomingSms { sender, text });
        }

        if buf_str.contains("+CGEV:") {
            let mut s = String::new();
            s.push_str(buf_str.trim()).ok();
            self.rx_buffer.clear();
            return Some(Urc::GprsEvent(s));
        }

        if buf_str.contains("Call Ready") {
            self.rx_buffer.clear();
            return Some(Urc::CallReady);
        }

        if buf_str.contains("NORMAL POWER DOWN")
            || buf_str.contains("UNDER-VOLTAGE POWER DOWN")
            || buf_str.contains("OVER-VOLTAGE POWER DOWN")
        {
            self.rx_buffer.clear();
            return Some(Urc::PowerDown);
        }

        if buf_str.contains("+CMGS:") {
            // +CMGS: <id>
            let code = parse_number_after_colon(buf_str);
            self.rx_buffer.clear();
            return Some(Urc::SmsSent(code as u16));
        }

        if buf_str.contains("UNDER-VOLTAGE WARNING") || buf_str.contains("OVER-VOLTAGE WARNING") {
            self.rx_buffer.clear();
            return Some(Urc::VoltageWarning);
        }

        None
    }

    /// Закодировать AT команду в CMUX кадр для отправки
    pub fn encode_at_cmd(&self, cmd: &str) -> Vec<u8, 300> {
        cmux::encode_cmux_frame(self.dlci, cmux::UIH, cmd.as_bytes())
    }

    /// Очистить буфер приёма
    pub fn clear(&mut self) {
        self.rx_buffer.clear();
    }
}

// ── GsmAtClient — высокоуровневый AT клиент ──────────────────────────────
//
/// Инкапсулирует последовательность AT команд для SIM800L
/// Включает: GPRS attach, SMS send/receive, TCP connect
pub struct GsmAtClient {
    /// AT канал через CMUX DLC1
    pub channel: CmuxAtChannel,
    /// Текущий этап инициализации
    pub init_stage: AtInitStage,
    /// GPRS подключён?
    pub gprs_attached: bool,
    /// IP адрес (если получен)
    pub ip_addr: Option<[u8; 4]>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AtInitStage {
    Idle,
    EchoOff,
    Cmgf,
    Cnmi,
    Csmp,
    Cclk,
    Cgatt,
    Cstt,
    Ciicr,
    Cifsr,
    Ready,
}

impl GsmAtClient {
    /// Создать новый AT клиент
    pub fn new(channel: CmuxAtChannel) -> Self {
        Self {
            channel,
            init_stage: AtInitStage::Idle,
            gprs_attached: false,
            ip_addr: None,
        }
    }

    /// Получить следующую AT команду для инициализации
    /// Возвращает None если инициализация завершена
    pub fn next_init_cmd(&mut self) -> Option<&'static str> {
        let cmd = match self.init_stage {
            AtInitStage::Idle => {
                self.init_stage = AtInitStage::EchoOff;
                "ATE0\r"
            }
            AtInitStage::EchoOff => {
                self.init_stage = AtInitStage::Cmgf;
                "AT+CMGF=1\r"
            }
            AtInitStage::Cmgf => {
                self.init_stage = AtInitStage::Cnmi;
                "AT+CNMI=2,2,0,0,0\r"
            }
            AtInitStage::Cnmi => {
                self.init_stage = AtInitStage::Csmp;
                "AT+CSMP=17,167,0,0\r"
            }
            AtInitStage::Csmp => {
                self.init_stage = AtInitStage::Cclk;
                "AT+CCLK?\r"
            }
            AtInitStage::Cclk => {
                self.init_stage = AtInitStage::Ready;
                return None; // Инициализация завершена
            }
            AtInitStage::Ready => return None,
            _ => return None,
        };
        Some(cmd)
    }

    /// Обработать ответ на AT команду при инициализации
    pub fn process_init_response(&mut self, response: AtResponse) {
        match response {
            AtResponse::Ok => {
                // Переход к следующему этапу уже сделан в next_init_cmd
            }
            AtResponse::Error | AtResponse::CmeError(_) | AtResponse::CmsError(_) => {
                // Ошибка — остаёмся на текущем этапе (повторить)
            }
            _ => {}
        }
    }

    /// Начать GPRS подключение
    pub fn start_gprs_connect(&mut self) -> Option<&'static str> {
        if self.init_stage != AtInitStage::Ready {
            return None;
        }
        self.init_stage = AtInitStage::Cgatt;
        Some("AT+CGATT=1\r")
    }

    /// Следующая команда GPRS подключения
    pub fn next_gprs_cmd(&mut self) -> Option<&'static str> {
        let cmd = match self.init_stage {
            AtInitStage::Cgatt => {
                self.init_stage = AtInitStage::Cstt;
                "AT+CSTT=\"internet\"\r" // APN = "internet"
            }
            AtInitStage::Cstt => {
                self.init_stage = AtInitStage::Ciicr;
                "AT+CIICR\r"
            }
            AtInitStage::Ciicr => {
                self.init_stage = AtInitStage::Cifsr;
                "AT+CIFSR\r"
            }
            AtInitStage::Cifsr => {
                self.init_stage = AtInitStage::Ready;
                self.gprs_attached = true;
                return None;
            }
            _ => return None,
        };
        Some(cmd)
    }

    /// Закодировать команду отправки SMS в CMUX кадр
    pub fn encode_sms_send(&self, number: &str, text: &str) -> Vec<Vec<u8, 300>, 2> {
        let mut frames = Vec::new();

        // Шаг 1: AT+CMGS="<number>"
        let mut cmd = String::<40>::new();
        use core::fmt::Write;
        let _ = write!(cmd, "AT+CMGS=\"{}\"\r", number);
        frames
            .push(cmux::encode_cmux_frame(
                self.channel.dlci,
                cmux::UIH,
                cmd.as_bytes(),
            ))
            .ok();

        // Шаг 2: текст + Ctrl+Z (0x1A)
        let mut msg_bytes = Vec::<u8, 200>::new();
        msg_bytes.extend_from_slice(text.as_bytes()).ok();
        msg_bytes.push(0x1A).ok(); // Ctrl+Z
        frames
            .push(cmux::encode_cmux_frame(
                self.channel.dlci,
                cmux::UIH,
                &msg_bytes,
            ))
            .ok();

        frames
    }
}

// ── Вспомогательные функции парсинга ──────────────────────────────────────

/// Разобрать числовой код ошибки после ":"
fn parse_error_code(s: &str) -> u16 {
    let trimmed = s.trim();
    let num_str = trimmed
        .split(|c: char| !c.is_ascii_digit())
        .next()
        .unwrap_or("0");
    num_str.parse::<u16>().unwrap_or(0)
}

/// Разобрать число после ":" (для +CMGS и т.д.)
fn parse_number_after_colon(s: &str) -> u32 {
    if let Some(pos) = s.find(':') {
        parse_error_code(&s[pos + 1..]) as u32
    } else {
        0
    }
}

/// Извлечь текст между кавычками (для +CMT sender)
fn extract_between_quotes(s: &str, start: usize) -> Option<String<20>> {
    let bytes = s.as_bytes();
    let mut first_quote = None;
    let mut second_quote = None;
    let mut count = 0;

    for (i, &b) in bytes.iter().enumerate().skip(start) {
        if b == b'"' {
            count += 1;
            if count == 1 {
                first_quote = Some(i);
            } else if count == 2 {
                second_quote = Some(i);
                break;
            }
        }
    }

    if let (Some(f), Some(s)) = (first_quote, second_quote) {
        let inner = core::str::from_utf8(&bytes[f + 1..s]).unwrap_or("");
        let mut result = String::new();
        result.push_str(inner).ok();
        Some(result)
    } else {
        None
    }
}
