//! GSM менеджер — инициализация, питание, CMUX, GPRS
//!
//! SIM800L модем через USART1. Режим CMUX для разделения
//! AT команд (DLC1) и PPP данных (DLC2).
//!
//! Перенос из gsm.c:
//!   - gsmHardwarePowerUp/Down — управление питанием через PWRKEY
//!   - initGSM_cmd — последовательность AT команд
//!   - gsmProcess — парсинг ответов модема
//!   - CMUX: AT+CMUX=0 → мультиплексирование каналов
//!
//! Жизненный цикл модема:
//!   1. Power on: PWRKEY pulse (1с high → 3с low → проверка STATUS)
//!   2. Init: ATE0 → CMGF=1 → CNMI → CSMP → CCLK?
//!   3. CMUX: AT+CMUX=0,1,5,128,10,3,30,10,2 → SABM DLC0/1/2
//!   4. Работа: AT команды через DLC1, PPP через DLC2
//!   5. Power off: AT+CPOWD=1 или PWRKEY pulse

pub mod at_channel;
pub mod cmux;
pub mod dns;
pub mod mqtt;
pub mod mqtt_topics;
pub mod ppp_channel;
pub mod sms;

use embassy_sync::channel::{Receiver, Sender};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;

use crate::state::VendingState;
use crate::error::GsmError;

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
    /// Отправить SMS на телефон с индексом phone_idx
    SendSms { phone_idx: u8, kind: crate::state::MessageKind },
    /// Запросить время у модема
    GetTime,
    /// Проверить GPRS подключение
    CheckGprs,
    /// Включить модем
    PowerOn,
    /// Выключить модем
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
    /// Модем проинициализирован и готов
    pub ready: bool,
    /// Call Ready получен
    pub call_ready: bool,
    /// GPRS подключён
    pub gprs_attached: bool,
    /// IP адрес (если GPRS подключён)
    pub ip_addr: Option<[u8; 4]>,
    /// Этап инициализации
    pub init_stage: GsmInitStage,
    /// Код последней ошибки CME
    pub cme_error: u16,
    /// Код последней ошибки CMS
    pub cms_error: u16,
    /// Отправка SMS в процессе
    pub sending_sms: bool,
    /// SMS отправлен успешно
    pub sms_sent: bool,
    /// ID последнего отправленного SMS
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

/// Таймаут ожидания ответа от модема (мс)
const GSM_RESPONSE_TIMEOUT_MS: u64 = 5000;
/// Таймаут ожидания Call Ready (с)
const GSM_CALL_READY_TIMEOUT_S: u64 = 15;
/// Таймаут инициализации модема (с)
const GSM_INIT_TIMEOUT_S: u64 = 30;
/// Задержка между AT командами (мс)
const GSM_CMD_DELAY_MS: u64 = 200;
/// Таймаут отправки SMS (с)
const GSM_SMS_TIMEOUT_S: u64 = 10;
/// Интервал проверки GPRS (с)
const GSM_GPRS_CHECK_INTERVAL_S: u64 = 60;

// ── Управление питанием модема ──────────────────────────────────────────

/// Включить модем SIM800L через PWRKEY
///
/// Последовательность (из даташита SIM800L):
///   1. PWRKEY = HIGH (подтяжка)
///   2. PWRKEY → LOW на 1с (минимум 100мс)
///   3. PWRKEY → HIGH
///   4. Ждём 3с
///   5. Проверяем STATUS pin = HIGH
///
/// В оригинале: gsmHardwarePowerUp()
async fn gsm_power_on() -> Result<(), GsmError> {
    // Шаг 1: Проверяем STATUS — если уже включён, пропускаем
    if gsm_check_status() {
        return Ok(());
    }

    // Шаг 2: PWRKEY pulse
    // В реальном железе:
    // PWRKEY.set_high().ok();
    // embassy_time::Timer::after_millis(100).await;
    // PWRKEY.set_low().ok();
    // embassy_time::Timer::after_secs(1).await;
    // PWRKEY.set_high().ok();
    // embassy_time::Timer::after_secs(3).await;

    // Safety: заглушка — в интеграции заменяется на GPIO
    embassy_time::Timer::after_secs(4).await;

    // Шаг 3: Проверяем STATUS
    if !gsm_check_status() {
        // Пробуем ещё раз
        embassy_time::Timer::after_secs(2).await;
        if !gsm_check_status() {
            return Err(GsmError::NoResponse);
        }
    }

    Ok(())
}

/// Выключить модем SIM800L через PWRKEY
///
/// Последовательность:
///   1. Отправить AT+CPOWD=1 (мягкое выключение)
///   2. Если нет ответа — PWRKEY pulse
///   3. STATUS → LOW
///
/// В оригинале: gsmHardwarePowerDown()
async fn gsm_power_off() {
    // Мягкое выключение через AT команду
    // В реальном железе:
    // uart.write(b"AT+CPOWD=1\r\n");
    // embassy_time::Timer::after_secs(3).await;

    // Если модем не ответил — hardware power down
    if gsm_check_status() {
        // PWRKEY pulse (как при включении)
        embassy_time::Timer::after_secs(1).await;
    }

    // Safety: заглушка
}

/// Проверить STATUS pin модема
///
/// STATUS = HIGH → модем включён
/// STATUS = LOW → модем выключен
fn gsm_check_status() -> bool {
    // В реальном железе:
    // STATUS_PIN.is_high()
    // Safety: заглушка — предполагаем модем включён
    true
}

// ── Инициализация модема ────────────────────────────────────────────────

/// Отправить AT команду в UART и дождаться OK
///
/// В оригинале: gsmWriteCommand() + gswWaitForOK()
async fn send_at_cmd(_cmd: &str) -> Result<AtResponse, GsmError> {
    // В реальном железе:
    // 1. Закодировать в CMUX frame (DLC1) или напрямую в UART
    // 2. Отправить
    // 3. Ждать ответ (OK/ERROR/+CME ERROR)
    // 4. Парсить

    // Safety: заглушка — имитируем успешный ответ
    embassy_time::Timer::after_millis(GSM_CMD_DELAY_MS).await;
    Ok(AtResponse::Ok)
}

/// Ответ на AT команду
#[derive(Debug, Clone, defmt::Format)]
pub enum AtResponse {
    Ok,
    Error,
    CmeError(u16),
    CmsError(u16),
    Prompt,
    Timeout,
}

/// Инициализация модема — последовательность AT команд
///
/// В оригинале (gsm.c): initGSM_cmd() → последовательность
/// ATE0+CMGF=1;+CNMI=2,2,0,0,0;+CSMP=17,167,0,0;+CCLK?
///
/// Мы разбиваем на отдельные шаги для надёжности
async fn gsm_init_sequence(status: &mut GsmStatus) -> Result<(), GsmError> {
    // Шаг 1: ATE0 — выключить эхо
    status.init_stage = GsmInitStage::EchoOff;
    let resp = send_at_cmd("ATE0\r").await?;
    if !matches!(resp, AtResponse::Ok) {
        return Err(GsmError::NoResponse);
    }

    // Шаг 2: AT+CMGF=1 — текстовый режим SMS
    status.init_stage = GsmInitStage::Cmgf;
    let _ = send_at_cmd("AT+CMGF=1\r").await;

    // Шаг 3: AT+CNMI=2,2,0,0,0 — прямая доставка SMS
    status.init_stage = GsmInitStage::Cnmi;
    let _ = send_at_cmd("AT+CNMI=2,2,0,0,0\r").await;

    // Шаг 4: AT+CSMP=17,167,0,0 — параметры SMS
    status.init_stage = GsmInitStage::Csmp;
    let _ = send_at_cmd("AT+CSMP=17,167,0,0\r").await;

    // Шаг 5: AT+CCLK? — запросить время модема
    status.init_stage = GsmInitStage::Cclk;
    let _ = send_at_cmd("AT+CCLK?\r").await;

    Ok(())
}

/// Включить CMUX мультиплексор
///
/// В оригинале: AT+CMUX=0 → затем SABM на каждый DLCI
async fn gsm_init_cmux(status: &mut GsmStatus) -> Result<(), GsmError> {
    // Включаем CMUX (Basic Mode)
    status.init_stage = GsmInitStage::CmuxEnable;
    send_at_cmd("AT+CMUX=0,1,5,128,10,3,30,10,2\r").await?;

    // Ждём переключения в CMUX режим
    embassy_time::Timer::after_millis(200).await;

    // Устанавливаем каналы SABM
    status.init_stage = GsmInitStage::CmuxDlc0;
    // В реальном железе: отправить SABM DLC0 через UART, ждать UA
    embassy_time::Timer::after_millis(100).await;

    status.init_stage = GsmInitStage::CmuxDlc1;
    embassy_time::Timer::after_millis(100).await;

    status.init_stage = GsmInitStage::CmuxDlc2;
    embassy_time::Timer::after_millis(100).await;

    Ok(())
}

/// Периодическая проверка GPRS
///
/// В оригинале: AT+CGATT? → AT+CSTT → AT+CIICR → AT+CIFSR
async fn gsm_check_gprs(status: &mut GsmStatus) -> bool {
    // AT+CGATT? — проверяем GPRS attach
    let resp = send_at_cmd("AT+CGATT?\r").await;
    if matches!(resp, Ok(AtResponse::Ok)) {
        // AT+CSTT — установить APN
        let _ = send_at_cmd("AT+CSTT=\"internet\"\r").await;

        // AT+CIICR — поднять GPRS
        let _ = send_at_cmd("AT+CIICR\r").await;

        // AT+CIFSR — получить IP
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
        // Включить CMUX
        let _ = gsm_init_cmux(&mut status).await;
        status.init_stage = GsmInitStage::Ready;
        status.ready = true;
        status.call_ready = true;
    }

    loop {
        // Обработка команд
        match gsm_cmd_rx.try_receive() {
            Ok(GsmCommand::SendSms { phone_idx, kind }) => {
                let phone = {
                    let guard = state.lock().await;
                    let st = guard.borrow();
                    // Извлекаем номер телефона из settings
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

                    // Отправляем SMS
                    let _ = send_at_cmd(&format_sms_cmd(&phone)).await;
                    embassy_time::Timer::after_millis(500).await;

                    // Шлём текст + Ctrl+Z
                    // В реальном железе: uart.write(text + 0x1A)
                    let _ = text;
                    status.sending_sms = false;
                    status.sms_sent = true;

                    embassy_time::Timer::after_millis(GSM_CMD_DELAY_MS).await;
                }

                let _ = (phone_idx, sms_event_tx.clone());
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

        // Периодическая проверка GPRS
        if status.ready && last_gprs_check.elapsed().as_secs() >= GSM_GPRS_CHECK_INTERVAL_S {
            if !status.gprs_attached {
                gsm_check_gprs(&mut status);
            }
            last_gprs_check = embassy_time::Instant::now();
        }

        embassy_time::Timer::after_millis(10).await;
    }
}

// ── Вспомогательные функции ────────────────────────────────────────────

/// Извлечь номер телефона из State по индексу
fn extract_phone_number(state: &VendingState, idx: u8) -> heapless::String<17> {
    let mut result = heapless::String::new();
    // phone_numbers: [[[u8; 16]; 2]; 2]
    let level = (idx as usize) / crate::state::PHONES_PER_LEVEL;
    let sub = (idx as usize) % crate::state::PHONES_PER_LEVEL;

    if level < crate::state::PHONE_ACCESS_LEVELS && sub < crate::state::PHONES_PER_LEVEL {
        let raw = &state.settings.phone_numbers[level][sub];
        // Конвертируем байты в строку (до первого нулевого)
        for &b in raw.iter() {
            if b == 0 {
                break;
            }
            if b >= b'0' && b <= b'9' || b == b'+' {
                result.push(b as char).ok();
            }
        }
    }
    result
}

/// Сформировать AT+CMGS команду
fn format_sms_cmd(phone: &str) -> heapless::String<32> {
    let mut s = heapless::String::new();
    use core::fmt::Write;
    let _ = write!(s, "AT+CMGS=\"{}\"\r", phone);
    s
}