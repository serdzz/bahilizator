# Заметки по портированию MSP430/C → STM32/Rust

Документ описывает ключевые решения, маппинги и отличия при портировании
Бахилизатора с MSP430F148/F2618 (IAR C) на STM32F103C8T6 (Rust/Embassy).

## Таблица маппинга C → Rust

### Функции

| C (MSP430/IAR) | Rust (STM32/Embassy) | Примечание |
|---|---|---|
| `main()` + `Run()` super-loop | `task_vending()` async fn | Embassy task вместо while(1) |
| `processNormalMode()` | `run_normal_mode()` async fn | Та же state machine, async timers |
| `processPulseMode()` | `run_pulse_mode()` async fn |  |
| `hopperA_process()` | `Hopper::poll()` | Параметризованный — один код для всех |
| `hopperB_process()` | `Hopper::poll()` | Устранён copy-paste |
| `hopperC_process()` | (удалено, 2 хоппера) | В оригинале 3, здесь 2 |
| `clearError(hopperNumber)` | `Hopper::clear_error()` | Hopper A особенность сохранена |
| `gsmHardwarePowerUp()` | `gsm_power_on()` async fn | PWRKEY pulse через Embassy timer |
| `gsmHardwarePowerDown()` | `gsm_power_off()` async fn |  |
| `initGSM_cmd()` | `gsm_init_sequence()` + `GsmAtClient` | Разбито на шаги |
| `gsmProcess()` | `task_gsm()` loop | CMUX декодер + AT парсер |
| `LcdWriteInstruction()` | `Hd44780I2c::write_cmd()` | I2C вместо direct GPIO |
| `LcdWriteData()` | `Hd44780I2c::write_data()` |  |
| `LcdUpdate()` | `Hd44780I2c::refresh()` | Буфер + полное обновление |
| `_flashWrite()` | `flash::save_settings()` | Direct register access (RM0008) |
| `framWrite()` / `framRead()` | `nvram::save_state()` / `load_state()` | FRAM → EEPROM + wear levelling |
| `oneWireReset()` | `OneWire::new()` + `search_next()` | one-wire-bus crate |
| `oneWireTxBit()` | (inside one-wire-bus) | async timers вместо delay loops |
| `oneWireRxBit()` | (inside one-wire-bus) |
| `AddEmptyEvent()` | `EventRing::push()` | Ring buffer вместо FIFO shift |
| `CloseTransaction()` | `event::close_transaction()` | FIFO сдвиг как в оригинале |
| `CreateStateReport()` | `report::generate_report()` | heapless::String<160> |
| `GetButtonPressed()` | EXTI ISR → AtomicU8 → Channel | Debounce 50мс |
| `RunMenu()` | `MenuNavigator` struct | Scroller для длинных строк |

### Типы данных

| C (MSP430) | Rust | Примечание |
|---|---|---|
| `int`, `long` | `i32` | Явный размер |
| `unsigned int` | `u16` / `u32` | По контексту |
| `unsigned char` | `u8` |  |
| `char[]` | `heapless::String<N>` | Без аллокации |
| `struct Bah` | `VendingState` | Разделён на data + settings + errors |
| `enum APP_STATE` | `enum AppState` | `#[repr(u8)]` для совместимости |
| `#define ERROR_xxx (1<<n)` | `bitflags::bitflags!` | Типобезопасно, `Errors::COIN_ACCEPTOR` |
| `const int` | `const` в `config.rs` |  |
| `volatile int` | `AtomicU8` | Для ISR → task обмена |
| Массивы фиксированного размера | `[T; N]` | Стек, без heap |
| Указатели + malloc | `&'static mut T` / `StaticCell<T>` | Embassy spawn pattern |

### Паттерны

| Паттерн C | Паттерн Rust | Примечание |
|---|---|---|
| Super-loop + switch/case | `loop { select! { ... } }` или `match` | Async await вместо polling |
| ISR → global flag | ISR → `AtomicU8` → `Channel` | Безопасная передача в async |
| `__delay_cycles(n)` | `embassy_time::Timer::after_micros()` | Точные таймеры |
| `__enable_interrupt()` | Embassy executor (auto) | Кооперативная многозадачность |
| Function pointer callbacks | async closures / channels | Типобезопасно |
| `memcpy()` | `.clone()` / `copy_from_slice()` | Safe Rust |
| `sprintf()` | `heapless::String` + `write!()` | Без аллокации |
| Manual CRC computation | `crc16()` функции | Та же реализация |
| Register access: `FCTL2 = ...` | `core::ptr::write_volatile()` | Direct register RW |

## Что упрощено

1. **3 хоппера → 2 хоппера.** В оригинале HOPPER_A (товар) + HOPPER_B + HOPPER_C (монеты).
   В порту — HOPPER_A (товар) + HOPPER_B (монеты). `HOPPER_COUNT = 2`.

2. **Copy-paste хопперов → параметризованный драйвер.** В оригинале 3 отдельных функции
   `hopperA_process()`, `hopperB_process()`, `hopperC_process()` с ~90% общим кодом.
   В порту — одна `struct Hopper` с `HopperConfig`.

3. **FRAM → EEPROM 24C08.** В оригинале — SPI FRAM FM25L04 (неограниченная запись).
   В порту — I2C EEPROM с wear levelling (4 сектора, ротация).

4. **Прямой GPIO дисплей → I2C backpack.** В оригинале HD44780 подключён напрямую
   к MSP430 GPIO. В порту — через PCF8574 I2C (2 провода вместо 8+).

5. **AT парсинг упрощён.** В оригинале — полноценный state machine для UART ответов.
   В порту — линейный поиск "OK"/"ERROR" в буфере (достаточно для SIM800L).

6. **EventRing вместо FIFO сдвига.** В оригинале — `memmove` для каждого события.
   В порту — кольцевой буфер (O(1) push, O(n) get_last).

## Что добавлено (новое по сравнению с C)

1. **IWDG (Independent Watchdog).** В оригинале — WDT MSP430.
   В порту — IWDG STM32 (TODO: feed loop в main).

2. **Wear levelling для EEPROM.** 4 сектора с ротацией + magic + CRC.
   В оригинале FRAM не требовала wear levelling (неограниченные циклы записи).

3. **Параметризованные хопперы.** `struct Hopper` с `HopperConfig` — один код
   для всех хопперов вместо copy-paste. HOPPER_A особенность (no start/stop cycle
   при clearError) сохранена через `is_hopper_a: bool`.

4. **defmt логирование.** Структурное логирование через RTT вместо `printf()`
   или `UART_debug_printf()`.

5. **Scroller для длинных строк.** Автопрокрутка текста >16 символов на 16×2.
   В оригинале — обрезка.

6. **Custom chars для латышского.** ā, ē, ī, ū, ķ, ļ, ņ, š — 8 слотов CGRAM.
   В оригинале — 3 символа (ā, ņ, ī).

7. **Bitflags для ошибок.** Типобезопасный `Errors` вместо `#define ERROR_xxx (1<<n)`.
   Поддерживает `|`, `&`, `!`, `contains()`.

8. **CMUX декодер.** Побайтовый state machine для парсинга GSM 07.10 кадров.
   В оригинале — частичный парсинг в `gsmProcess()`.

9. **PPP state machine → embassy-net-ppp.** Вместо ручной LCP/PAP/IPCP реализации —
   embassy-net-ppp + embassy-net. Даёт TCP, UDP, DNS из коробки.

10. **MQTT через rust-mqtt.** Телеметрия, ошибки, события — через MQTT v5.
    Топики: settings, errors, event, state, accounting.

11. **one-wire-bus вместо bit-bang.** Crate `one-wire-bus` для iButton вместо
    ручного управления таймингами. CRC-8 внутри crate.

## Известные отличия от оригинала

1. **GSM — заглушки.** UART запись, I2C запись, GPIO — всё через заглушки
   (stub функции). Реальная интеграция с embassy-stm32 периферией — TODO.

2. **Время (RTC).** В оригинале — MSP430 RTC с батарейкой. В порту —
   запрашивается у SIM800L через `AT+CCLK?`. Нет встроенного RTC STM32.

3. **Power fail detection.** В оригинале — прерывание по снижению питания
   (MSP430 имеет детектор). В порту — PB1 пин (POWER_FAIL_PIN), но обработка
   TODO.

4. **Двери — 2 вместо 3.** В оригинале — 3 двери (основная + 2 сервисных).
   В порту — `MAX_DOORS = 2` (PA7, PA8).

5. **iButton — poll вместо interrupt.** В оригинале — прерывание по 1-Wire.
   В порту — опрос каждые 200мс (менее эффективно, но проще).

6. **Каналы монетоприёмника — 6 вместо 4.** В оригинале — 4 канала.
   В порту — 6 каналов (NRI G-13.6000 поддерживает 6).

7. **Отсутствует CRC проверка при Flash записи.** В оригинале — двойная CRC.
   В порту — single CRC, но проверка magic 0xDEADBEEF.

## Чеклист: что нужно сделать перед production

### Критичное (без этого не работает)

- [ ] **GPIO интеграция.** Заменить все заглушки `read_coin_channels()`,
  `read_coin_pulse_pin()`, `i2c_write_stub`, `Hopper::update_sensors` на реальные
  embassy-stm32 GPIO операции
- [ ] **I2C реальная запись.** `Hd44780I2c::i2c_write` и `nvram::eeprom_*` —
  заменить на `embassy_stm32::i2c::I2c::write()`
- [ ] **UART TX/RX.** GSM модуль — заменить заглушки `send_at_cmd()` на реальные
  `embassy_stm32::usart::Uart` операции
- [ ] **EXTI настройка.** Кнопки и двери — настроить EXTI прерывания через
  `embassy_stm32::exti::ExtiInput`
- [ ] **Hopper GPIO.** Управление моторами (PB15 и т.д.) и чтение датчиков
- [ ] **CMUX UART.** Реальная отправка/приём CMUX кадров через UART
- [ ] **PPP UART.** DLC2 → UART для PPP данных

### Важное (работает, но ненадёжно)

- [ ] **IWDG feed loop.** Заменить TODO в main на реальную обработку watchdog
- [ ] **Power fail handler.** PB1 → детектор снижения питания → аварийное сохранение
- [ ] **CMUX FCS strict check.** Сейчас FCS проверка отключена (SIM800L иногда
  отправляет некорректные кадры). Для production — включить
- [ ] **EEPROM wear levelling — реальное чтение.** `find_active_sector()` — заглушка
- [ ] **AT timeout handling.** Таймауты при ожидании ответа от модема
- [ ] **PPP retry logic.** Обработка LCP Conf-Nak, повторные попытки IPCP

### Желательное (качество)

- [ ] **Настройки меню — запись в State.** `MenuNavigator::apply_edit_value()` — TODO
- [ ] **SMS входящих обработка.** `SmsEvent` отправляется в канал, но потребитель
  не подключён
- [ ] **iButton событие — обработка в vending.** `IBUTTON_CHANNEL` не обрабатывается
  в task_vending (только в task_ibutton для whitelist)
- [ ] **CanAcceptCash — полная проверка.** Упрощённая проверка, TODO: полная как в оригинале
- [ ] **Бесплатная выдача.** Free item switch — EXTI заглушка
- [ ] **Scroller в дисплее.** ScrollBounce для длинных строк — не подключён
- [ ] **GPRS TCP connection.** AT+CIPSTART — не реализовано
- [ ] **День/ночь режим.** `workday_start_hour`/`workday_end_hour` — не используется

### Косметическое

- [x] Устранить warning'и компилятора (clippy clean, 0 warnings)
- [ ] Добавить `.cargo/config.toml` с target по умолчанию
- [ ] Добавить `Embed.toml` для probe-rs
- [ ] CI: `cargo clippy` + `cargo build --release`
- [ ] Тесты для `cmux::parse_cmux_frame`, `crc16`, `check_crc`