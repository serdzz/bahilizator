//! SMS send/receive — AT команды для SIM800L
//!
//! Ручное формирование AT команд (без atat derive — слишком тяжёлый dependency)
//! atat используется только для парсинга URC responses

use heapless::String;

use crate::state::SMS_BUF_SIZE;

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

/// Префикс входящего SMS
pub const URC_SMS: &str = "+CMT:";
/// Префикс готовности модема
pub const URC_CALL_READY: &str = "Call Ready";
/// Префикс отключения питания
pub const URC_POWER_DOWN: &str = "NORMAL POWER DOWN";
/// Префикс низкого напряжения
pub const URC_UNDER_VOLTAGE: &str = "UNDER-VOLTAGE";
/// Префикс высокого напряжения
pub const URC_OVER_VOLTAGE: &str = "OVER-VOLTAGE";

// ── Функции отправки SMS ─────────────────────────────────────────────────

/// Отправить SMS с текстом
/// В текстовом режиме: сначала AT+CMGS="<number>", ждём ">", затем текст + Ctrl+Z
pub async fn send_sms_text(
    channel: &mut crate::gsm::at_channel::GsmAtClient,
    number: &str,
    text: &str,
) -> Result<(), crate::error::GsmError> {
    // Шаг 1: AT+CMGS="number"
    let cmd = cmd_cmgs(number);
    channel.send_simple(&cmd).await.map_err(|_| crate::error::GsmError::UartError)?;

    // Шаг 2: Дождаться ">" prompt
    embassy_time::Timer::after_millis(500).await;

    // Шаг 3: Отправить текст + Ctrl+Z (0x1A)
    let mut msg = String::<{ SMS_BUF_SIZE + 1 }>::new();
    use core::fmt::Write;
    let _ = write!(msg, "{}", text);
    // Ctrl+Z
    channel.send_simple(&msg).await.map_err(|_| crate::error::GsmError::UartError)?;

    Ok(())
}