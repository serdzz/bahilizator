//! Бахилизатор — Rust/Embassy порт для LilyGo T-Call SIM800 (IP5306)
//!
//! Плата: LilyGo T-Call SIM800 v20190610
//! ESP32 Xtensa LX6 dual-core, 520KB SRAM, 4MB Flash
//! SIM800L на борту, IP5306 power management (I2C 0x75)

#![no_std]
#![no_main]

use esp_backtrace as _; // panic handler
use esp_println as _; // defmt-espflash global logger

defmt::timestamp!(""); // defmt timestamp placeholder

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
use esp_hal::gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull};
use esp_hal::i2c::master::{Config as I2cConfig, I2c};
use esp_hal::time::Rate;
use esp_hal::uart::{Config as UartConfig, Uart};
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

// ── Статические ячейки для I2C1 (EEPROM) ────────────────────────────────
static I2C1_CELL: StaticCell<esp_hal::i2c::master::I2c<'static, esp_hal::Blocking>> =
    StaticCell::new();

// ── Статические ячейки для UART2 ──────────────────────────────────────────
// UART2 split() даёт UartRx и UartTx, передаются через set_uart() в gsm модуль

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

#[embassy_executor::task]
async fn task_display(i2c: esp_hal::i2c::master::I2c<'static, esp_hal::Blocking>) {
    ui::display_task_with_i2c(&DISPLAY_SIGNAL, i2c).await;
}

// ── main — точка входа ESP32 ─────────────────────────────────────────────

#[esp_rtos::main]
async fn main(spawner: Spawner) {
    defmt::info!("Бахилизатор v2.0 — LilyGo T-Call SIM800 — запуск...");

    // ══════════════════════════════════════════════════════════════════
    // Инициализация ESP32 периферии
    // ══════════════════════════════════════════════════════════════════
    let peripherals = esp_hal::init(esp_hal::Config::default());

    // ── I2C0 — Дисплей (SDA=GPIO21, SCL=GPIO22) ────────────────────
    let i2c = I2c::new(
        peripherals.I2C0,
        I2cConfig::default().with_frequency(Rate::from_khz(100)),
    )
    .expect("I2C0 init failed")
    .with_sda(peripherals.GPIO21)
    .with_scl(peripherals.GPIO22);

    // ── I2C1 — EEPROM 24C08 (SDA=GPIO18, SCL=GPIO19) ────────────────
    let i2c1 = I2c::new(
        peripherals.I2C1,
        I2cConfig::default().with_frequency(Rate::from_khz(100)),
    )
    .expect("I2C1 init failed")
    .with_sda(peripherals.GPIO18)
    .with_scl(peripherals.GPIO19);
    let i2c1_ref = I2C1_CELL.init(i2c1);
    nvram::set_i2c(i2c1_ref);

    // ── UART2 — SIM800L (GPIO26 TX, GPIO27 RX) ─────────────────────
    let uart2 = Uart::new(peripherals.UART2, UartConfig::default())
        .expect("UART2 init failed")
        .with_rx(peripherals.GPIO27)
        .with_tx(peripherals.GPIO26)
        .into_async();

    // Передаём UART2 в GSM драйвер — split() на TX и RX происходит внутри
    gsm::set_uart(uart2);

    // ── GSM управляющие пины (на плате LilyGo T-Call) ────────────────
    // PWRKEY = GPIO4: LOW pulse >1с включает/выключает модем
    // RST = GPIO5: LOW — hard reset
    // POWER = GPIO23: HIGH — подаёт питание на модем
    let gsm_pwrkey = Output::new(peripherals.GPIO4, Level::High, OutputConfig::default());
    let gsm_rst = Output::new(peripherals.GPIO5, Level::High, OutputConfig::default());
    let gsm_power = Output::new(peripherals.GPIO23, Level::High, OutputConfig::default());
    gsm::set_control_pins(gsm_pwrkey, gsm_rst, gsm_power);

    // ── GPIO выходы ──────────────────────────────────────────────────
    // Hopper A Enable — GPIO32
    let _hopper_a_enable = Output::new(peripherals.GPIO32, Level::Low, OutputConfig::default());
    // Hopper B Enable — GPIO25
    let _hopper_b_enable = Output::new(peripherals.GPIO25, Level::Low, OutputConfig::default());

    // LED — GPIO13 (User LED на T-Call v1.4)
    let mut led = Output::new(peripherals.GPIO13, Level::Low, OutputConfig::default());

    // ── GPIO входы ──────────────────────────────────────────────────
    // Coin channels (через NPN: HIGH = монета обнаружена)
    // ВНИМАНИЕ: GPIO12 — strapping pin (MTDI), нужен внешний pull-down!
    let _coin_ch1 = Input::new(
        peripherals.GPIO12,
        InputConfig::default().with_pull(Pull::Down),
    );
    // GPIO13 — используется под LED. Coin CH2 переносим на другой пин.
    // Если LED не нужен — можно использовать GPIO13 под Coin CH2.
    // Пока: Coin CH2 не подключён (GPIO13 = LED)

    // Coin CH3 — GPIO14
    let _coin_ch3 = Input::new(
        peripherals.GPIO14,
        InputConfig::default().with_pull(Pull::Down),
    );
    // Coin CH4 — GPIO15 (strapping pin MTDO, boot messages)
    let _coin_ch4 = Input::new(
        peripherals.GPIO15,
        InputConfig::default().with_pull(Pull::Down),
    );
    // Coin CH5 — GPIO16 (если нет PSRAM)
    let _coin_ch5 = Input::new(
        peripherals.GPIO16,
        InputConfig::default().with_pull(Pull::Down),
    );
    // Coin CH6 — GPIO17 (если нет PSRAM)
    let _coin_ch6 = Input::new(
        peripherals.GPIO17,
        InputConfig::default().with_pull(Pull::Down),
    );

    // Hopper A Sensor — GPIO34 (input-only)
    let _hopper_a_sensor = Input::new(
        peripherals.GPIO34,
        InputConfig::default().with_pull(Pull::Up),
    );
    // Hopper B Sensor — GPIO35 (input-only)
    let _hopper_b_sensor = Input::new(
        peripherals.GPIO35,
        InputConfig::default().with_pull(Pull::Up),
    );

    // Buttons (input-only: GPIO36, GPIO39 + GPIO2, GPIO33)
    let _btn_prev = Input::new(
        peripherals.GPIO36,
        InputConfig::default().with_pull(Pull::Up),
    );
    let _btn_next = Input::new(
        peripherals.GPIO33,
        InputConfig::default().with_pull(Pull::Up),
    );
    let _btn_ok = Input::new(
        peripherals.GPIO39,
        InputConfig::default().with_pull(Pull::Up),
    );
    let _btn_cancel = Input::new(
        peripherals.GPIO2,
        InputConfig::default().with_pull(Pull::Up),
    );

    // Door 1 — нет свободного input-only GPIO на LilyGo T-Call
    // GPIO34/35/36/39 заняты сенсорами хопперов и кнопками
    // Если нужна дверь — перенести сенсоры на обычные GPIO через NPN

    // ── iButton 1-Wire ──────────────────────────────────────────────
    // На LilyGo T-Call GPIO4 занят под SIM800L PWRKEY.
    // GPIO33 — input-only, не подходит для 1-Wire OpenDrain.
    // iButton не инициализируется на этой плате (нет свободного output-capable GPIO).
    // Если нужен iButton — освободить GPIO32 (DTR не нужен) или GPIO0.

    // ── GPIO пины переданы в драйверы через set_control_pins / StaticCell ──
    // Кнопки, монеты, хопперы — читаются через polling в своих задачах.
    // Глобальные GPIO-указатели хранятся в драйверных модулях.

    // ══════════════════════════════════════════════════════════════════
    // Инициализация состояния
    // ══════════════════════════════════════════════════════════════════
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

    // ══════════════════════════════════════════════════════════════════
    // Spawn задач
    // ══════════════════════════════════════════════════════════════════
    let _ = spawner.spawn(task_vending(state));
    let _ = spawner.spawn(task_coin_acceptor(state));
    let _ = spawner.spawn(task_hopper());
    let _ = spawner.spawn(task_buttons());
    let _ = spawner.spawn(task_gsm(state));
    let _ = spawner.spawn(task_ibutton(None, state)); // Пока без iButton
    let _ = spawner.spawn(task_state_persist(state));
    let _ = spawner.spawn(task_display(i2c));

    defmt::info!("Все задачи запущены");

    // ── Мигаем LED — индикация работы ───────────────────────────────
    led.set_high();

    loop {
        embassy_time::Timer::after_secs(1).await;
        led.toggle();
    }
}
