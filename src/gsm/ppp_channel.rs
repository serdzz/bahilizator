//! PPP over CMUX DLC2 — заготовка
//!
//! SIM800L: ATD*99***1# → CONNECT → PPP negotiation → IP
//! STM32F103 не имеет hardware PPP — минимальный software PPP

use crate::error::PppError;

// ── Состояние PPP ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
enum PppState {
    Closed,
    LcpNegotiating,
    AuthPap,
    IpcpNegotiating,
    Up,
}

// ── PppChannel ───────────────────────────────────────────────────────────

/// Software PPP поверх CMUX DLC2
pub struct PppChannel {
    dlci: u8,
    state: PppState,
    ip_addr: Option<[u8; 4]>,
}

impl PppChannel {
    /// Создать новый PPP канал для DLC2
    pub fn new() -> Self {
        Self {
            dlci: 2,
            state: PppState::Closed,
            ip_addr: None,
        }
    }

    /// Инициировать PPP соединение
    /// ATD*99***1# должен быть отправлен через at_channel до вызова
    pub async fn connect(&mut self) -> Result<(), PppError> {
        self.state = PppState::LcpNegotiating;

        // 1. LCP negotiation на DLC2
        self.send_lcp_configure_request().await?;
        self.wait_lcp_ack().await?;

        // 2. PAP authentication
        self.state = PppState::AuthPap;
        self.send_pap_auth_request().await?;
        self.wait_pap_ack().await?;

        // 3. IPCP negotiation → получаем IP
        self.state = PppState::IpcpNegotiating;
        self.send_ipcp_configure_request().await?;
        self.wait_ipcp_ack().await?;

        self.state = PppState::Up;
        Ok(())
    }

    /// Отключить PPP
    pub async fn disconnect(&mut self) {
        // Отправить LCP Terminate-Request
        self.state = PppState::Closed;
        self.ip_addr = None;
    }

    /// PPP поднят?
    pub fn is_up(&self) -> bool {
        matches!(self.state, PppState::Up)
    }

    /// Получить IP адрес
    pub fn ip_addr(&self) -> Option<[u8; 4]> {
        self.ip_addr
    }

    /// Отправить IP пакет (после установления PPP)
    pub async fn send_ip_packet(&mut self, packet: &[u8]) -> Result<(), PppError> {
        if !self.is_up() {
            return Err(PppError::NotConnected);
        }

        // Инкапсулировать в CMUX frame DLC2, добавить PPP protocol field
        // 0x0021 = IP protocol
        let mut ppp_frame = heapless::Vec::<u8, 300>::new();
        ppp_frame.push(0x00).ok(); // High byte of protocol
        ppp_frame.push(0x21).ok(); // Low byte = IP
        ppp_frame.extend_from_slice(packet).ok();

        let _cmux_frame = crate::gsm::cmux::encode_cmux_frame(self.dlci, crate::gsm::cmux::UIH, &ppp_frame);
        // TODO: записать cmux_frame в UART

        Ok(())
    }

    // ── PPP LCP ──────────────────────────────────────────────────────

    async fn send_lcp_configure_request(&mut self) -> Result<(), PppError> {
        // LCP Configure-Request: code=1, id=0, len=...
        // Поля: MRU, Authentication-Protocol, Magic-Number
        let _lcp_packet = self.build_lcp_configure_req(0);
        // TODO: отправить через CMUX DLC2
        Ok(())
    }

    async fn wait_lcp_ack(&mut self) -> Result<(), PppError> {
        // TODO: ждать LCP Configure-Ack от модема
        embassy_time::Timer::after_secs(5).await;
        Ok(())
    }

    // ── PPP PAP ──────────────────────────────────────────────────────

    async fn send_pap_auth_request(&mut self) -> Result<(), PppError> {
        // PAP Authenticate-Request: code=1, id=1
        // Peer-ID + Password
        let _pap_packet = self.build_pap_auth_req();
        // TODO: отправить через CMUX DLC2
        Ok(())
    }

    async fn wait_pap_ack(&mut self) -> Result<(), PppError> {
        // TODO: ждать PAP Authenticate-Ack
        embassy_time::Timer::after_secs(5).await;
        Ok(())
    }

    // ── PPP IPCP ──────────────────────────────────────────────────────

    async fn send_ipcp_configure_request(&mut self) -> Result<(), PppError> {
        // IPCP Configure-Request: code=1, id=2
        // Поля: IP-Address, DNS
        let _ipcp_packet = self.build_ipcp_configure_req();
        // TODO: отправить через CMUX DLC2
        Ok(())
    }

    async fn wait_ipcp_ack(&mut self) -> Result<(), PppError> {
        // TODO: ждать IPCP Configure-Ack, извлечь IP
        embassy_time::Timer::after_secs(5).await;
        // Заглушка: присваиваем IP
        self.ip_addr = Some([10, 0, 0, 1]);
        Ok(())
    }

    // ── Построение PPP пакетов ───────────────────────────────────────

    fn build_lcp_configure_req(&self, id: u8) -> [u8; 20] {
        // LCP Configure-Request
        // Code=1, ID=id, Length=N
        // Options: MRU=128 (1), Auth=PAP (3), Magic-Number (5)
        let mut pkt = [0u8; 20];
        pkt[0] = 0x01; // Code: Configure-Request
        pkt[1] = id;
        pkt[2] = 0x00; // Length high
        pkt[3] = 0x14; // Length low = 20
        // MRU option: type=1, len=4, value=128
        pkt[4] = 0x01; pkt[5] = 0x04; pkt[6] = 0x00; pkt[7] = 0x80;
        // Auth Protocol: type=3, len=4, PAP=0xC023
        pkt[8] = 0x03; pkt[9] = 0x04; pkt[10] = 0xC0; pkt[11] = 0x23;
        // Magic Number: type=5, len=6, value=random
        pkt[12] = 0x05; pkt[13] = 0x06; pkt[14] = 0x12; pkt[15] = 0x34;
        pkt[16] = 0x56; pkt[17] = 0x78;
        // Padding
        pkt[18] = 0x00; pkt[19] = 0x00;
        pkt
    }

    fn build_pap_auth_req(&self) -> [u8; 20] {
        // PAP Authenticate-Request
        // Code=1, ID=1, Length=N
        // Peer-ID-Length + Peer-ID + Password-Length + Password
        let mut pkt = [0u8; 20];
        pkt[0] = 0x01; // Code: Authenticate-Request
        pkt[1] = 0x01; // ID
        pkt[2] = 0x00;
        pkt[3] = 0x0A; // Length = 10
        pkt[4] = 0x00; // Peer-ID length = 0 (empty)
        pkt[5] = 0x00; // Password length = 0 (empty)
        pkt
    }

    fn build_ipcp_configure_req(&self) -> [u8; 16] {
        // IPCP Configure-Request
        // Code=1, ID=2, Length=N
        // Option: IP-Address type=3
        let mut pkt = [0u8; 16];
        pkt[0] = 0x01; // Code: Configure-Request
        pkt[1] = 0x02; // ID
        pkt[2] = 0x00;
        pkt[3] = 0x0A; // Length = 10
        // IP Address: type=3, len=6, value=0.0.0.0 (request)
        pkt[4] = 0x03; pkt[5] = 0x06; pkt[6] = 0x00; pkt[7] = 0x00;
        pkt[8] = 0x00; pkt[9] = 0x00;
        // Primary DNS: type=129, len=6
        pkt[10] = 0x81; pkt[11] = 0x06; pkt[12] = 0x00; pkt[13] = 0x00;
        pkt[14] = 0x00; pkt[15] = 0x00;
        pkt
    }
}