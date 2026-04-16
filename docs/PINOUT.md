# Распиновка STM32F103C8T6 Bluepill — Бахилизатор

Полная таблица назначения пинов для проекта Бахилизатор v2.0.

## Общая таблица

| Пин | Порт | AF | Назначение | Направление | Примечание |
|-----|------|----|-----------|-------------|------------|
| PA0 | GPIO | — | GSM PWRKEY | Output (PP) | NPN ключ, инверсия |
| PA1 | GPIO | — | GSM STATUS | Input (PU) | Делитель 3.3V |
| PA2 | GPIO | — | GSM DTR | Output (PP) | Управление сном |
| PA3 | GPIO | — | Кнопка PREV | Input (PU) | Active-low, EXTI |
| PA4 | GPIO | — | Кнопка NEXT | Input (PU) | Active-low, EXTI |
| PA5 | GPIO | — | Кнопка OK | Input (PU) | Active-low, EXTI |
| PA6 | GPIO | — | Кнопка CANCEL | Input (PU) | Active-low, EXTI |
| PA7 | GPIO | — | Дверь 1 | Input (PU) | Active-low, EXTI |
| PA8 | GPIO | — | Дверь 2 | Input (PU) | Active-low, EXTI |
| PA9 | GPIO | AF1 | USART1 TX | AF Push-Pull | → SIM800L RXD |
| PA10 | GPIO | AF1 | USART1 RX | Input (PU) | ← SIM800L TXD |
| PA11 | GPIO | — | 1-Wire iButton | Output OD / Input | Bit-bang |
| PB0 | GPIO | — | LED зелёный | Output (PP) | Активный высокий |
| PB1 | GPIO | — | Power Fail | Input (PU) | Детектор питания |
| PB6 | GPIO | AF4 | I2C1 SCL | AF Open-Drain | → PCF8574 + 24C08 |
| PB7 | GPIO | AF4 | I2C1 SDA | AF Open-Drain | → PCF8574 + 24C08 |
| PB8 | GPIO | — | Coin CH1 | Input (PU) | Через NPN инверсию |
| PB9 | GPIO | — | Coin CH2 | Input (PU) | Через NPN инверсию |
| PB10 | GPIO | — | Coin CH3 | Input (PU) | Через NPN инверсию |
| PB11 | GPIO | — | Coin CH4 | Input (PU) | Через NPN инверсию |
| PB12 | GPIO | — | Coin CH5 | Input (PU) | Через NPN инверсию |
| PB13 | GPIO | — | Coin CH6 | Input (PU) | Через NPN инверсию |
| PB14 | GPIO | — | Coin BLOCK | Output (PP) | Active HIGH, без инверсии |
| PB15 | GPIO | — | Hopper A Enable | Output (PP) | LOW = мотор ON |
| PC13 | GPIO | — | (свободен) | — | Bluepill LED, не используется |
| PC14 | GPIO | — | (свободен) | — |  |
| PC15 | GPIO | — | LED красный | Output (PP) | Активный высокий |

**PP** = Push-Pull, **PU** = Pull-Up, **OD** = Open-Drain

## Группировка по подсистемам

### I2C (дисплей + EEPROM)

| Пин | Назначение | Устройство | Адрес |
|-----|-----------|-----------|-------|
| PB6 | I2C1 SCL | Общая шина | — |
| PB7 | I2C1 SDA | Общая шина | — |

Устройства на шине:
- PCF8574 (0x27) — HD44780 I2C backpack
- 24C08 (0x50) — EEPROM для VendingStateData

Частота: 100 kHz. Pull-up резисторы 4.7kΩ на SDA/SCL к +3.3V.

### UART (GSM)

| Пин | AF | Назначение | Направление |
|-----|----|-----------|-------------|
| PA9 | USART1_TX | → SIM800L RXD | Output |
| PA10 | USART1_RX | ← SIM800L TXD | Input |

Скорость: 115200 8N1. Cross-connect: STM32 TX → SIM800L RX, STM32 RX ← SIM800L TX.

### GSM управление

| Пин | Назначение | Направление | Примечание |
|-----|-----------|-------------|------------|
| PA0 | PWRKEY | Output (PP) | NPN транзистор, инверсия |
| PA1 | STATUS | Input (PU) | HIGH = модем включён |
| PA2 | DTR | Output (PP) | HIGH = модем активен |

**PWRKEY** — NPN транзистор (BC547):
```
STM32 PA0 ──→ 10kΩ ──→ Base
                     Emitter → GND
                     Collector → SIM800L PWRKEY
                     (SIM800L PWRKEY имеет внутренний pull-up)
```

**STATUS** — делитель напряжения (SIM800L STATUS = 2.8V):
```
SIM800L STATUS ──→ 1kΩ ──┬─→ STM32 PA1
                          │
                         2kΩ
                          │
                         GND
```

### Монетоприёмник NRI G-13

| Пин | Назначение | Направление | Примечание |
|-----|-----------|-------------|------------|
| PB8 | Coin CH1 | Input (PU) | Через NPN, инверсия |
| PB9 | Coin CH2 | Input (PU) | Через NPN, инверсия |
| PB10 | Coin CH3 | Input (PU) | Через NPN, инверсии |
| PB11 | Coin CH4 | Input (PU) | Через NPN, инверсия |
| PB12 | Coin CH5 | Input (PU) | Через NPN, инверсия |
| PB13 | Coin CH6 | Input (PU) | Через NPN, инверсия |
| PB14 | Coin BLOCK | Output (PP) | Active HIGH, прямое управление |

**NPN инверсия** (для PB8–PB13):
```
NRI output (active low) ──→ 10kΩ → Base
                            NPN (BC547)
                            Emitter → GND
                            Collector → STM32 GPIO + 10kΩ pull-up → +3.3V
```

Логика: NRI low → NPN открыт → Collector = HIGH на GPIO → "монета на линии"

**Coin BLOCK** (PB14): прямое управление, без NPN.
HIGH на PB14 = блокировка монетоприёмника (NRI pin 6).

### Хопперы

| Пин | Назначение | Направление | Примечание |
|-----|-----------|-------------|------------|
| PB15 | Hopper A Enable | Output (PP) | LOW = мотор ON |
| PAxx | Hopper A Sensor | Input (PU) | TODO: назначить пин |
| PBxx | Hopper B Enable | Output (PP) | TODO: назначить пин |
| PAxx | Hopper B Sensor | Input (PU) | TODO: назначить пин |

**Управление мотором** — NPN транзистор:
```
STM32 GPIO ──→ 10kΩ → Base
                     NPN (BC547)
                     Emitter → GND
                     Collector → Hopper Motor (+12V через мотор)
```

**Сенсор** — оптический датчик хоппера, active-low через NPN:
```
Hopper sensor ──→ NPN → STM32 GPIO + pull-up
```

**Примечание:** Точные пины сенсоров и Enable хоппера B — TODO, зависят от
конкретной схемы подключения.

### Кнопки

| Пин | Назначение | Направление | Примечание |
|-----|-----------|-------------|------------|
| PA3 | PREV ("<") | Input (PU) | Active-low, EXTI |
| PA4 | NEXT (">") | Input (PU) | Active-low, EXTI |
| PA5 | OK | Input (PU) | Active-low, EXTI |
| PA6 | CANCEL | Input (PU) | Active-low, EXTI |

Все кнопки — между пином и GND. Внутренний pull-up = не нажата (HIGH).
Нажата = LOW. EXTI по falling edge.

Debounce: 50мс программный в `buttons::run()`.

### Двери

| Пин | Назначение | Направление | Примечание |
|-----|-----------|-------------|------------|
| PA7 | Дверь 1 | Input (PU) | Active-low, EXTI |
| PA8 | Дверь 2 | Input (PU) | Active-low, EXTI |

Датчик двери = геркон / микропереключатель.
Закрыта = HIGH (pull-up). Открыта = LOW.
EXTI по обоим фронтам (открытие + закрытие).

### 1-Wire iButton

| Пин | Назначание | Направление | Примечание |
|-----|-----------|-------------|------------|
| PA11 | 1-Wire Data | Output OD / Input | Bit-bang |

```
PA11 ──── 4.7kΩ ──── +3.3V (pull-up)
  │
  └──── Data DS1990A iButton
         │
        GND
```

PA11 конфигурируется динамически:
- **Передача:** Output Open-Drain, LOW = pull bus low
- **Чтение:** Input с pull-up = released bus

Тайминги: reset 480µs, write-1 = 6µs, write-0 = 60µs, read = 10µs+9µs sample.

### LED

| Пин | Назначение | Направление | Примечание |
|-----|-----------|-------------|------------|
| PC15 | LED красный | Output (PP) | Ошибка |
| PB0 | LED зелёный | Output (PP) | Норма |

### Power Fail

| Пин | Назначание | Направление | Примечание |
|-----|-----------|-------------|------------|
| PB1 | Power Fail | Input (PU) | LOW = питание падает |

Подключение: компаратор или делитель от основного питания.
При падении ниже порога → PB1 = LOW → аварийное сохранение в EEPROM.

## Примечания по питанию

### Уровни напряжения

| Узел | Напряжение | Источник |
|------|-----------|----------|
| STM32 VDD | +3.3V | LDO (AMS1117-3.3) от +12V |
| HD44780 VCC | +5V | Отдельный LDO или от USB |
| SIM800L VCC | 3.4–4.2V | Отдельный LDO (до 2A!) |
| NRI G-13 | +12V DC | От основного БП |
| Хопперы | +12V DC | От основного БП |
| 24C08 VCC | +3.3V | От STM32 питания |

### Ключевые моменты

1. **+12V → NPN → +3.3V.** Все внешние сигналы (NRI, хопперы) идут через
   NPN транзисторы. Транзистор работает как ключ: управление от +3.3V логики,
   нагрузка на +12V стороне.

2. **SIM800L — отдельное питание.** Модем требует 3.4–4.2V и пиковый ток
   до 2A при передаче. Питать от 3.3V STM32 — **нельзя**.
   Использовать отдельный LDO/DC-DC с байпасным конденсатором 1000µF.

3. **I2C pull-up.** SDA/SCL требуют внешние pull-up 4.7kΩ к +3.3V.
   Внутренние pull-up STM32 недостаточны для I2C.

4. **1-Wire pull-up.** PA11 требует 4.7kΩ pull-up к +3.3V.
   Для длинных линий (>10м) — уменьшить до 2.2kΩ.

5. **Общая земля.** Все GND (STM32, SIM800L, NRI, хопперы, дисплей) —
   соединены вместе.

## Нераспределённые пины

Следующие пины STM32F103C8T6 свободны:

| Пин | Примечание |
|-----|-----------|
| PA12 | Свободен (USB D+ на Bluepill) |
| PA15 | Свободен (JTDI, может потребоваться release) |
| PB3 | Свободен (JTDO, может потребоваться release) |
| PB4 | Свободен (NTRST, может потребоваться release) |
| PB5 | Свободен |
| PC13 | Bluepill LED (не используется в проекте) |
| PC14 | Свободен |

**Внимание:** PA15, PB3, PB4 — JTAG пины. При использовании SWD (2 провода)
эти пины доступны как GPIO, но нужно отпустить их через AFIO remap:
`RCC.APB2ENR.modify(|_, w| w.afioen().set_bit()); AFIO.MAPR.modify(|_, w| w.swj_cfg().jtag_disable());`