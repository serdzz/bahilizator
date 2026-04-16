# Бахилизатор v2.0

Rust/Embassy порт прошивки вендингового аппарата с MSP430 (C/IAR) на ESP32 LilyGo T-Call.

## Ветки

| Ветка | Платформа | Статус |
|-------|----------|--------|
| `master` | STM32F103C8T6 Bluepill | Компилируется, clippy clean |
| `esp32` | ESP32 LilyGo T-Call SIM800 | Компилируется, clippy clean |

## Описание

Бахилизатор — автомат по продаже штучного товара за монеты. Оригинальная прошивка
работала на MSP430F148/F2618 (IAR Embedded Workbench, ~15K строк C). Этот проект —
полный порт на Rust с использованием async/await фреймворка Embassy.

**Зачем порт:**
- Оригинальный MSP430 снят с производства
- ESP32 LilyGo T-Call — плата с встроенным SIM800L модемом
- Embassy даёт async многозадачность без RTOS
- Rust — безопасность памяти, типобезопасность, без UB

## ESP32 Branch — LilyGo T-Call SIM800 v20190610

### Аппаратная часть

| Компонент | Модель | Подключение |
|-----------|--------|-------------|
| МК | ESP32 Xtensa LX6 | 240 MHz dual-core, 520KB SRAM, 4MB Flash |
| GSM модем | SIM800L (onboard) | UART2 (GPIO26/27) |
| Power Mgmt | IP5306 (onboard) | I2C0 (0x75) |
| Дисплей | HD44780 16×2 | I2C0 через PCF8574 (GPIO21/22) |
| EEPROM | 24C08 (1024 байт) | I2C1 (GPIO18/19) |
| Монетоприёмник | NRI G-13.6000 | 6 каналов + блокировка, через NPN |
| Хоппер A/B | Товарный/монетный | Enable + Sensor |
| Кнопки | 4 шт. | GPIO2/33/36/39, active-low |
| LED | User LED | GPIO13 |

### Использование ресурсов (ESP32)

```
Flash:  ~80 KB / 4 MB   (2%)
SRAM:  ~194 KB / 520 KB (37%)
Lines:  ~7000
```

### Реальные драйверы esp-hal (без заглушек)

| Периферия | Драйвер | Статус |
|-----------|---------|--------|
| UART2 (SIM800L) | `esp_hal::Uart` async split | ✅ Реальный |
| I2C0 (дисплей) | `esp_hal::i2c::master::I2c` | ✅ Реальный |
| I2C1 (EEPROM) | `esp_hal::i2c::master::I2c` | ✅ Реальный |
| GPIO выходы | `esp_hal::gpio::Output` | ✅ Реальный |
| GPIO входы | `esp_hal::gpio::Input` | ✅ Реальный |
| LED | `esp_hal::gpio::Output` | ✅ Реальный |
| GSM PWRKEY/RST/POWER | `esp_hal::gpio::Output` | ✅ Реальный |
| 1-Wire (iButton) | EspHalPin (заглушка) | ⚠️ Нет свободного GPIO |
| Двери | Не подключены | ⚠️ Нет свободных input-only GPIO |

### GPIO Map (LilyGo T-Call)

| GPIO | Назначение | Примечание |
|------|-----------|------------|
| GPIO0 | Coin CH2 | Strapping (boot mode) |
| GPIO2 | Кнопка CANCEL | Strapping |
| GPIO4 | GSM PWRKEY | SIM800L управление |
| GPIO5 | GSM RST | Hard reset модема |
| GPIO12 | Coin CH1 | Strapping MTDI |
| GPIO13 | LED | User LED |
| GPIO14 | Coin CH3 | — |
| GPIO15 | Coin CH4 | Strapping MTDO |
| GPIO16 | Coin CH5 | Занят при PSRAM |
| GPIO17 | Coin CH6 | Занят при PSRAM |
| GPIO18 | I2C1 SDA (EEPROM) | — |
| GPIO19 | I2C1 SCL (EEPROM) | — |
| GPIO21 | I2C0 SDA (дисплей) | — |
| GPIO22 | I2C0 SCL (дисплей) | — |
| GPIO23 | GSM POWER | HIGH = ON |
| GPIO25 | Hopper B Enable | LOW = мотор ON |
| GPIO26 | UART2 TX → SIM800L | — |
| GPIO27 | UART2 RX ← SIM800L | — |
| GPIO32 | Hopper A Enable | — |
| GPIO33 | Кнопка NEXT | — |
| GPIO34 | Hopper A Sensor | Input-only |
| GPIO35 | Hopper B Sensor | Input-only |
| GPIO36 | Кнопка PREV | Input-only |
| GPIO39 | Кнопка OK | Input-only |

### Сборка ESP32

```bash
# Установить espup
cargo install espup
espup install --targets esp32

# Загрузить окружение
source ~/export-esp.sh  # или . ~/export-esp.sh

# Собрать
cargo +esp build --release -Zbuild-std=core,alloc

# Прошить
espflash flash --release -Zbuild-std=core,alloc --monitor
```

### Сетевой стек (GPRS)

| Компонент | Crate | Назначение |
|-----------|-------|------------|
| PPP | embassy-net-ppp | GPRS интернет (DLC2) |
| IP Stack | embassy-net | IPv4, DHCP, TCP, UDP, DNS |
| DNS | embassy-net (dns) | hostname → IP, кэш 4 записи |
| MQTT | rust-mqtt v5 | Телеметрия, ошибки, события |
| 1-Wire | one-wire-bus | iButton DS1990A (заглушка) |

## Схемы подключения

### NRI G-13 → ESP32 (через NPN транзистор)

```
NRI G-13                  NPN (BC547/2N2222)              ESP32
┌──────────┐           ┌──────────────┐            ┌──────────┐
│ Pin 3-4  ├───────────┤ 10kΩ → Base │            │          │
│ Pin 7-10 │ (6 линий) ├── Emitter→GND│            │ GPIO12-17│
│ (active  │           └── Collector ─┤ 10kΩ ↑3.3V├→ GPIO    │
│  low at  │              │           │            │          │
│  +12V)   │              └───────────┤            │          │
└──────────┘                          │            └──────────┘
```

NRI G-13 активный-low (+12V standby, GND при монете).
NPN инвертирует: GND → Collector HIGH (3.3V через pull-up).
ESP32 видит HIGH = монета обнаружена.

### SIM800L (на плате LilyGo T-Call)

```
ESP32 GPIO26 (UART2 TX) ────→ SIM800L RXD
ESP32 GPIO27 (UART2 RX) ←──── SIM800L TXD
ESP32 GPIO4  (PWRKEY)   ────→ SIM800L PWRKEY (LOW pulse >1с)
ESP32 GPIO5  (RST)      ────→ SIM800L RST (LOW = reset)
ESP32 GPIO23 (POWER)    ────→ SIM800L POWER (HIGH = ON)
```

### I2C (два контроллера)

```
ESP32 GPIO21 (I2C0 SDA) ←→ PCF8574 (0x27) ←→ HD44780
ESP32 GPIO22 (I2C0 SCL) ←→ IP5306 (0x75)
ESP32 GPIO18 (I2C1 SDA) ←→ 24C08 (0x50) EEPROM
ESP32 GPIO19 (I2C1 SCL) ←→ 24C08 (0x50) EEPROM
```

## Структура проекта

```
bahilizator/
├── src/
│   ├── main.rs            Embassy entry point, периферия, GPIO init
│   ├── config.rs           Константы (пины, адреса, таймауты)
│   ├── state.rs            VendingState, Settings, AppState
│   ├── vending.rs          State machine (AcceptCash → PayoutItems → ...)
│   ├── coin_acceptor.rs    NRI G-13.6000 poll (1мс, 6 каналов + BLOCK)
│   ├── hopper.rs           Хопперы A/B (motor + sensor)
│   ├── buttons.rs          Кнопки + двери (10мс poll)
│   ├── gsm/
│   │   ├── mod.rs           GSM менеджер (init, CMUX, power)
│   │   ├── at_channel.rs    AT команды через CMUX DLC1
│   │   ├── cmux.rs          GSM 07.10 Basic Mode mux
│   │   ├── ppp_channel.rs  PPP через CMUX DLC2
│   │   ├── sms.rs           SMS отправка/приём
│   │   ├── dns.rs           DNS resolver (кэш 4 записи)
│   │   ├── mqtt.rs          MQTT v5 клиент (rust-mqtt)
│   │   └── mqtt_topics.rs  Топики и формат JSON
│   ├── nvram.rs            EEPROM 24C08 (wear levelling, I2C1)
│   ├── flash.rs            Settings → NVS (esp-storage)
│   ├── ui.rs               HD44780 через PCF8574 (I2C0)
│   ├── menu.rs              Навигация меню
│   ├── ibutton.rs           1-Wire DS1990A (one-wire-bus)
│   ├── event.rs             Event ring buffer + TransactionEntry
│   ├── report.rs            SMS отчёты (state, errors, accounting)
│   └── error.rs             Error bitflags (18 категорий)
├── docs/
│   ├── ARCHITECTURE.md     Архитектура и потоки данных
│   ├── PINOUT.md           Распиновка ESP32 LilyGo T-Call
│   ├── PORTING_NOTES.md    Заметки по портированию MSP430 → ESP32
│   └── CMUX_PPP.md         CMUX/PPP протоколы
├── Cargo.toml
└── Makefile
```

## Тестирование

```bash
# Unit-тесты (host)
cargo test

# Проверка сборки ESP32
cargo +esp build --release -Zbuild-std=core,alloc

# Clippy
cargo +esp clippy --release -Zbuild-std=core,alloc -- -D warnings

# Прошивка
espflash flash --release -Zbuild-std=core,alloc --monitor /dev/ttyUSB0
```

## Ключевые решения

1. **Embassy вместо RTIC** — async/await更适合 для GPRS/CMUX/MQTT
2. **Два I2C** — раздельные шины для дисплея и EEPROM
3. **UART2 async split** — TX и RX в разных задачах
4. **CMUX** — мультиплексирование AT команд и PPP на одном UART
5. **Wear levelling** — 4 сектора EEPROM по 256 байт, ротация при записи
6. **No heap** — `heapless::String`, `heapless::Vec`, статические буферы
7. **NPN инверсия** — монетоприёмник +12V → ESP32 3.3V через BC547
8. **1-Wire заглушка** — на LilyGo T-Call нет свободного GPIO для iButton

## Лицензия

Проприетарное ПО. Оригинальная прошивка © Aledo / Bahilizator KWT.
Rust порт — по договорённости.