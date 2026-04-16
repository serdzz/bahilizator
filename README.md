# Бахилизатор v2.0

Rust/Embassy порт прошивки вендингового аппарата с MSP430 (C/IAR) на STM32F103C8T6 Bluepill.

## Описание

Бахилизатор — автомат по продаже штучного товара за монеты. Оригинальная прошивка
работала на MSP430F148/F2618 (IAR Embedded Workbench, ~15K строк C). Этот проект —
полный порт на Rust с использованием async/await фреймворка Embassy.

**Зачем порт:**
- Оригинальный MSP430 снят с производства
- STM32F103C8T6 Bluepill — дешёвая и доступная платформа
- Embassy даёт async многозадачность без RTOS
- Rust — безопасность памяти, типобезопасность, без UB

## Аппаратная часть

| Компонент | Модель | Подключение |
|-----------|--------|-------------|
| МК | STM32F103C8T6 Bluepill | 72 MHz, 64KB Flash, 20KB SRAM |
| Дисплей | HD44780 16×2 | I2C через PCF8574 backpack |
| EEPROM | 24C08 (1024 байт) | I2C (общая шина с дисплеем) |
| Монетоприёмник | NRI G-13.6000 | 6 каналов + блокировка, через NPN |
| Хоппер A | Товарный диспенсер | Enable + Sensor, через NPN |
| Хоппер B | Монетный диспенсер | Enable + Sensor, через NPN |
| GSM модем | SIM800L | USART1 + PWRKEY + STATUS |
| iButton | DS1990A | 1-Wire bit-bang на PA11 |
| Кнопки | 4 шт. | PA3–PA6, active-low, pull-up |
| Двери | 2 датчика | PA7, PA8, active-low |
| LED | Красный / Зелёный | PC15 / PB0 |

### Использование ресурсов

```
Flash:  ~35 KB / 64 KB  (55%)
SRAM:   ~4.8 KB / 20 KB (24%)
```

## Схемы подключения

### NRI G-13 → STM32 (через NPN транзистор)

```
NRI G-13                  NPN (BC547/2N2222)              STM32
┌──────────┐           ┌──────────────┐            ┌──────────┐
│ Pin 3-4  ├───────────┤ 10kΩ → Base │            │          │
│ Pin 7-10 │ (6 линий) ├── Emitter→GND│            │ PB8-13   │
│ (active  │           └── Collector ─┤ 10kΩ ↑3.3V├→ GPIO    │
│  low)    │                          │            │          │
│          │                          │            │          │
│ Pin 6    ├──────────────────────────┤────────────┤ PB14     │
│ (blocking│ (active HIGH,            │            │ (blocking│
│  output) │  без инверсии)           │            │  pin)    │
└──────────┘                          │            └──────────┘
```

Транзистор инвертирует: NRI active-low → NPN → HIGH на GPIO.
HIGH на GPIO STM32 = монета обнаружена на линии.

### Хопперы → STM32 (через NPN транзистор)

```
Hopper                NPN (BC547)                STM32
┌──────────┐       ┌──────────────┐          ┌──────────┐
│ Enable   ├───────┤ 10kΩ→Base   │          │          │
│ (motor)  │       ├── Emitter→GND│          │ PB15/..  │
│          │       └── Collector ─┤────── ───┤→ GPIO    │
│ Sensor   ├─────────────────────────────────┤ PAxx     │
│ (optical)│  (active-low через NPN)         │ (input)  │
└──────────┘                                 └──────────┘
```

+12V питания хоппера → NPN ключ → управление от +3.3V логики STM32.

### HD44780 → PCF8574 → STM32 I2C1

```
HD44780              PCF8574 I2C Backpack          STM32
┌──────────┐       ┌──────────────────┐         ┌──────────┐
│ D4-D7    ├───────┤ P4-P7            │         │          │
│ RS       ├───────┤ P0               │         │          │
│ RW       ├───────┤ P1        SDA ──┤─────────┤ PB7      │
│ EN       ├───────┤ P2        SCL ──┤─────────┤ PB6      │
│ BL       ├───────┤ P3               │         │          │
└──────────┘       └──────────────────┘         └──────────┘
```

PCF8574 pin mapping: P0=RS, P1=RW, P2=E, P3=Backlight, P4=D4, P5=D5, P6=D6, P7=D7

I2C адрес: 0x27 (или 0x3F для альтернативного модуля)

### SIM800L → STM32 USART1

```
SIM800L                                      STM32
┌──────────┐                               ┌──────────┐
│ TXD      ├────────────────────────────────┤ PA10     │
│ RXD      ├────────────────────────────────┤ PA9      │
│ PWRKEY   ├──── NPN транзистор ────────────┤ PA0      │
│ STATUS   ├──── делитель 3.3V ─────────────┤ PA1      │
│ DTR      ├────────────────────────────────┤ PA2      │
│ VCC      │  3.4–4.2V (отдельный LDO!)     │          │
│ GND      ├──── общая земля ──────────────┤ GND      │
└──────────┘                               └──────────┘
```

**Важно:** SIM800L требует 3.4–4.2V и до 2A при передаче!
Использовать отдельный LDO или DC-DC, не питать от 3.3V STM32.

### iButton (DS1990A) → STM32

```
iButton                     STM32
┌──────────┐             ┌──────────┐
│ Data     ├──── 4.7kΩ ──┤ PA11     │
│          │     pull-up   │          │
│ GND      ├──────────────┤ GND      │
└──────────┘             └──────────┘
```

1-Wire bit-bang: OpenDrain, pull-low для передачи, release + pull-up для чтения.

### EEPROM 24C08 → STM32 I2C1

```
24C08                                       STM32
┌──────────┐                               ┌──────────┐
│ SDA      ├────────────────────────────────┤ PB7      │
│ SCL      ├────────────────────────────────┤ PB6      │
│ A0,A1,A2 ├──── GND (адрес 0x50)          │          │
│ WP       ├──── GND (запись разрешена)     │          │
│ VCC      ├──── +3.3V                      │          │
│ GND      ├──── GND                        │          │
└──────────┘                               └──────────┘
```

Общая шина I2C с PCF8574 (дисплей). Адреса не конфликтуют: 0x27 vs 0x50.

### Кнопки и двери → STM32

```
Кнопки (active-low, internal pull-up)       STM32
  PREV  ──── 10kΩ pull-up ──── PA3
  NEXT  ──── 10kΩ pull-up ──── PA4
  OK    ──── 10kΩ pull-up ──── PA5
  CANCEL─── 10kΩ pull-up ──── PA6

Двери (active-low, normally-open)
  Дверь 1 ──── PA7
  Дверь 2 ──── PA8
```

EXTI прерывания → debounce 50мс → Channel → задача обработки.

## Программная архитектура

### Embassy задачи

Проект использует Embassy executor с 8 задачами:

```
┌──────────────────┐   COIN_CHANNEL    ┌──────────────────┐
│  task_coin_       │ ───────────────→  │  task_vending    │
│  acceptor        │                   │  (state machine)  │
└──────────────────┘                   │                   │
┌──────────────────┐  BUTTON_CHANNEL   │  ┌──────────────┐ │
│  task_buttons    │ ───────────────→  │  │ AcceptCash   │ │
└──────────────────┘                   │  │ PayoutItems  │ │
┌──────────────────┐  IBUTTON_CHANNEL  │  │ PayoutRemindr│ │
│  task_ibutton    │ ───────────────→  │  │ ProcessResid │ │
└──────────────────┘                   │  └──────────────┘ │
                                       │                   │
┌──────────────────┐  HOPPER_CMD       │  HOPPER_EVENT     │
│  task_vending    │ ──────────────→   │  ┌──────────────┐ │
│  (выдача)        │                   │  │ task_hopper  │ │
└──────────────────┘                   │  └──────────────┘ │
                                       └──────────────────┘
┌──────────────────┐  GSM_CMD          ┌──────────────────┐
│  task_vending    │ ──────────────→   │  task_gsm       │
│  (SMS/GPRS)      │                   │  (SIM800L)      │
└──────────────────┘                   └──────────────────┘
┌──────────────────┐  DISPLAY_SIGNAL   ┌──────────────────┐
│  task_vending    │ ──────────────→   │  display_task    │
│  (дисплей)        │                   │  (HD44780 I2C)  │
└──────────────────┘                   └──────────────────┘
┌──────────────────┐  PERSIST_SIGNAL   ┌──────────────────┐
│  Все задачи      │ ──────────────→   │  task_state_    │
│                   │                   │  persist        │
└──────────────────┘                   └──────────────────┘
```

### Shared state

Общее состояние — `Mutex<CriticalSectionRawMutex, RefCell<VendingState>>`:

- **VendingState** — верхний уровень: data + settings + errors
- **VendingStateData** — рабочее состояние (EEPROM 24C08): наличность, уровни, транзакции
- **Settings** — конфигурация (Flash page 63): номиналы, ключи, телефоны
- **Errors** — bitflags: ошибки монетоприёмника, хопперов, GSM, дверей

### Vending state machine

4 основных состояния:

```
AcceptCash ──(монета→хватит)──→ PayoutItems
    ↑                              │
    │                         (товар выдан)
    │                              ↓
ProcessResidual ←── PayoutReminder
    │
    └──(сброс)──→ AcceptCash
```

При ошибках → экран "НЕТ ОБСЛУЖИВАНИЯ" + SMS.

### Протоколы

| Протокол | Назначение | Реализация |
|----------|-----------|------------|
| GSM 07.10 CMUX | Мультиплексирование UART | `gsm/cmux.rs` — Basic Mode |
| AT commands | Управление SIM800L | `gsm/at_channel.rs` — DLC1 |
| PPP (LCP/PAP/IPCP) | GPRS интернет | `gsm/ppp_channel.rs` — DLC2 |
| I2C | Дисплей + EEPROM | Embassy I2C (PB6/PB7) |
| 1-Wire | iButton DS1990A | `ibutton.rs` — bit-bang async |

## Как собрать

### Требования

- Rust nightly (embedded target)
- `thumbv7m-none-eabi` target
- `cargo-flash` или `st-flash` для прошивки
- ST-Link V2 (или совместимый)

### Сборка

```bash
# Добавить target (один раз)
rustup target add thumbv7m-none-eabi

# Release сборка
cargo build --release

# Результат: target/thumbv7m-none-eabi/release/bahilizator
```

### Прошивка

```bash
# Вариант 1: cargo-flash
cargo flash --chip stm32f103c8t6 --release

# Вариант 2: st-flash
arm-none-eabi-objcopy -O binary target/thumbv7m-none-eabi/release/bahilizator \
  target/thumbv7m-none-eabi/release/bahilizator.bin
st-flash write target/thumbv7m-none-eabi/release/bahilizator.bin 0x08000000
```

### Отладка

```bash
# defmt лог через RTT (нужен probe-rs)
cargo run --release --features defmt

# Или openocd + gdb
openocd -f interface/stlink.cfg -f target/stm32f1x.cfg
arm-none-eabi-gdb target/thumbv7m-none-eabi/release/bahilizator
```

## Зависимости Embassy

Все Embassy крейты — из git, не crates.io (требуемые фичи ещё не опубликованы):

```toml
embassy-stm32  = { git = "https://github.com/embassy-rs/embassy.git", features = ["stm32f103c8", "time-driver-tim2", "defmt"] }
embassy-executor = { git = "https://github.com/embassy-rs/embassy.git", features = ["executor-thread", "defmt", "platform-cortex-m"] }
embassy-time    = { git = "https://github.com/embassy-rs/embassy.git" }
embassy-sync    = { git = "https://github.com/embassy-rs/embassy.git" }
embassy-futures = { git = "https://github.com/embassy-rs/embassy.git" }
```

## Оригинальный проект

Оригинальная C-прошивка для MSP430 (IAR Embedded Workbench):
`~/Aledo/bahilizator_kwt_svn/`

Содержит: `src/` (C файлы), `include/` (заголовки), `settings/`, `lnk/` (линкер),
IAR project файлы (`.ewp`, `.eww`).

## Структура проекта

```
bahilizator/
├── Cargo.toml
├── memory.x              # Линкер: 64K Flash, 20K RAM
├── src/
│   ├── main.rs            # Embassy tasks + каналы + точка входа
│   ├── lib.rs             # re-exports всех модулей
│   ├── config.rs          # Пин-мап и константы
│   ├── state.rs           # VendingState, Settings, Errors
│   ├── event.rs           # Лог событий и транзакций
│   ├── error.rs           # ErrorSet (bitflags) + типы ошибок
│   ├── vending.rs         # State machine: Accept→Payout→Remind→Residual
│   ├── coin_acceptor.rs   # NRI G-13 (6-канальный / пульсный)
│   ├── hopper.rs           # Параметризованный драйвер хоппера
│   ├── ui.rs              # HD44780 через PCF8574 I2C
│   ├── buttons.rs         # Кнопки + двери → EXTI → Channel
│   ├── ibutton.rs         # 1-Wire DS1990A bit-bang
│   ├── flash.rs           # Flash page 63 — Settings (read/write)
│   ├── nvram.rs           # EEPROM 24C08 — State + wear levelling
│   ├── menu.rs            # Сервисное меню навигация
│   ├── report.rs          # Генерация SMS отчётов
│   └── gsm/
│       ├── mod.rs          # GSM менеджер: питание, init, GPRS
│       ├── cmux.rs         # GSM 07.10 CMUX Basic Mode
│       ├── at_channel.rs   # AT канал через DLC1
│       ├── ppp_channel.rs  # PPP (LCP+PAP+IPCP) через DLC2
│       └── sms.rs          # SMS AT команды
└── docs/
    ├── ARCHITECTURE.md     # Детальная архитектура
    ├── PORTING_NOTES.md    # Заметки по портированию
    ├── PINOUT.md           # Полная распиновка
    └── CMUX_PPP.md         # Протокол CMUX + PPP
```

## Лицензия

Проприетарный. Оригинальная C-версия: Aledo SIA.
Rust-порт: Automated Systems SIA.