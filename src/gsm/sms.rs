//! SMS send/receive — AT команды для SIM800L
//!
//! Ручное формирование AT команд (без atat derive)
//! Перенос из gsm.c: gsmSendSms(), waitForPrompt()
//!
//! SIM800L SMS flow:
//!   1. AT+CMGF=1 (text mode) — уже при инициализации
//!   2. AT+CMGS="<number>" → ждём ">" prompt
//!   3. Отправляем текст + Ctrl+Z (0x1A)
//!   4. Ждём +CMGS: <id> → OK

use heapless::String;

// ── AT команды — строковое представление ──────────────────────────────────

/// Построить команду установки формата SMS
pub fn cmd_cmgf(mode: u8) -> String<16> {
    let mut s = String::new();
    use core::fmt::Write;
    let _ = write!(s, "AT+CMGF={}\r", mode);
    s
}

/// Построить команду настройки CNMI
pub fn cmd_cnmi(mode: u8, mt: u8, bm: u8, ds: u8, bfr: u8) -> String<32> {
    let mut s = String::new();
    use core::fmt::Write;
    let _ = write!(s, "AT+CNMI={},{},{},{},{}\r", mode, mt, bm, ds, bfr);
    s
}

/// Построить команду CSMP
pub fn cmd_csmp(fo: u8, vp: u8, pid: u8, dcs: u8) -> String<32> {
    let mut s = String::new();
    use core::fmt::Write;
    let _ = write!(s, "AT+CSMP={},{},{},{}\r", fo, vp, pid, dcs);
    s
}

/// Построить команду отправки SMS
pub fn cmd_cmgs(number: &str) -> String<32> {
    let mut s = String::new();
    use core::fmt::Write;
    let _ = write!(s, "AT+CMGS=\"{}\"\r", number);
    s
}

/// Построить команду чтения SMS
pub fn cmd_cmgr(index: u8) -> String<16> {
    let mut s = String::new();
    use core::fmt::Write;
    let _ = write!(s, "AT+CMGR={}\r", index);
    s
}

/// Построить команду удаления SMS
pub fn cmd_cmgd(index: u8) -> String<16> {
    let mut s = String::new();
    use core::fmt::Write;
    let _ = write!(s, "AT+CMGD={}\r", index);
    s
}

/// Выключить эхо
pub fn cmd_echo_off() -> &'static str {
    "ATE0\r"
}

/// Запрос времени модема
pub fn cmd_cclk() -> &'static str {
    "AT+CCLK?\r"
}

/// Включить CMUX
pub fn cmd_cmux() -> &'static str {
    "AT+CMUX=0,1,5,128,10,3,30,10,2\r"
}

/// Мягкое выключение модема
pub fn cmd_power_down() -> &'static str {
    "AT+CPOWD=1\r"
}

/// Набрать PPP
pub fn cmd_dial_ppp() -> &'static str {
    "ATD*99***1#\r"
}

// ── URC — префиксы входящих событий от модема ────────────────────────────

pub const URC_SMS: &str = "+CMT:";
pub const URC_CALL_READY: &str = "Call Ready";
pub const URC_POWER_DOWN: &str = "NORMAL POWER DOWN";
pub const URC_UNDER_VOLTAGE: &str = "UNDER-VOLTAGE";
pub const URC_OVER_VOLTAGE: &str = "OVER-VOLTAGE";

// ── Функции отправки SMS ─────────────────────────────────────────────────
//
/// Отправить SMS с текстом через GsmAtClient
///
/// Шаг 1: AT+CMGS="number" → CMUX frame DLC1
/// Шаг 2: Дождаться ">" prompt
/// Шаг 3: Отправить текст + Ctrl+Z (0x1A) → CMUX frame DLC1
///
/// В оригинале (gsm.c): gsmSendSms() + waitForPrompt() + gsmWriteMessage()
pub async fn send_sms_text(
    channel: &mut crate::gsm::at_channel::CmuxAtChannel,
    number: &str,
    text: &str,
) -> Result<(), crate::error::GsmError> {
    // Шаг 1: AT+CMGS="number"
    let cmd = cmd_cmgs(number);
    let frame = channel.encode_at_cmd(&cmd);
    crate::gsm::uart_write(&frame).await.ok();

    // Шаг 2: Дождаться ">" prompt
    // В оригинале: waitForPrompt(250) — 250мс таймаут
    embassy_time::Timer::after_millis(500).await;

    // Шаг 3: Отправить текст + Ctrl+Z
    let mut msg_bytes = heapless::Vec::<u8, 200>::new();
    msg_bytes
        .extend_from_slice(text.as_bytes())
        .map_err(|_| crate::error::GsmError::UartError)?;
    msg_bytes
        .push(0x1A)
        .map_err(|_| crate::error::GsmError::UartError)?; // Ctrl+Z

    let frame =
        crate::gsm::cmux::encode_cmux_frame(channel.dlci, crate::gsm::cmux::UIH, &msg_bytes);
    crate::gsm::uart_write(&frame).await.ok();

    Ok(())
}
