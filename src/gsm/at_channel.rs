//! AT command channel поверх CMUX DLC1 — интеграция с atat
//!
//! Обёртка CMUX DLC1 → Read + Write для atat crate

use embedded_io_async::{Error, Read, Write};

use crate::gsm::cmux;

// ── CmuxAtChannel — Read + Write для atat ────────────────────────────────

/// Обёртка CMUX DLC1, реализующая embedded_io_async Read + Write
pub struct CmuxAtChannel {
    dlci: u8,
    tx_buffer: heapless::Vec<u8, 256>,
}

/// Ошибка канала AT команд
#[derive(Debug, Clone, Copy, defmt::Format)]
pub struct ChannelError;

impl embedded_io_async::Error for ChannelError {
    fn kind(&self) -> embedded_io_async::ErrorKind {
        embedded_io_async::ErrorKind::Other
    }
}

impl CmuxAtChannel {
    /// Создать новый AT канал для указанного DLCI
    pub fn new(dlci: u8) -> Self {
        Self {
            dlci,
            tx_buffer: heapless::Vec::new(),
        }
    }

    /// Отправить AT команду через CMUX
    pub async fn send_at_cmd(&mut self, cmd: &str) -> Result<(), ChannelError> {
        let frame = cmux::encode_cmux_frame(self.dlci, cmux::UIH, cmd.as_bytes());
        // TODO: записать frame в UART TX
        let _ = frame; // заглушка
        Ok(())
    }
}

impl embedded_io_async::ErrorType for CmuxAtChannel {
    type Error = ChannelError;
}

impl Write for CmuxAtChannel {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, ChannelError> {
        // Инкапсулируем buf в CMUX frame для DLC1 и отправляем в UART
        let frame = cmux::encode_cmux_frame(self.dlci, cmux::UIH, buf);
        // TODO: записать frame в UART TX (через общий UART writer)
        let _ = frame; // заглушка
        Ok(buf.len())
    }
}

impl Read for CmuxAtChannel {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, ChannelError> {
        // TODO: читать из CMUX DLC1 receiver channel
        // Когда CMUX декодер получит кадр для DLC1,
        // данные будут доступны здесь
        // Пока — заглушка (никогда не вернёт данные)
        embassy_time::Timer::after_secs(1).await;
        Ok(0)
    }
}

// ── GsmAtClient — обёртка над atat ───────────────────────────────────────

/// AT клиент для GSM модема
/// Использует atat crate для парсинга AT команд
pub struct GsmAtClient {
    /// Канал AT команд через CMUX
    pub channel: CmuxAtChannel,
}

impl GsmAtClient {
    /// Создать новый AT клиент
    pub fn new(channel: CmuxAtChannel) -> Self {
        Self { channel }
    }

    /// Отправить простую AT команду и дождаться OK
    pub async fn send_simple(&mut self, cmd: &str) -> Result<(), ChannelError> {
        self.channel.send_at_cmd(cmd).await
    }
}