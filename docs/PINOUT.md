# Распиновка LilyGo T-Call SIM800 v20190610 — Бахилизатор ESP32

Полная таблица назначения пинов для проекта Бахилизатор v2.0 (ESP32 branch).

## Плата: LilyGo T-Call SIM800 v20190610

- **МК**: ESP32 Xtensa LX6 dual-core, 520KB SRAM, 4MB Flash
- **Модем**: SIM800L onboard (UART2)
- **Питание**: IP5306 I2C power management (0x75)
- **USB**: Micro-USB (CP2102 UART bridge)

## Общая таблица GPIO

| GPIO | Направление | Назначение | Примечание |
|------|------------|-----------|------------|
| GPIO0 | Input (PD) | Coin CH2 | Strapping (boot mode), pull-down |
| GPIO2 | Input (PU) | Кнопка CANCEL | Strapping (boot log), после boot — кнопка |
| GPIO4 | Output | GSM PWRKEY | LOW pulse >1с включает модем |
| GPIO5 | Output | GSM RST | LOW = hard reset модема |
| GPIO12 | Input (PD) | Coin CH1 | Strapping MTDI, **внешний pull-down обязателен** |
| GPIO13 | Output | LED | User LED на T-Call v1.4 |
| GPIO14 | Input (PD) | Coin CH3 | — |
| GPIO15 | Input (PD) | Coin CH4 | Strapping MTDO |
| GPIO16 | Input (PD) | Coin CH5 | Не доступен при PSRAM |
| GPIO17 | Input (PD) | Coin CH6 | Не доступен при PSRAM |
| GPIO18 | I2C1 SDA | EEPROM 24C08 | SDA для I2C1 |
| GPIO19 | I2C1 SCL | EEPROM 24C08 | SCL для I2C1 |
| GPIO21 | I2C0 SDA | HD44780 (PCF8574) | SDA для I2C0 |
| GPIO22 | I2C0 SCL | HD44780 (PCF8574) | SCL для I2C0 |
| GPIO23 | Output | GSM POWER | HIGH = питание модема ON |
| GPIO25 | Output | Hopper B Enable | LOW = мотор ON |
| GPIO26 | UART2 TX | → SIM800L RXD | TX модема |
| GPIO27 | UART2 RX | ← SIM800L TXD | RX модема |
| GPIO32 | Output | Hopper A Enable | LOW = мотор ON |
| GPIO33 | Input (PU) | Кнопка NEXT | — |
| GPIO34 | Input (PU) | Hopper A Sensor | Input-only, нет pull-up (внешний) |
| GPIO35 | Input (PU) | Hopper B Sensor | Input-only, нет internal pull-up |
| GPIO36 | Input (PU) | Кнопка PREV | Input-only (VP) |
| GPIO39 | Input (PU) | Кнопка OK | Input-only (VN) |

**PD** = Pull-Down, **PU** = Pull-Up

## Группировка по подсистемам

### I2C0 — Дисплей (SDA=GPIO21, SCL=GPIO22)

| Устройство | I2C адрес | Описание |
|-----------|----------|----------|
| PCF8574 | 0x27 | HD44780 I2C backpack |
| IP5306 | 0x75 | Power management IC |

Частота: 100 kHz. I2C0 передаётся по значению в `task_display`.

### I2C1 — EEPROM (SDA=GPIO18, SCL=GPIO19)

| Устройство | I2C адрес | Описание |
|-----------|----------|----------|
| 24C08 | 0x50 | EEPROM 1024 байт, wear-levelling |

Частота: 100 kHz. I2C1 через `StaticCell` + `nvram::set_i2c()`.

**Почему два I2C**: Разделение шин — дисплей и EEPROM на разных I2C
контроллерах (I2C0 и I2C1). Устраняет конфликт владения и повышает
надёжность (no bus contention).

### UART2 — SIM800L (TX=GPIO26, RX=GPIO27)

| Параметр | Значение |
|---------|---------|
| Скорость | 115200 8N1 |
| TX | GPIO26 → SIM800L RXD |
| RX | GPIO27 ← SIM800L TXD |

UART2 split на `UartTx`/`UartRx` через `gsm::set_uart()`.
Асинхронный режим (`.into_async()`) для Embassy.

### GSM управление

| GPIO | Назначение | Активный уровень | Описание |
|------|-----------|----------------|---------|
| GPIO4 | PWRKEY | LOW pulse >1с | Включает/выключает модем |
| GPIO5 | RST | LOW | Hard reset модема |
| GPIO23 | POWER | HIGH | Питание модема ON |

### Монетоприёмник NRI G-13.6000

| GPIO | Назначение | Направление | Примечание |
|------|-----------|------------|-----------|
| GPIO12 | Coin CH1 | Input (PD) | Strapping MTDI, внешний pull-down |
| GPIO0 | Coin CH2 | Input (PD) | Strapping boot, после boot — вход |
| GPIO14 | Coin CH3 | Input (PD) | — |
| GPIO15 | Coin CH4 | Input (PD) | Strapping MTDO |
| GPIO16 | Coin CH5 | Input (PD) | Нет при PSRAM |
| GPIO17 | Coin CH6 | Input (PD) | Нет при PSRAM |

**NPN инверсия**: NRI G-13 активный-low (+12V) → NPN BC547 → HIGH на GPIO.

```
NRI output (active low) ──→ 10kΩ → Base (BC547)
                            Emitter → GND
                            Collector → ESP32 GPIO + 10kΩ pull-up → +3.3V
```

### Хопперы

| GPIO | Назначение | Направление | Примечание |
|------|-----------|------------|-----------|
| GPIO32 | Hopper A Enable | Output | LOW = мотор ON |
| GPIO25 | Hopper B Enable | Output | LOW = мотор ON |
| GPIO34 | Hopper A Sensor | Input | Input-only, внешний pull-up |
| GPIO35 | Hopper B Sensor | Input | Input-only, внешний pull-up |

### Кнопки

| GPIO | Назначение | Направление | Примечание |
|------|-----------|------------|-----------|
| GPIO36 | PREV ("<") | Input (PU) | Input-only |
| GPIO33 | NEXT (">") | Input (PU) | — |
| GPIO39 | OK | Input (PU) | Input-only |
| GPIO2 | CANCEL | Input (PU) | Strapping, после boot — кнопка |

Все кнопки — между пином и GND. Внутренний pull-up = HIGH (не нажата).

### LED

| GPIO | Назначение | Направление |
|------|-----------|------------|
| GPIO13 | User LED | Output |

### iButton 1-Wire

**Не подключён на LilyGo T-Call** — GPIO4 занят под SIM800L PWRKEY.
Нет свободного output-capable GPIO для 1-Wire Open-Drain.

Если нужен iButton — варианты:
1. Освободить GPIO0 (Coin CH2) — но это strapping pin
2. Убрать один Coin channel
3. Использовать GPIO32 (Hopper A Enable) если хоппер не нужен

## ESP32 Strapping Pins

| GPIO | Функция при boot | Ограничение |
|------|-----------------|------------|
| GPIO0 | Boot mode (HIGH = SPI boot) | Внешний pull-up 10kΩ |
| GPIO2 | Boot log enable (LOW = silent) | Нужен LOW при boot |
| GPIO5 | SDIO timing (HIGH = 3.3V) | — |
| GPIO12 | Voltage (HIGH = 1.8V, LOW = 3.3V) | **Внешний pull-down обязателен** |
| GPIO15 | Boot message enable (HIGH = verbose) | — |

**Критично**: GPIO12 без внешнего pull-down может вызвать boot при 1.8V вместо 3.3V — ESP32 не загрузится.

## Input-Only GPIO

На ESP32 пины GPIO34–39 — только вход, без internal pull-up/pull-down:

| GPIO | Назначение | Внешний pull |
|------|-----------|-------------|
| GPIO34 | Hopper A Sensor | Внешний 10kΩ pull-up |
| GPIO35 | Hopper B Sensor | Внешний 10kΩ pull-up |
| GPIO36 | Кнопка PREV | Внешний 10kΩ pull-up |
| GPIO39 | Кнопка OK | Внешний 10kΩ pull-up |

## Нераспределённые / недоступные GPIO

| GPIO | Статус | Причина |
|------|--------|---------|
| GPIO1 | Занят | UART0 TX (console) |
| GPIO3 | Занят | UART0 RX (console) |
| GPIO4 | Занят | SIM800L PWRKEY |
| GPIO6–11 | Недоступны | Внутренняя Flash |
| GPIO16 | Условно свободен | Занят при PSRAM |
| GPIO17 | Условно свободен | Занят при PSRAM |

## Питание

| Узел | Напряжение | Источник |
|------|-----------|----------|
| ESP32 VDD | 3.3V | IP5306 Buck/Boost |
| SIM800L VCC | 3.4–4.2V | IP5306 VBatt (до 2A!) |
| HD44780 VCC | 5V | IP5306 Boost |
| NRI G-13 | +12V DC | Внешний БП |
| Хопперы | +12V DC | Внешний БП |
| 24C08 VCC | 3.3V | От ESP32 |

IP5306 управляет питанием: заряжает LiPo 3.7V, обеспечивает 5V/3.3V/4.2V.
I2C адрес 0x75 (I2C0 шина).