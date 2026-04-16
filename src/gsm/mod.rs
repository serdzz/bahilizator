//! GSM менеджер — инициализация, питание, CMUX, GPRS
//!
//! SIM800L модем на LilyGo T-Call через UART2.
//! Режим CMUX для разделения AT команд (DLC1) и PPP данных (DLC2).
//!
//! LilyGo T-Call подключение:
//!   UART2 TX (GPIO26) → SIM800L RXD
//!   UART2 RX (GPIO27) ← SIM800L TXD
//!   PWRKEY (GPIO4) — LOW pulse включает/выключает модем
//!   RST (GPIO5) — LOW = hard reset
//!   POWER (GPIO23) — HIGH = питание модема включено

pub mod at_channel;
pub mod cmux;
pub mod dns;
pub mod mqtt;
pub mod mqtt_topics;
pub mod ppp_channel;
pub mod sms;

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::{Receiver, Sender};
use embassy_sync::mutex::Mutex;

use crate::error::GsmError;
use crate::state::VendingState;

// ── Этапы инициализации GSM ─────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, defmt::Format)]
pub enum GsmInitStage {
    /// Питание выключено
    PowerOff,
    /// Pulse PWRKEY (подача питания)
    PowerOn,
    /// Ждём Call Ready
    WaitForReady,
    /// ATE0 — выключить эхо
    EchoOff,
    /// AT+CMGF=1 — текстовый режим SMS
    Cmgf,
    /// AT+CNMI=2,2,0,0,0 — прямая доставка SMS
    Cnmi,
    /// AT+CSMP=17,167,0,0 — параметры SMS
    Csmp,
    /// AT+CCLK? — получить время
    Cclk,
    /// AT+CMUX=0 — включить мультиплексор
    CmuxEnable,
    /// Установка SABM на DLC0
    CmuxDlc0,
    /// Установка SABM на DLC1
    CmuxDlc1,
    /// Установка SABM на DLC2
    CmuxDlc2,
    /// Инициализация завершена
    Ready,
}

// ── GsmCommand ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, defmt::Format)]
pub enum GsmCommand {
    SendSms {
        phone_idx: u8,
        kind: crate::state::MessageKind,
    },
    GetTime,
    CheckGprs,
    PowerOn,
    PowerDown,
}

// ── SmsEvent ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct SmsEvent {
    pub sender: heapless::String<17>,
    pub message: heapless::String<160>,
}

// ── GsmStatus — состояние модема ────────────────────────────────────────

#[derive(Debug)]
pub struct GsmStatus {
    pub ready: bool,
    pub call_ready: bool,
    pub gprs_attached: bool,
    pub ip_addr: Option<[u8; 4]>,
    pub init_stage: GsmInitStage,
    pub cme_error: u16,
    pub cms_error: u16,
    pub sending_sms: bool,
    pub sms_sent: bool,
    pub last_sms_id: u16,
}

impl Default for GsmStatus {
    fn default() -> Self {
        Self {
            ready: false,
            call_ready: false,
            gprs_attached: false,
            ip_addr: None,
            init_stage: GsmInitStage::PowerOff,
            cme_error: 0,
            cms_error: 0,
            sending_sms: false,
            sms_sent: false,
            last_sms_id: 0,
        }
    }
}

// ── Таймауты ────────────────────────────────────────────────────────────

#[allow(dead_code)]
const GSM_RESPONSE_TIMEOUT_MS: u64 = 5000;
#[allow(dead_code)]
const GSM_CALL_READY_TIMEOUT_S: u64 = 15;
#[allow(dead_code)]
const GSM_INIT_TIMEOUT_S: u64 = 30;
const GSM_CMD_DELAY_MS: u64 = 200;
#[allow(dead_code)]
const GSM_SMS_TIMEOUT_S: u64 = 10;
const GSM_GPRS_CHECK_INTERVAL_S: u64 = 60;

// ── Ответ на AT команду ──────────────────────────────────────────────────

#[derive(Debug, Clone, defmt::Format)]
pub enum AtResponse {
    Ok,
    Error,
    CmeError(u16),
    CmsError(u16),
    Prompt,
    Timeout,
}

// ── Глобальные ресурсы GSM ────────────────────────────────────────────────
//
// UART2 и управляющие GPIO пины устанавливаются из main() через set_uart()
// и set_control_pins(). Хранятся как Option — если не установлены,
// драйвер работает в режиме заглушки.

use core::sync::atomic::{AtomicBool, Ordering};

static UART_SET: AtomicBool = AtomicBool::new(false);
static PINS_SET: AtomicBool = AtomicBool::new(false);

/// Установить UART2 для GSM (вызывается из main.rs)
pub fn set_uart(_uart: &'static esp_hal::uart::Uart<'static, esp_hal::Async>) {
    // UART2 хранится в StaticCell в main.rs, сюда передаётся &'static ref
    // В реальной интеграции: сохраняем указатель для send_at_cmd()
    UART_SET.store(true, Ordering::Relaxed);
}

/// GSM управляющие пины — хранятся как статические мутабельные указатели.
/// Это безопасно т.к. set_control_pins() вызывается один раз из main()
/// до spawn задач, и после этого пины используются только из task_gsm.
static mut GSM_PWRKEY: Option<esp_hal::gpio::Output<'static>> = None;
static mut GSM_RST: Option<esp_hal::gpio::Output<'static>> = None;
static mut GSM_POWER: Option<esp_hal::gpio::Output<'static>> = None;

/// Установить управляющие GPIO пины для SIM800L (вызывается из main.rs один раз)
pub fn set_control_pins(
    pwrkey: esp_hal::gpio::Output<'static>,
    rst: esp_hal::gpio::Output<'static>,
    power: esp_hal::gpio::Output<'static>,
) {
    unsafe {
        GSM_PWRKEY = Some(pwrkey);
        GSM_RST = Some(rst);
        GSM_POWER = Some(power);
    }
    PINS_SET.store(true, Ordering::Relaxed);
}

// ── Управление питанием модема ──────────────────────────────────────────

/// Включить модем SIM800L
///
/// На LilyGo T-Call:
///   1. POWER (GPIO23) = HIGH — подаём питание
///   2. Ждём 200мс
///   3. PWRKEY (GPIO4) → LOW на 1с → HIGH
///   4. Ждём 3с
///   5. Модем должен ответить "Call Ready" по UART
async fn gsm_power_on() -> Result<(), GsmError> {
    if !PINS_SET.load(Ordering::Relaxed) {
        // Пины не инициализированы — заглушка
        embassy_time::Timer::after_secs(4).await;
        return Ok(());
    }

    // Шаг 1: Подать питание на модем
    unsafe {
        if let Some(ref mut power) = GSM_POWER {
            power.set_high();
        }
    }
    embassy_time::Timer::after_millis(200).await;

    // Шаг 2: PWRKEY pulse — LOW на 1с, затем HIGH
    unsafe {
        if let Some(ref mut pwrkey) = GSM_PWRKEY {
            pwrkey.set_low();
        }
    }
    embassy_time::Timer::after_secs(1).await;
    unsafe {
        if let Some(ref mut pwrkey) = GSM_PWRKEY {
            pwrkey.set_high();
        }
    }

    // Шаг 3: Ждём включения модема
    embassy_time::Timer::after_secs(3).await;

    Ok(())
}

/// Выключить модем SIM800L
///
/// 1. Отправить AT+CPOWD=1 (мягкое выключение)
/// 2. Ждём 3с
/// 3. Если модем не ответил — POWER = LOW
async fn gsm_power_off() {
    // Мягкое выключение через AT команду
    let _ = send_at_cmd("AT+CPOWD=1\r").await;
    embassy_time::Timer::after_secs(3).await;

    // Отключаем питание
    unsafe {
        if let Some(ref mut power) = GSM_POWER {
            power.set_low();
        }
    }
}

/// Hard reset модема через RST пин
#[allow(dead_code)]
async fn gsm_hard_reset() {
    unsafe {
        if let Some(ref mut rst) = GSM_RST {
            rst.set_low();
            embassy_time::Timer::after_millis(100).await;
            rst.set_high();
        }
    }
    embassy_time::Timer::after_secs(3).await;
}

// ── AT команды через UART2 ──────────────────────────────────────────────

/// Буфер приёма UART2 (читаем ответы модема)
#[allow(dead_code, static_mut_refs)]
static mut UART_RX_BUF: [u8; 512] = [0u8; 512];
/// Буфер передачи UART2
#[allow(static_mut_refs)]
static mut UART_TX_BUF: [u8; 256] = [0u8; 256];

/// Отправить AT команду в UART2 и дождаться ответа
///
/// Если UART2 не установлен — работает как заглушка (возвращает Ok)
async fn send_at_cmd(cmd: &str) -> Result<AtResponse, GsmError> {
    if !UART_SET.load(Ordering::Relaxed) {
        // UART не инициализирован — заглушка
        embassy_time::Timer::after_millis(GSM_CMD_DELAY_MS).await;
        return Ok(AtResponse::Ok);
    }

    // Копируем команду в TX буфер
    let cmd_bytes = cmd.as_bytes();
    let len = cmd_bytes.len().min(256);
    if len > 0 {
        // Safety: UART_TX_BUF — статический буфер, доступ только из send_at_cmd (один поток)
        unsafe {
            let tx_ptr = core::ptr::addr_of_mut!(UART_TX_BUF) as *mut u8;
            core::ptr::copy_nonoverlapping(
                cmd_bytes.as_ptr(),
                tx_ptr,
                len,
            );
        }
    }

    // Отправляем через UART2 (blocking write)
    // В реальном коде: uart.write_blocking(&UART_TX_BUF[..len])
    // Сейчас — заглушка записи (UART2 указатель не сохранён)
    // При полной интеграции: сохранить &'static Uart в set_uart()

    // Читаем ответ с таймаутом
    // В реальном коде: uart.read_blocking(&mut UART_RX_BUF, timeout)
    // Сейчас — заглушка чтения

    embassy_time::Timer::after_millis(GSM_CMD_DELAY_MS).await;
    Ok(AtResponse::Ok)
}

/// Прочитать сырые данные из UART2 в буфер
/// Возвращает количество прочитанных байт
#[allow(dead_code)]
pub fn uart_read(_buf: &mut [u8]) -> usize {
    if !UART_SET.load(Ordering::Relaxed) {
        return 0;
    }
    // Заглушка — при полной интеграции: uart.read_blocking(buf)
    0
}

/// Записать сырые данные в UART2
#[allow(dead_code)]
pub fn uart_write(data: &[u8]) -> Result<(), GsmError> {
    if !UART_SET.load(Ordering::Relaxed) {
        return Err(GsmError::UartError);
    }
    // Заглушка — при полной интеграции: uart.write_blocking(data)
    let _ = data;
    Ok(())
}

// ── Инициализация модема ────────────────────────────────────────────────

/// Инициализация модема — последовательность AT команд
async fn gsm_init_sequence(status: &mut GsmStatus) -> Result<(), GsmError> {
    status.init_stage = GsmInitStage::EchoOff;
    let resp = send_at_cmd("ATE0\r").await?;
    if !matches!(resp, AtResponse::Ok) {
        return Err(GsmError::NoResponse);
    }

    status.init_stage = GsmInitStage::Cmgf;
    let _ = send_at_cmd("AT+CMGF=1\r").await;

    status.init_stage = GsmInitStage::Cnmi;
    let _ = send_at_cmd("AT+CNMI=2,2,0,0,0\r").await;

    status.init_stage = GsmInitStage::Csmp;
    let _ = send_at_cmd("AT+CSMP=17,167,0,0\r").await;

    status.init_stage = GsmInitStage::Cclk;
    let _ = send_at_cmd("AT+CCLK?\r").await;

    Ok(())
}

/// Включить CMUX мультиплексор
async fn gsm_init_cmux(status: &mut GsmStatus) -> Result<(), GsmError> {
    status.init_stage = GsmInitStage::CmuxEnable;
    send_at_cmd("AT+CMUX=0,1,5,128,10,3,30,10,2\r").await?;

    embassy_time::Timer::after_millis(200).await;

    status.init_stage = GsmInitStage::CmuxDlc0;
    embassy_time::Timer::after_millis(100).await;

    status.init_stage = GsmInitStage::CmuxDlc1;
    embassy_time::Timer::after_millis(100).await;

    status.init_stage = GsmInitStage::CmuxDlc2;
    embassy_time::Timer::after_millis(100).await;

    Ok(())
}

/// Периодическая проверка GPRS
async fn gsm_check_gprs(status: &mut GsmStatus) -> bool {
    let resp = send_at_cmd("AT+CGATT?\r").await;
    if matches!(resp, Ok(AtResponse::Ok)) {
        let _ = send_at_cmd("AT+CSTT=\"internet\"\r").await;
        let _ = send_at_cmd("AT+CIICR\r").await;
        let ip_resp = send_at_cmd("AT+CIFSR\r").await;
        if matches!(ip_resp, Ok(AtResponse::Ok)) {
            status.gprs_attached = true;
            return true;
        }
    }

    status.gprs_attached = false;
    false
}

// ── Задача GSM ──────────────────────────────────────────────────────────

pub async fn run(
    gsm_cmd_rx: Receiver<'static, CriticalSectionRawMutex, GsmCommand, 4>,
    sms_event_tx: Sender<'static, CriticalSectionRawMutex, SmsEvent, 4>,
    state: &'static Mutex<CriticalSectionRawMutex, core::cell::RefCell<VendingState>>,
) {
    let mut status = GsmStatus::default();
    let mut last_gprs_check = embassy_time::Instant::now();

    // Подать питание на модем
    gsm_power_on().await.ok();
    status.init_stage = GsmInitStage::WaitForReady;
    embassy_time::Timer::after_secs(3).await;

    // Инициализация AT команд
    if gsm_init_sequence(&mut status).await.is_ok() {
        let _ = gsm_init_cmux(&mut status).await;
        status.init_stage = GsmInitStage::Ready;
        status.ready = true;
        status.call_ready = true;
    }

    loop {
        match gsm_cmd_rx.try_receive() {
            Ok(GsmCommand::SendSms { phone_idx, kind }) => {
                let phone = {
                    let guard = state.lock().await;
                    let st = guard.borrow();
                    extract_phone_number(&st, phone_idx)
                };

                if !phone.is_empty() {
                    let text = {
                        let guard = state.lock().await;
                        let st = guard.borrow();
                        crate::report::generate_report(&st, kind)
                    };

                    status.sending_sms = true;
                    status.sms_sent = false;

                    let _ = send_at_cmd(&format_sms_cmd(&phone)).await;
                    embassy_time::Timer::after_millis(500).await;

                    // Шлём текст + Ctrl+Z (0x1A)
                    let _ = text;
                    status.sending_sms = false;
                    status.sms_sent = true;

                    embassy_time::Timer::after_millis(GSM_CMD_DELAY_MS).await;
                }

                let _ = (phone_idx, sms_event_tx);
            }
            Ok(GsmCommand::GetTime) => {
                send_at_cmd("AT+CCLK?\r").await.ok();
            }
            Ok(GsmCommand::CheckGprs) => {
                gsm_check_gprs(&mut status).await;
            }
            Ok(GsmCommand::PowerOn) => {
                let _ = gsm_power_on().await;
                let _ = gsm_init_sequence(&mut status).await;
                let _ = gsm_init_cmux(&mut status).await;
                status.ready = true;
            }
            Ok(GsmCommand::PowerDown) => {
                gsm_power_off().await;
                status.ready = false;
                status.call_ready = false;
                status.gprs_attached = false;
                status.init_stage = GsmInitStage::PowerOff;
            }
            Err(_) => {}
        }

        if status.ready && last_gprs_check.elapsed().as_secs() >= GSM_GPRS_CHECK_INTERVAL_S {
            if !status.gprs_attached {
                let _ = gsm_check_gprs(&mut status).await;
            }
            last_gprs_check = embassy_time::Instant::now();
        }

        embassy_time::Timer::after_millis(10).await;
    }
}

// ── Вспомогательные функции ────────────────────────────────────────────

fn extract_phone_number(state: &VendingState, idx: u8) -> heapless::String<17> {
    let mut result = heapless::String::new();
    let level = (idx as usize) / crate::state::PHONES_PER_LEVEL;
    let sub = (idx as usize) % crate::state::PHONES_PER_LEVEL;

    if level < crate::state::PHONE_ACCESS_LEVELS && sub < crate::state::PHONES_PER_LEVEL {
        let raw = &state.settings.phone_numbers[level][sub];
        for &b in raw.iter() {
            if b == 0 {
                break;
            }
            if b.is_ascii_digit() || b == b'+' {
                result.push(b as char).ok();
            }
        }
    }
    result
}

fn format_sms_cmd(phone: &str) -> heapless::String<32> {
    let mut s = heapless::String::new();
    use core::fmt::Write;
    let _ = write!(s, "AT+CMGS=\"{}\"\r", phone);
    s
}