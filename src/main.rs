//! Бахилизатор — Rust/Embassy порт для ESP32 DevKit V1 (Xtensa LX6)
//!
//! Перенос с STM32F103C8T6 Bluepill на ESP32
//! Xtensa LX6 dual-core, 520KB SRAM, 4MB Flash, WiFi+BT

#![no_std]
#![no_main]

use esp_backtrace as _; // panic handler
use esp_println as _; // defmt-espflash global logger

defmt::timestamp!(""); // заглушка — пока нет таймера

/// defmt panic handler — использует esp_backtrace для backtrace
#[defmt::panic_handler]
fn panic() -> ! {
    loop {
        core::hint::spin_loop();
    }
}

use embassy_executor::Spawner;
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel, signal::Signal,
};
use static_cell::StaticCell;

use bahilizator::{buttons, coin_acceptor, flash, gsm, hopper, ibutton, nvram, state, ui, vending};

// ── Статические каналы и сигналы ────────────────────────────────────────

static COIN_CHANNEL: Channel<CriticalSectionRawMutex, coin_acceptor::CoinEvent, 4> = Channel::new();
static BUTTON_CHANNEL: Channel<CriticalSectionRawMutex, buttons::ButtonEvent, 4> = Channel::new();
static HOPPER_CMD_CHANNEL: Channel<CriticalSectionRawMutex, hopper::HopperCmd, 4> = Channel::new();
static HOPPER_EVENT_CHANNEL: Channel<CriticalSectionRawMutex, hopper::HopperEvent, 4> =
    Channel::new();
static GSM_CMD_CHANNEL: Channel<CriticalSectionRawMutex, gsm::GsmCommand, 4> = Channel::new();
static SMS_EVENT_CHANNEL: Channel<CriticalSectionRawMutex, gsm::SmsEvent, 4> = Channel::new();
static IBUTTON_CHANNEL: Channel<CriticalSectionRawMutex, ibutton::IbuttonEvent, 1> = Channel::new();

static DISPLAY_SIGNAL: Signal<CriticalSectionRawMutex, ui::DisplayCommand> = Signal::new();
static PERSIST_SIGNAL: Signal<CriticalSectionRawMutex, state::PersistReason> = Signal::new();

// ── Общее состояние ────────────────────────────────────────────────────

static STATE_CELL: StaticCell<
    embassy_sync::mutex::Mutex<CriticalSectionRawMutex, core::cell::RefCell<state::VendingState>>,
> = StaticCell::new();

// ── Embassy tasks ────────────────────────────────────────────────────────

#[embassy_executor::task]
async fn task_vending(
    state: &'static embassy_sync::mutex::Mutex<
        CriticalSectionRawMutex,
        core::cell::RefCell<state::VendingState>,
    >,
) {
    vending::run(
        state,
        COIN_CHANNEL.receiver(),
        BUTTON_CHANNEL.receiver(),
        HOPPER_EVENT_CHANNEL.receiver(),
        HOPPER_CMD_CHANNEL.sender(),
        GSM_CMD_CHANNEL.sender(),
        &DISPLAY_SIGNAL,
        &PERSIST_SIGNAL,
    )
    .await;
}

#[embassy_executor::task]
async fn task_coin_acceptor(
    state: &'static embassy_sync::mutex::Mutex<
        CriticalSectionRawMutex,
        core::cell::RefCell<state::VendingState>,
    >,
) {
    coin_acceptor::run(COIN_CHANNEL.sender(), state).await;
}

#[embassy_executor::task]
async fn task_hopper() {
    hopper::run(HOPPER_CMD_CHANNEL.receiver(), HOPPER_EVENT_CHANNEL.sender()).await;
}

#[embassy_executor::task]
async fn task_buttons() {
    buttons::run(BUTTON_CHANNEL.sender()).await;
}

#[embassy_executor::task]
async fn task_gsm(
    state: &'static embassy_sync::mutex::Mutex<
        CriticalSectionRawMutex,
        core::cell::RefCell<state::VendingState>,
    >,
) {
    gsm::run(
        GSM_CMD_CHANNEL.receiver(),
        SMS_EVENT_CHANNEL.sender(),
        state,
    )
    .await;
}

#[embassy_executor::task]
async fn task_ibutton(
    driver: Option<ibutton::IbuttonDriver>,
    state: &'static embassy_sync::mutex::Mutex<
        CriticalSectionRawMutex,
        core::cell::RefCell<state::VendingState>,
    >,
) {
    ibutton::run(driver, IBUTTON_CHANNEL.sender(), state).await;
}

#[embassy_executor::task]
async fn task_state_persist(
    state: &'static embassy_sync::mutex::Mutex<
        CriticalSectionRawMutex,
        core::cell::RefCell<state::VendingState>,
    >,
) {
    nvram::persist_task(state, &PERSIST_SIGNAL).await;
}

// ── main — точка входа ESP32 ─────────────────────────────────────────────

#[esp_rtos::main]
async fn main(spawner: Spawner) {
    defmt::info!("Бахилизатор v2.0 ESP32 — запуск...");

    // Инициализация ESP32
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // TODO: Настроить периферию через esp_hal::Peripherals
    // I2C: SDA=GPIO21, SCL=GPIO22
    // UART0 (USB): TX=GPIO1, RX=GPIO3 — debug
    // UART2 (GSM): TX=GPIO17, RX=GPIO16
    // Coin CH1-6: GPIO13-18 (через NPN)
    // Coin BLOCK: GPIO19
    // Hopper A: Enable=GPIO25, Sensor=GPIO26
    // Hopper B: Enable=GPIO27, Sensor=GPIO14
    // Buttons: GPIO32-35 (input-only!)
    // Doors: GPIO36, GPIO39 (input-only!)
    // iButton 1-Wire: GPIO4
    // GSM PWRKEY: GPIO5, STATUS: GPIO33 (input-only)
    // LED: GPIO2 (встроенный синий)
    let _ = peripherals;

    // Инициализация состояния
    let state = STATE_CELL.init(embassy_sync::mutex::Mutex::new(core::cell::RefCell::new(
        state::VendingState::default(),
    )));

    // Загрузка из Flash/NVS и EEPROM
    {
        let mut guard = state.lock().await;
        let st = guard.get_mut();
        st.settings = flash::load_settings().unwrap_or_default();
        st.data = nvram::load_state_default();
    }

    // Spawn задач
    let _ = spawner.spawn(task_vending(state));
    let _ = spawner.spawn(task_coin_acceptor(state));
    let _ = spawner.spawn(task_hopper());
    let _ = spawner.spawn(task_buttons());
    let _ = spawner.spawn(task_gsm(state));
    let _ = spawner.spawn(task_ibutton(None, state));
    let _ = spawner.spawn(task_state_persist(state));

    defmt::info!("Все задачи запущены");

    // TWDT feed loop
    loop {
        embassy_time::Timer::after_secs(1).await;
    }
}
