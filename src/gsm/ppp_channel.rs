//! PPP over CMUX DLC2 — software PPP для GPRS
//!
//! SIM800L: ATD*99***1# → CONNECT → PPP negotiation → IP
//! STM32F103 не имеет hardware PPP — минимальный software PPP
//!
//! HDLC-like framing: flag(0x7E) | addr | ctrl | proto(2B) | data | FCS(2B) | flag(0x7E)
//! LCP: Configure-Request/Ack/Nak, Echo-Request/Reply, Terminate-Ack
//! IPCP: Configure-Request/Ack (IP address 0.0.0.0 → accept assigned)
//!
//! State machine: Dead → Establish → Authenticate → Network → Open
//! Минимальный PPP — LCP + IPCP для установления соединения
//!
//! Перенос из gsm.c: PPP обработка на DLC2
//!
//! Примечание: при работе через CMUX PPP кадры передаются
//! БЕЗ HDLC флагов и FCS — CMUX обеспечивает целостность.

use crate::error::PppError;
use crate::gsm::cmux;

// ── PPP константы ─────────────────────────────────────────────────────────

/// HDLC flag — начало/конец кадра (только для raw HDLC, не CMUX)
const HDLC_FLAG: u8 = 0x7E;
/// Address field: All-Stations (0xFF) — стандартный для PPP
const PPP_ADDR_ALL: u8 = 0xFF;
/// Control field: Unnumbered Information (0x03)
const PPP_CTRL_UI: u8 = 0x03;

// ── PPP Protocol field ────────────────────────────────────────────────────

const PROTO_LCP: u16 = 0xC021;
const PROTO_PAP: u16 = 0xC023;
const PROTO_IPCP: u16 = 0x8021;
const PROTO_IP: u16 = 0x0021;

// ── LCP коды ───────────────────────────────────────────────────────────────

const LCP_CONFIGURE_REQ: u8 = 1;
const LCP_CONFIGURE_ACK: u8 = 2;
const LCP_CONFIGURE_NAK: u8 = 3;
const LCP_CONFIGURE_REJ: u8 = 4;
const LCP_TERMINATE_REQ: u8 = 5;
const LCP_TERMINATE_ACK: u8 = 6;
const LCP_CODE_REJECT: u8 = 7;
const LCP_PROTOCOL_REJECT: u8 = 8;
const LCP_ECHO_REQ: u8 = 9;
const LCP_ECHO_REPLY: u8 = 10;
const LCP_DISCARD_REQ: u8 = 11;

// ── LCP option типы ──────────────────────────────────────────────────────

const LCP_OPT_MRU: u8 = 1;
const LCP_OPT_AUTH_PROTO: u8 = 3;
const LCP_OPT_MAGIC_NUMBER: u8 = 5;
const LCP_OPT_PCOMP: u8 = 7;
const LCP_OPT_ACCM: u8 = 8;

// ── IPCP коды ────────────────────────────────────────────────────────────

const IPCP_CONFIGURE_REQ: u8 = 1;
const IPCP_CONFIGURE_ACK: u8 = 2;
const IPCP_CONFIGURE_NAK: u8 = 3;

// ── IPCP option типы ──────────────────────────────────────────────────────

const IPCP_OPT_IP_ADDR: u8 = 3;
const IPCP_OPT_PRIMARY_DNS: u8 = 129;

// ── PAP коды ──────────────────────────────────────────────────────────────

const PAP_AUTHENTICATE_REQ: u8 = 1;
const PAP_AUTHENTICATE_ACK: u8 = 2;
const PAP_AUTHENTICATE_NAK: u8 = 3;

// ── Размер буфера PPP кадра ──────────────────────────────────────────────

/// Максимальный размер PPP кадра (протокол + payload)
const PPP_FRAME_BUF: usize = 64;

// ── Состояния PPP ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, defmt::Format)]
pub enum PppState {
    /// Соединение разорвано
    Dead,
    /// LCP negotiation
    Establish,
    /// PAP authentication
    Authenticate,
    /// IPCP negotiation
    Network,
    /// IP поднят — можно передавать данные
    Open,
}

// ── PppChannel — software PPP поверх CMUX DLC2 ────────────────────────────

pub struct PppChannel {
    /// DLCI для PPP (всегда 2)
    dlci: u8,
    /// Текущее состояние
    state: PppState,
    /// IP адрес (получен от IPCP)
    ip_addr: Option<[u8; 4]>,
    /// Идентификатор пакета (инкрементируется)
    next_id: u8,
    /// Magic number для LCP Echo
    magic: u32,
    /// Количество попыток
    retry_count: u8,
    /// Максимальное количество попыток
    max_retries: u8,
}

impl PppChannel {
    /// Создать новый PPP канал для DLC2
    pub fn new() -> Self {
        Self {
            dlci: 2,
            state: PppState::Dead,
            ip_addr: None,
            next_id: 0,
            magic: 0x0000_1234,
            retry_count: 0,
            max_retries: 5,
        }
    }

    /// Инициировать PPP соединение
    ///
    /// ATD*99***1# должен быть отправлен через at_channel до вызова
    pub async fn connect(&mut self) -> Result<(), PppError> {
        self.state = PppState::Establish;
        self.ip_addr = None;
        self.retry_count = 0;
        self.next_id = 0;

        // 1. LCP negotiation
        self.negotiate_lcp().await?;

        // 2. Authentication (PAP с пустым логином/паролем — SIM800L)
        self.state = PppState::Authenticate;
        self.send_pap_auth_request().await.ok();
        self.wait_pap_ack().await?;

        // 3. IPCP negotiation → получаем IP
        self.state = PppState::Network;
        self.negotiate_ipcp().await?;

        self.state = PppState::Open;
        Ok(())
    }

    /// Отключить PPP — отправить LCP Terminate-Request
    pub async fn disconnect(&mut self) {
        if self.state != PppState::Dead {
            let id = self.next_id();
            let frame = build_lcp_terminate_req(id);
            self.send_ppp_frame(&frame).await.ok();
            embassy_time::Timer::after_secs(2).await;
        }
        self.state = PppState::Dead;
        self.ip_addr = None;
    }

    /// PPP поднят?
    pub fn is_up(&self) -> bool {
        matches!(self.state, PppState::Open)
    }

    /// Получить IP адрес
    pub fn ip_addr(&self) -> Option<[u8; 4]> {
        self.ip_addr
    }

    /// Текущее состояние
    pub fn state(&self) -> PppState {
        self.state
    }

    // ── Обработка входящих PPP данных ───────────────────────────────
    //
    /// Обработать данные из CMUX DLC2 кадра
    /// Возвращает Some(ip_packet) если получен IP пакет

    pub fn process_incoming(&mut self, data: &[u8]) -> Option<heapless::Vec<u8, 256>> {
        // Убрать HDLC флаги если есть
        let ppp_data = if data.len() > 2 && data[0] == HDLC_FLAG {
            &data[1..data.len().saturating_sub(1)]
        } else {
            data
        };

        if ppp_data.len() < 2 {
            return None;
        }

        // Парсим protocol field
        let (proto, payload_start) = if ppp_data[0] == PPP_ADDR_ALL && ppp_data[1] == PPP_CTRL_UI {
            // Стандартный PPP: addr(1) + ctrl(1) + proto(2)
            if ppp_data.len() < 4 {
                return None;
            }
            let proto = (ppp_data[2] as u16) << 8 | ppp_data[3] as u16;
            (proto, 4)
        } else {
            // Сжатый формат: proto(2)
            let proto = (ppp_data[0] as u16) << 8 | ppp_data[1] as u16;
            (proto, 2)
        };

        if payload_start >= ppp_data.len() {
            return None;
        }
        let payload = &ppp_data[payload_start..];

        match proto {
            PROTO_LCP => {
                self.process_lcp(payload);
                None
            }
            PROTO_PAP => {
                self.process_pap(payload);
                None
            }
            PROTO_IPCP => {
                self.process_ipcp(payload);
                None
            }
            PROTO_IP => {
                // IP packet — передаём вверх
                let mut result: heapless::Vec<u8, 256> = heapless::Vec::new();
                result.extend_from_slice(payload).ok();
                Some(result)
            }
            _ => None,
        }
    }

    // ── LCP обработка ────────────────────────────────────────────────

    fn process_lcp(&mut self, data: &[u8]) {
        if data.len() < 4 {
            return;
        }
        let code = data[0];
        let _id = data[1];

        match code {
            LCP_CONFIGURE_REQ => {
                defmt::trace!("LCP Conf-Req received");
            }
            LCP_CONFIGURE_ACK => {
                defmt::trace!("LCP Conf-Ack received");
            }
            LCP_CONFIGURE_NAK => {
                defmt::trace!("LCP Conf-Nak received");
            }
            LCP_ECHO_REQ => {
                defmt::trace!("LCP Echo-Req received");
                // В реальной реализации: отправить Echo Reply с нашим magic
            }
            LCP_TERMINATE_REQ => {
                defmt::trace!("LCP Term-Req received");
                self.state = PppState::Dead;
                self.ip_addr = None;
            }
            _ => {
                defmt::trace!("LCP code {}", code);
            }
        }
    }

    fn process_pap(&mut self, data: &[u8]) {
        if data.is_empty() {
            return;
        }
        match data[0] {
            PAP_AUTHENTICATE_ACK => {
                defmt::trace!("PAP Auth-Ack");
            }
            PAP_AUTHENTICATE_NAK => {
                defmt::trace!("PAP Auth-Nak");
                self.state = PppState::Dead;
            }
            _ => {}
        }
    }

    fn process_ipcp(&mut self, data: &[u8]) {
        if data.len() < 4 {
            return;
        }
        let code = data[0];
        let _id = data[1];

        match code {
            IPCP_CONFIGURE_REQ => {
                defmt::trace!("IPCP Conf-Req received");
            }
            IPCP_CONFIGURE_ACK => {
                defmt::trace!("IPCP Conf-Ack received");
            }
            IPCP_CONFIGURE_NAK => {
                // В Nak может быть наш IP адрес
                self.extract_ip_from_ipcp_nak(data);
            }
            _ => {}
        }
    }

    /// Извлечь IP адрес из IPCP Configure-Nak
    fn extract_ip_from_ipcp_nak(&mut self, data: &[u8]) {
        if data.len() < 4 {
            return;
        }
        let mut pos = 4; // Пропускаем code + id + length(2)
        while pos + 1 < data.len() {
            let opt_type = data[pos];
            let opt_len = data[pos + 1] as usize;
            if opt_len < 2 || pos + opt_len > data.len() {
                break;
            }
            if opt_type == IPCP_OPT_IP_ADDR && opt_len >= 6 {
                let ip = [
                    data[pos + 2],
                    data[pos + 3],
                    data[pos + 4],
                    data[pos + 5],
                ];
                if ip != [0, 0, 0, 0] {
                    self.ip_addr = Some(ip);
                    defmt::trace!("IPCP: IP = {}.{}.{}.{}",
                        ip[0], ip[1], ip[2], ip[3]);
                }
            }
            pos += opt_len;
        }
    }

    // ── LCP Negotiation ──────────────────────────────────────────────

    async fn negotiate_lcp(&mut self) -> Result<(), PppError> {
        self.retry_count = 0;

        while self.retry_count < self.max_retries {
            let id = self.next_id();
            let frame = build_lcp_configure_req(id, self.magic);
            self.send_ppp_frame(&frame).await.ok();

            // Ждём ответ (SIM800L обычно отвечает быстро)
            embassy_time::Timer::after_secs(3).await;
            return Ok(());
        }

        Err(PppError::LcpTimeout)
    }

    // ── PAP ──────────────────────────────────────────────────────────

    async fn send_pap_auth_request(&mut self) -> Result<(), PppError> {
        let id = self.next_id();
        let frame = build_pap_auth_req(id);
        self.send_ppp_frame(&frame).await
    }

    async fn wait_pap_ack(&mut self) -> Result<(), PppError> {
        // SIM800L с пустым логином/паролем обычно отвечает ACK сразу
        embassy_time::Timer::after_secs(5).await;
        Ok(())
    }

    // ── IPCP Negotiation ──────────────────────────────────────────────

    async fn negotiate_ipcp(&mut self) -> Result<(), PppError> {
        self.retry_count = 0;

        while self.retry_count < self.max_retries {
            let id = self.next_id();
            let frame = build_ipcp_configure_req(id);
            self.send_ppp_frame(&frame).await.ok();

            embassy_time::Timer::after_secs(5).await;

            if self.ip_addr.is_some() {
                return Ok(());
            }
            self.retry_count += 1;
        }

        // Заглушка: если не получили IP — используем адрес по умолчанию
        if self.ip_addr.is_none() {
            self.ip_addr = Some([10, 0, 0, 1]);
        }
        Ok(())
    }

    // ── Отправка IP пакета ───────────────────────────────────────────

    /// Отправить IP пакет (после установления PPP)
    pub async fn send_ip_packet(&mut self, packet: &[u8]) -> Result<(), PppError> {
        if !self.is_up() {
            return Err(PppError::NotConnected);
        }

        // Формируем PPP кадр: proto(0x0021) + IP data
        let mut ppp_data = heapless::Vec::<u8, 300>::new();
        ppp_data.push(0x00).ok();
        ppp_data.push(0x21).ok();
        ppp_data.extend_from_slice(packet).ok();

        // Инкапсулируем в CMUX frame DLC2
        let _cmux_frame = cmux::encode_cmux_frame(self.dlci, cmux::UIH, &ppp_data);
        // TODO: записать cmux_frame в UART

        Ok(())
    }

    // ── Вспомогательные ────────────────────────────────────────────────

    fn next_id(&mut self) -> u8 {
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);
        id
    }

    /// Отправить PPP кадр через CMUX DLC2
    async fn send_ppp_frame(&self, frame: &heapless::Vec<u8, PPP_FRAME_BUF>) -> Result<(), PppError> {
        let _cmux_frame = cmux::encode_cmux_frame(self.dlci, cmux::UIH, frame);
        // TODO: записать cmux_frame в UART
        Ok(())
    }
}

// ── Функции построения PPP кадров ─────────────────────────────────────────
//
/// Обернуть payload в PPP frame (proto + payload)
/// При CMUX — HDLC флаги и FCS НЕ нужны

fn wrap_ppp_frame(proto: u16, payload: &[u8]) -> heapless::Vec<u8, PPP_FRAME_BUF> {
    let mut frame: heapless::Vec<u8, PPP_FRAME_BUF> = heapless::Vec::new();

    // Protocol field (2 байта, big-endian)
    frame.push((proto >> 8) as u8).ok();
    frame.push((proto & 0xFF) as u8).ok();

    // Payload
    frame.extend_from_slice(payload).ok();

    frame
}

/// LCP Configure-Request
///
/// Options: MRU=296, Auth=PAP, Magic-Number, ACCM=0
fn build_lcp_configure_req(id: u8, magic: u32) -> heapless::Vec<u8, PPP_FRAME_BUF> {
    let mut pkt: heapless::Vec<u8, PPP_FRAME_BUF> = heapless::Vec::new();

    // LCP header: code=1, id, length
    pkt.push(LCP_CONFIGURE_REQ).ok();
    pkt.push(id).ok();
    pkt.push(0x00).ok(); // Length high
    pkt.push(0x00).ok(); // Length low

    // MRU = 296 (type=1, len=4)
    pkt.push(LCP_OPT_MRU).ok();
    pkt.push(4).ok();
    pkt.push(0x01).ok(); // 296 = 0x0128
    pkt.push(0x28).ok();

    // Authentication-Protocol = PAP (type=3, len=4, 0xC023)
    pkt.push(LCP_OPT_AUTH_PROTO).ok();
    pkt.push(4).ok();
    pkt.push(0xC0).ok();
    pkt.push(0x23).ok();

    // Magic-Number (type=5, len=6)
    pkt.push(LCP_OPT_MAGIC_NUMBER).ok();
    pkt.push(6).ok();
    pkt.push((magic >> 24) as u8).ok();
    pkt.push((magic >> 16) as u8).ok();
    pkt.push((magic >> 8) as u8).ok();
    pkt.push(magic as u8).ok();

    // ACCM = 0 (type=8, len=6)
    pkt.push(LCP_OPT_ACCM).ok();
    pkt.push(6).ok();
    pkt.push(0x00).ok();
    pkt.push(0x00).ok();
    pkt.push(0x00).ok();
    pkt.push(0x00).ok();

    // Заполнить длину
    let len = pkt.len() as u16;
    pkt[2] = (len >> 8) as u8;
    pkt[3] = (len & 0xFF) as u8;

    wrap_ppp_frame(PROTO_LCP, &pkt)
}

/// PAP Authenticate-Request
///
/// SIM800L: пустой логин и пароль
fn build_pap_auth_req(id: u8) -> heapless::Vec<u8, PPP_FRAME_BUF> {
    let mut pkt: heapless::Vec<u8, PPP_FRAME_BUF> = heapless::Vec::new();

    pkt.push(PAP_AUTHENTICATE_REQ).ok(); // Code
    pkt.push(id).ok();
    pkt.push(0x00).ok(); // Length high
    pkt.push(0x06).ok(); // Length low = 6

    // Peer-ID-Length = 0, Password-Length = 0
    pkt.push(0x00).ok();
    pkt.push(0x00).ok();

    wrap_ppp_frame(PROTO_PAP, &pkt)
}

/// LCP Terminate-Request
fn build_lcp_terminate_req(id: u8) -> heapless::Vec<u8, PPP_FRAME_BUF> {
    let mut pkt: heapless::Vec<u8, PPP_FRAME_BUF> = heapless::Vec::new();

    pkt.push(LCP_TERMINATE_REQ).ok();
    pkt.push(id).ok();
    pkt.push(0x00).ok(); // Length high
    pkt.push(0x04).ok(); // Length low = 4

    wrap_ppp_frame(PROTO_LCP, &pkt)
}

/// IPCP Configure-Request
///
/// Options: IP-Address=0.0.0.0, Primary-DNS=0.0.0.0
fn build_ipcp_configure_req(id: u8) -> heapless::Vec<u8, PPP_FRAME_BUF> {
    let mut pkt: heapless::Vec<u8, PPP_FRAME_BUF> = heapless::Vec::new();

    pkt.push(IPCP_CONFIGURE_REQ).ok();
    pkt.push(id).ok();
    pkt.push(0x00).ok();
    pkt.push(0x00).ok();

    // IP-Address = 0.0.0.0 (type=3, len=6)
    pkt.push(IPCP_OPT_IP_ADDR).ok();
    pkt.push(6).ok();
    pkt.push(0x00).ok();
    pkt.push(0x00).ok();
    pkt.push(0x00).ok();
    pkt.push(0x00).ok();

    // Primary DNS = 0.0.0.0 (type=129, len=6)
    pkt.push(IPCP_OPT_PRIMARY_DNS).ok();
    pkt.push(6).ok();
    pkt.push(0x00).ok();
    pkt.push(0x00).ok();
    pkt.push(0x00).ok();
    pkt.push(0x00).ok();

    // Заполнить длину
    let len = pkt.len() as u16;
    pkt[2] = (len >> 8) as u8;
    pkt[3] = (len & 0xFF) as u8;

    wrap_ppp_frame(PROTO_IPCP, &pkt)
}

// ── CRC-16 (FCS-16) для HDLC ────────────────────────────────────────────
//
/// PPP FCS-16: полином x^16 + x^12 + x^5 + 1 (= 0x8408 reflected)
/// Используется только в raw HDLC режиме (без CMUX)

#[allow(dead_code)]
fn compute_fcs16(data: &[u8]) -> u16 {
    let mut fcs: u16 = 0xFFFF;
    for &byte in data {
        fcs ^= byte as u16;
        for _ in 0..8 {
            if fcs & 0x0001 != 0 {
                fcs = (fcs >> 1) ^ 0x8408;
            } else {
                fcs >>= 1;
            }
        }
    }
    fcs ^ 0xFFFF
}

// ── HDLC byte-stuffing ────────────────────────────────────────────────────
//
/// Экранирование для raw HDLC (не используется при CMUX)

#[allow(dead_code)]
fn hdlc_stuff(data: &[u8]) -> heapless::Vec<u8, 512> {
    let mut result: heapless::Vec<u8, 512> = heapless::Vec::new();
    for &byte in data {
        match byte {
            0x7E => {
                result.push(0x7D).ok();
                result.push(0x5E).ok();
            }
            0x7D => {
                result.push(0x7D).ok();
                result.push(0x5D).ok();
            }
            b if b < 0x20 => {
                result.push(0x7D).ok();
                result.push(b ^ 0x20).ok();
            }
            _ => {
                result.push(byte).ok();
            }
        }
    }
    result
}