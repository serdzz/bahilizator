//! Бахилизатор — Rust/Embassy порт для STM32F103C8T6 Bluepill

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    channel::Channel,
    signal::Signal,
};
use static_cell::StaticCell;

use bahilizator::{
    buttons, coin_acceptor, flash, gsm, hopper, ibutton, nvram, state, ui, vending,
};

use panic_probe as _;

// ── Статические каналы и сигналы ────────────────────────────────────────

static COIN_CHANNEL: Channel<CriticalSectionRawMutex, coin_acceptor::CoinEvent, 4> = Channel::new();
static BUTTON_CHANNEL: Channel<CriticalSectionRawMutex, buttons::ButtonEvent, 4> = Channel::new();
static HOPPER_CMD_CHANNEL: Channel<CriticalSectionRawMutex, hopper::HopperCmd, 4> = Channel::new();
static HOPPER_EVENT_CHANNEL: Channel<CriticalSectionRawMutex, hopper::HopperEvent, 4> = Channel::new();
static GSM_CMD_CHANNEL: Channel<CriticalSectionRawMutex, gsm::GsmCommand, 4> = Channel::new();
static SMS_EVENT_CHANNEL: Channel<CriticalSectionRawMutex, gsm::SmsEvent, 4> = Channel::new();
static IBUTTON_CHANNEL: Channel<CriticalSectionRawMutex, ibutton::IbuttonEvent, 1> = Channel::new();

static DISPLAY_SIGNAL: Signal<CriticalSectionRawMutex, ui::DisplayCommand> = Signal::new();
static PERSIST_SIGNAL: Signal<CriticalSectionRawMutex, state::PersistReason> = Signal::new();

// ── Общее состояние ────────────────────────────────────────────────────

static STATE_CELL: StaticCell<embassy_sync::mutex::Mutex<
    CriticalSectionRawMutex,
    core::cell::RefCell<state::VendingState>,
>> = StaticCell::new();

// ── Embassy tasks ────────────────────────────────────────────────────────

#[embassy_executor::task]
async fn task_vending(
    state: &'static embassy_sync::mutex::Mutex<CriticalSectionRawMutex, core::cell::RefCell<state::VendingState>>,
) {
    vending::run(
        state,
        COIN_CHANNEL.receiver(),
        BUTTON_CHANNEL.receiver(),
        HOPPER_EVENT_CHANNEL.receiver(),
        GSM_CMD_CHANNEL.sender(),
        &DISPLAY_SIGNAL,
        &PERSIST_SIGNAL,
    )
    .await;
}

#[embassy_executor::task]
async fn task_coin_acceptor(
    state: &'static embassy_sync::mutex::Mutex<CriticalSectionRawMutex, core::cell::RefCell<state::VendingState>>,
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
    state: &'static embassy_sync::mutex::Mutex<CriticalSectionRawMutex, core::cell::RefCell<state::VendingState>>,
) {
    gsm::run(GSM_CMD_CHANNEL.receiver(), SMS_EVENT_CHANNEL.sender(), state).await;
}

#[embassy_executor::task]
async fn task_ibutton() {
    ibutton::run(IBUTTON_CHANNEL.sender()).await;
}

#[embassy_executor::task]
async fn task_state_persist(
    state: &'static embassy_sync::mutex::Mutex<CriticalSectionRawMutex, core::cell::RefCell<state::VendingState>>,
) {
    nvram::persist_task(state, &PERSIST_SIGNAL).await;
}

// ── main — точка входа ──────────────────────────────────────────────────

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    defmt::info!("Бахилизатор v2.0 — запуск...");

    let _p = embassy_stm32::init(embassy_stm32::Config::default());

    // Инициализация состояния
    let state = STATE_CELL.init(embassy_sync::mutex::Mutex::new(
        core::cell::RefCell::new(state::VendingState::default()),
    ));

    // Загрузка из Flash/EEPROM
    {
        let mut guard = state.lock().await;
        let st = guard.get_mut();
        st.settings = flash::load_settings().unwrap_or_default();
        st.data = nvram::load_state_default();
    }

    // Spawn задач
    spawner.spawn(task_vending(state)).ok();
    spawner.spawn(task_coin_acceptor(state)).ok();
    spawner.spawn(task_hopper()).ok();
    spawner.spawn(task_buttons()).ok();
    spawner.spawn(task_gsm(state)).ok();
    spawner.spawn(task_ibutton()).ok();
    spawner.spawn(task_state_persist(state)).ok();

    defmt::info!("Все задачи запущены");

    // TODO: IWDG feed loop
    loop {
        embassy_time::Timer::after_secs(1).await;
    }
}