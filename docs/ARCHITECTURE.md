# Архитектура Бахилизатора v2.0 (ESP32 / LilyGo T-Call)

Детальное описание программной архитектуры Rust/Embassy порта для ESP32.

## Целевая плата

**LilyGo T-Call SIM800 v20190610**
- ESP32 Xtensa LX6 dual-core, 520KB SRAM, 4MB Flash
- SIM800L onboard (UART2, GPIO26/27)
- IP5306 power management (I2C 0x75)
- I2C0: дисплей (GPIO21/22), I2C1: EEPROM (GPIO18/19)

**Ветка:** `esp32` | **Target:** `xtensa-esp32-none-elf`
**HAL:** `esp-hal` v1.0 | **Framework:** Embassy async

**Сборка:** `cargo +esp build --release -Zbuild-std=core,alloc`

## Карта Embassy задач

| Задача | Приоритет | Вход | Выход | Назначение |
|--------|-----------|------|-------|-----------|
| `task_vending` | default | COIN_CHANNEL, BUTTON_CHANNEL, HOPPER_EVENT_CHANNEL | HOPPER_CMD_CHANNEL, GSM_CMD_CHANNEL, DISPLAY_SIGNAL, PERSIST_SIGNAL | State machine вендинга |
| `task_coin_acceptor` | default | GPIO (poll) | COIN_CHANNEL | Опрос NRI G-13 (1ms) |
| `task_hopper` | default | HOPPER_CMD_CHANNEL | HOPPER_EVENT_CHANNEL | Управление хопперами (1ms poll) |
| `task_buttons` | default | EXTI ISR (AtomicU8) | BUTTON_CHANNEL | Кнопки + двери (10ms poll) |
| `task_gsm` | default | GSM_CMD_CHANNEL | SMS_EVENT_CHANNEL | SIM800L: AT/CMUX/PPP |
| `task_ibutton` | default | GPIO (1-Wire) | IBUTTON_CHANNEL | Опрос DS1990A (200ms) |
| `task_state_persist` | default | PERSIST_SIGNAL | I2C (EEPROM) | Сохранение State (debounce 5s) |
| `display_task` | default | DISPLAY_SIGNAL | I2C (HD44780) | Обновление дисплея |

**Примечание:** Embassy executor на Cortex-M использует кооперативную многозадачность.
Все задачи имеют один приоритет, переключение по `await` точкам.

### Диаграмма потоков данных

```
 ┌──────────────────┐   COIN_CHANNEL(4)   ┌───────────────────────┐
 │  [Coin ISR /     ]│ ─────────────────→ │                       │
 │   Poll 1ms]      │                     │                       │
 └──────────────────┘                     │                       │
                                          │   task_vending        │
 ┌──────────────────┐  BUTTON_CHANNEL(4)  │   ┌─────────────────┐ │
 │  [EXTI ISR /     ]│ ─────────────────→ │   │ AcceptCash     │ │
 │   Poll 10ms]     │                     │   │ PayoutItems    │ │
 └──────────────────┘                     │   │ PayoutReminder │ │
                                          │   │ ProcessResidual│ │
 ┌──────────────────┐  IBUTTON_CHANNEL(1) │   └─────────────────┘ │
 │  [1-Wire /       ]│ ─────────────────→ │                       │
 │   Poll 200ms]    │                     │  HOPPER_CMD_CHANNEL(4)│
 └──────────────────┘                     │ ───────────────────┐  │
                                          │                    ↓  │
 ┌──────────────────┐  HOPPER_EVENT(4)    │           ┌──────────────┐
 │                  │ ←────────────────── │           │ task_hopper  │
 │                  │                     │           │ (poll 1ms)   │
 │                  │                     │           └──────────────┘
 │                  │                     │                       │
 │                  │  GSM_CMD_CHANNEL(4) │                       │
 │                  │ ───────────────────┐│                       │
 │                  │                   ↓│                       │
 │                  │           ┌──────────────┐                 │
 │                  │           │  task_gsm    │                 │
 │                  │           │  (SIM800L)   │                 │
 │                  │           └──────────────┘                 │
 │                  │            │ SMS_EVENT_CHANNEL(4)           │
 │                  │            ↓                                │
 └──────────────────┘           └─→ обработка SMS              │
                                          │                       │
                                          │ DISPLAY_SIGNAL        │
                                          │ ───────────────────┐  │
                                          │                   ↓  │
                                          │           ┌──────────────┐
                                          │           │ display_task │
                                          │           │ (HD44780)   │
                                          │           └──────────────┘
                                          │                       │
                                          │ PERSIST_SIGNAL        │
                                          │ ───────────────────┐  │
                                          │                   ↓  │
                                          │           ┌──────────────┐
                                          │           │task_persist  │
                                          │           │(EEPROM/Flash)│
                                          │           └──────────────┘
                                          └───────────────────────┘
```

### Каналы и сигналы

| Канал/Сигнал | Тип | Ёмкость | Производитель | Потребитель |
|---|---|---|---|---|
| `COIN_CHANNEL` | `Channel<CoinEvent>` | 4 | coin_acceptor | vending |
| `BUTTON_CHANNEL` | `Channel<ButtonEvent>` | 4 | buttons | vending |
| `HOPPER_CMD_CHANNEL` | `Channel<HopperCmd>` | 4 | vending | hopper |
| `HOPPER_EVENT_CHANNEL` | `Channel<HopperEvent>` | 4 | hopper | vending |
| `GSM_CMD_CHANNEL` | `Channel<GsmCommand>` | 4 | vending | gsm |
| `SMS_EVENT_CHANNEL` | `Channel<SmsEvent>` | 4 | gsm | (внешний) |
| `IBUTTON_CHANNEL` | `Channel<IbuttonEvent>` | 1 | ibutton | (внешний) |
| `DISPLAY_SIGNAL` | `Signal<DisplayCommand>` | 1 | vending | display |
| `PERSIST_SIGNAL` | `Signal<PersistReason>` | 1 | все | persist |

**Channel** — MPSC очередь (несколько производителей, один потребитель).
**Signal** — единичное значение, перезаписывается (последний выигрывает).

## Shared state

### VendingState — верхний уровень

```
VendingState
├── data: VendingStateData     ← EEPROM 24C08 (wear levelling)
│   ├── cash: i32              текущая наличность (центы)
│   ├── item_level: i32        остаток товара
│   ├── coin_levels: [i32; 2]  остатки монет в хопперах
│   ├── items_pending: i32     товар к выдаче
│   ├── coins_pending: [i32; 2] монеты сдачи к выдаче
│   ├── app_state: AppState    текущее состояние SM
│   ├── transactions: [TransactionEntry; 5]  лог транзакций
│   ├── events: [EventEntry; 20]             лог событий
│   ├── overall_accounting: Accounting       общий бухгалтерский учёт
│   ├── period_accounting: Accounting        периодический учёт
│   └── messages_pending: [[Errors; 2]; 2]   SMS к отправке
│
├── settings: Settings          ← Flash page 63 (редко пишется)
│   ├── user_language: Language  Latvian/Russian
│   ├── machine_id: i32         ID автомата
│   ├── coin_acceptor: CoinAcceptorSettings
│   │   ├── pulse_mode: bool    нормальный/пульсный режим
│   │   ├── coin_values: [i32; 6]  номиналы по каналам
│   │   └── coin_enable: [bool; 6] разрешение каналов
│   ├── coin_hoppers: [HopperSettings; 2]
│   ├── item_dispenser: HopperSettings
│   ├── keys: [[IbuttonKey; 2]; 3]  whitelist iButton
│   ├── phone_numbers: [[[u8; 16]; 2]; 2]  телефоны для SMS
│   ├── workday_start/end_hour: u8
│   ├── residual_timeout: u8       таймаут сдачи (с)
│   ├── menu_exit_timeout: u8
│   ├── cash_clear_timeout: u8
│   └── crc: u16
│
└── errors: Errors              ← RAM (bitflags, 64 бит)
    ├── FIRMWARE / SETTINGS_VERSION / SETTINGS_CRC
    ├── STATE_VERSION / STATE_CRC
    ├── BATTERY_LOW / FLASH_CORRUPTED / NVRAM_CORRUPTED
    ├── GSM_MODULE / SIM_CARD
    ├── COIN_ACCEPTOR / COIN_ACCEPTOR_OFF
    ├── ITEM_DISPENSER / ITEM_DISPENSER_EMPTY
    ├── COIN_HOPPER / COIN_HOPPER_EMPTY
    ├── CANNOT_PAYOUT
    └── DOOR_OPENED
```

### Доступ к состоянию

Все задачи разделяют `Mutex<CriticalSectionRawMutex, RefCell<VendingState>>`.

**Чтение** (без изменения):
```rust
let guard = state.lock().await;
let s = guard.borrow();
let cash = s.data.cash;
```

**Запись** (с изменением):
```rust
let mut guard = state.lock().await;
let mut s = guard.borrow_mut();
s.data.cash += coin_value;
```

Макрос `read_state!` — сокращение для чтения с автоматическим drop.

## Vending State Machine

### Состояния

```rust
enum AppState {
    AcceptCash = 0,      // Приём монет
    PayoutItems = 1,     // Выдача товара
    PayoutReminder = 2,  // Выдача сдачи
    ProcessResidual = 3, // Финализация / "Спасибо"
}
```

### Диаграмма переходов

```
                    ┌─────────────────────────────┐
                    │        Ошибки?               │
                    │  (errors != NONE)            │
                    │  → "НЕТ ОБСЛУЖИВАНИЯ"       │
                    │  + SMS об ошибках            │
                    └─────────────────────────────┘
                              ↑
                              │ ошибка в любом состоянии
                              │
  ┌──────────┐    монета    ┌──────────────┐    товар выдан   ┌──────────────┐
  │          │  ──сумма───→  │              │ ──────────────→  │              │
  │ Accept   │    ≥ цена     │  Payout      │                  │  Payout      │
  │ Cash     │               │  Items       │                  │  Reminder    │
  │          │               │  (хоппер A)  │                  │  (хоппер B)  │
  └────┬─────┘               └──────────────┘                  └──────┬───────┘
       ↑                                                             │
       │                                                    сдача выдана /
       │                                                    таймаут 15с
       │                                                             ↓
       │               ┌──────────────┐                            
       │               │              │                            
       └───────────────│  Process     │                            
         сброс         │  Residual    │                            
         pending       │              │                            
                       └──────────────┘                            
```

### Логика каждого состояния

**AcceptCash:**
1. Проверить `can_accept_cash()` — есть ли товар и сдача
2. Показать "ВСТАВЬТЕ МОНЕТЫ" / "Цена: X"
3. Ждать монеты (COIN_CHANNEL) и кнопки (BUTTON_CHANNEL)
4. Каждая монета → `cash += coin.value`, обновить дисплей
5. Если `cash >= item_price` → вычислить payout, перейти в PayoutItems
6. Таймаут бездействия → сброс наличности

**PayoutItems:**
1. Показать "ВЫДАЧА ТОВАРА"
2. Отправить `HopperCmd::Payout { hopper: 0, count: items_pending }`
3. Считать `HopperEvent::CoinDispensed` → обновить дисплей "0/N шт"
4. Ошибка/таймаут хоппера → установить ошибку, SMS

**PayoutReminder:**
1. Показать "ЗАБЕРИТЕ СДАЧУ"
2. Выдавать монеты из хопперов (от большего номинала к меньшему)
3. Каждая монета → `cash_out += coin_value`, обновить уровни
4. Таймаут 15с → "наказание" (не забрал сдачу → -1 монета)
5. Ошибка хоппера → установить ошибку, SMS

**ProcessResidual:**
1. Есть невыданное → "НЕ ВЫДАНО Тов:X Сдач:Y" + закрыть транзакцию
2. Всё выдано → "СПАСИБО! ЗА ПОКУПКУ"
3. Через 3с → сброс pending, переход в AcceptCash

## Протоколы

### CMUX (GSM 07.10 Basic Mode)

Формат кадра:
```
┌──────┬─────────┬─────────┬──────────┬──────┬─────┬──────┐
│  F9  │ Address │ Control │ Length   │ Data │ FCS │  F9  │
│ flag │ 1 байт  │ 1 байт  │ 1-2 байт │ N B  │ 1 B │ flag │
└──────┴─────────┴─────────┴──────────┴──────┴─────┴──────┘

Address: EA(1) | CR(1) | DLCI(6)
Control: SABM=0x2F, UA=0x63, UIH=0xEF, DISC=0x43, DM=0x0F
FCS: CRC-8 по address + control (полином 0x07, reflected 0x8C)
```

DLCI каналы:
- **DLC0** — управление мультиплексором (MSC)
- **DLC1** — AT команды (SMS, конфигурация)
- **DLC2** — PPP данные (GPRS интернет)

Подробнее: [CMUX_PPP.md](CMUX_PPP.md)

### AT команды (DLC1)

Последовательность инициализации SIM800L:
```
ATE0          → выключить эхо
AT+CMGF=1     → текстовый режим SMS
AT+CNMI=2,2,0,0,0 → прямая доставка SMS
AT+CSMP=17,167,0,0 → параметры SMS
AT+CCLK?      → получить время модема
```

GPRS подключение:
```
AT+CGATT=1    → GPRS attach
AT+CSTT="internet" → APN
AT+CIICR      → поднять соединение
AT+CIFSR      → получить IP
```

### PPP (embassy-net-ppp) — DLC2

PPP через embassy-net-ppp — полная интеграция с Embassy сетевым стеком.
После PPP подключения получаем `embassy_net::Stack` с:
- IPv4 + DHCP клиент
- TCP и UDP сокеты
- DNS резолвер

```rust
// Инициализация
let device = embassy_net_ppp::Runner::new(rx, tx);
let config = embassy_net::Config::dhcpv4();
let (stack, runner) = embassy_net::new(device, config);

// Использование
let mut socket = TcpSocket::new(stack, &mut rx_buf, &mut tx_buf);
socket.connect(remote_addr, port).await;
```

### DNS (embassy-net)

DNS через embassy-net Stack (smoltcp внутри):

```rust
let ip = stack.dns_query("broker.example.com", DnsQueryType::A).await;
```

Кэш: 4 записи (hostname → [u8; 4]), TTL 300с.
Fallback DNS: 8.8.8.8 / 8.8.4.4.

### MQTT (rust-mqtt v5)

MQTT v5 клиент поверх embassy-net TCP:

```rust
let transport = TcpTransport::new(stack, &mut rx_buf, &mut tx_buf);
let client = Client::new(transport, "bahilizator-42", KeepAlive::from_secs(60));
client.connect(broker_ip, 1883).await;
client.publish("bahilizator/42/state", &payload, QoS::AtMostOnce).await;
```

Топики:
| Топик | Период | Содержание |
|-------|--------|------------|
| `bahilizator/{id}/settings` | по изменению | Конфигурация JSON |
| `bahilizator/{id}/errors` | сразу при ошибке | Флаги ошибок JSON |
| `bahilizator/{id}/event` | batched 5с | События (CoinIn, ItemDispensed...) JSON |
| `bahilizator/{id}/state` | каждые 5 мин | Текущее состояние JSON |
| `bahilizator/{id}/accounting` | каждый час | Бухгалтерский учёт JSON |

Keepalive: 60с. Reconnect: каждые 30с при потере.

### 1-Wire (one-wire-bus crate)

iButton через one-wire-bus вместо ручного bit-bang:

```rust
let mut onewire = OneWire::new(pin, false);
let mut search = Search::new();
while let Some(device) = onewire.search_next(&mut search, delay).ok() {
    // device.address — ROM код DS1990A
}
```

CRC-8 Dallas/Maxim проверяется внутри crate.

### I2C0 — Дисплей (GPIO21/GPIO22)

I2C0 эксклюзивно для дисплея:
- **PCF8574** (0x27) — HD44780 backpack, частые записи
- **IP5306** (0x75) — Power management IC

Частота: 100 kHz. Передаётся по значению в `task_display`.

### I2C1 — EEPROM (GPIO18/GPIO19)

I2C1 для персистентного хранения:
- **24C08** (0x50) — EEPROM 1024 байт, wear-levelling

Частота: 100 kHz. Через `StaticCell` + `nvram::set_i2c()`.

**Разделение шин**: I2C0 (дисплей) и I2C1 (EEPROM) на разных I2C контроллерах.
Устраняет конфликт владения и contention на шине.

### 1-Wire — не подключён на LilyGo T-Call

GPIO4 занят под SIM800L PWRKEY. Нет свободного output-capable GPIO
для 1-Wire Open-Drain. EspHalPin — GPIO пин-заглушка, позволяет
скомпилировать код, но 1-Wire не работает.

Для реального iButton: освободить GPIO0, GPIO2 или GPIO32.

## Память

### Flash (4MB, esp-storage)

ESP32 4MB Flash разделён через partition table:
- 0x0000–0x8000: bootloader
- 0x8000–0x9000: partition table
- 0x9000–0x3D000: приложение (~212KB)
- 0x3D000–0x3F000: NVS (Non-Volatile Storage)
- Другие разделы по partition table

Settings хранятся в NVS (esp-storage) вместо Flash page.

### EEPROM 24C08 layout (Wear Levelling)

```
0x000 ┌──────────────────────┐  Sector 0
      │ magic 0xBABE (2B)    │
      │ CRC16 (2B)           │
      │ VendingStateData     │
      │ (N байт)             │
0x0FF └──────────────────────┘
0x100 ┌──────────────────────┐  Sector 1
      │ ...                  │
0x1FF └──────────────────────┘
0x200 ┌──────────────────────┐  Sector 2
      │ ...                  │
0x2FF └──────────────────────┘
0x300 ┌──────────────────────┐  Sector 3
      │ ...                  │
0x3FF └──────────────────────┘
```

**Wear levelling:** 4 сектора по 256 байт, ротация при каждой записи.
1. Активный сектор = найден по magic 0xBABE + валидный CRC
2. Запись: следующий сектор → записать CRC + State → записать magic → обнулить magic старого
3. Чтение: поиск по секторам, первый валидный

**Страницы 24C08:** 8 байт (page-write). Запись постраничная.

### RAM usage

```
┌────────────────────────────────────────────┐
│ .bss / .data                               │
│   VendingState (~600B)                     │
│   UART2 TX/RX static refs                 │
│   I2C1 static ref (EEPROM)                 │
│   CmuxDecoder (~302B)                      │
│   Statics (channels, signals, atoms)       │
│   Task stacks (Embassy internal)           │
│   defmt-espflash buffer                    │
│                                            │
│ Heap: нет (no_std, heapless only)          │
│ ESP32: 520KB SRAM, используется ~194KB     │
│ Flash: ~80KB из 4MB                        │
└────────────────────────────────────────────┘
```

## Логирование и отладка

### defmt

Все модули используют `defmt::info!`, `defmt::trace!`, `defmt::warn!`, `defmt::error!`.
Транспорт: `defmt-rtt` (Real-Time Transfer через SWD).

### Ошибки

`Errors` — bitflags (u64), до 18 категорий ошибок. Отображается как hex:
"ERR 20000" = ITEM_DISPENSER_EMPTY.

### Логи

- **EventRing** — кольцевой буфер на 32 события (RAM, не персистентный)
- **TransactionEntry** — FIFO буфер на 5 транзакций (в EEPROM через VendingStateData)
- Форматирование: `format_event()` → `heapless::String<32>`

## SMS отчёты

Генерируются в `report.rs`, отправляются через `task_gsm`:

| Тип | Формат SMS |
|-----|-----------|
| State | `ID:x Item:xx H1:xx H2:xx Cash:xx Err:no` |
| Period accounting | `Period IN:xx OUT:xx PROF:xx SOLD:xx FREE:xx` |
| Overall accounting | `Overall IN:xx ...` |
| Errors | `ERR:hex CASH:xx ITEM:xx DOOR:0/1` |
| Intrusion | `ALARM! Door open! Cash:xx` |
| Power up | `BAH #x UP` |
| Hopper warning | `LOW! H1:xx H2:xx` |
| Item warning | `LOW! Item:xx` |