# Протокол CMUX + PPP — Бахилизатор

Документ описывает реализацию мультиплексирования GSM 07.10 (CMUX)
и PPP поверх CMUX для SIM800L в проекте Бахилизатор.

## Обзор

SIM800L подключён к STM32F103C8T6 через один UART (USART1, 115200 8N1).
Для одновременной работы AT команд и PPP данных используется CMUX —
мультиплексирование по стандарту GSM 07.10.

```
STM32 USART1 (PA9/PA10)
       │
       ↓ 115200 8N1
  ┌─────────┐
  │ SIM800L │
  │  CMUX   │
  └────┬────┘
       │
  ┌────┴───────────────┐
  │                    │
  ↓                    ↓
DLC1: AT команды    DLC2: PPP данные
(SMS, конфиг)       (GPRS интернет)
```

## GSM 07.10 CMUX — Basic Mode

### Формат кадра

```
┌──────┬─────────┬──────────┬──────────┬──────┬─────┬──────┐
│ FLAG │ ADDRESS │ CONTROL  │ LENGTH   │ DATA │ FCS │ FLAG │
│ 0xF9 │ 1 байт  │ 1 байт   │ 1-2 байта│ N B  │ 1 B │ 0xF9 │
└──────┴─────────┴──────────┴──────────┴──────┴─────┴──────┘
```

#### Address (1 байт)

```
┌─────┬──────┬──────────────────────────────┐
│ EA  │  CR  │          DLCI                │
│  1  │ 1/0  │        6 бит                 │
└─────┴──────┴──────────────────────────────┘

EA  = 1 (Extension bit, всегда 1 для 1-байтового адреса)
CR  = Command/Response (1 = команда от инициатора)
DLCI = Data Link Connection Identifier (0-63)
```

- **EA=1** — однобайтовый адрес (Basic Mode всегда использует 1 байт)
- **CR=1** — STM32 → модем (команда)
- **CR=0** — модем → STM32 (ответ)

#### Control (1 байт)

| Тип | Значение | Назначение |
|-----|----------|-----------|
| SABM | 0x2F | Установить канал (Set Asynchronous Balanced Mode) |
| UA | 0x63 | Подтверждение установки |
| DM | 0x0F | Отказ (Disconnected Mode) |
| DISC | 0x43 | Разорвать канал |
| UIH | 0xEF | Данные с header check (Unnumbered Information with Header check) |
| UI | 0x03 | Данные без контроля |

#### Length (1-2 байта)

```
1 байт:  EA=1 | длина(7 бит)          → длина 0-127
2 байта: EA=0 | длина(7 бит) | старшие биты  → длина 0-32767
```

Basic Mode обычно использует 1-байтовую длину (данные до 127 байт).

#### FCS (1 байт)

CRC-8 по полям **Address + Control** (только 2 байта).

Полином: x⁸+x²+x+1 = 0x07 (reflected = 0x8C).
Инициализация: 0xFF. Результат: complement.

```rust
fn compute_fcs(data: &[u8]) -> u8 {
    let mut fcs: u8 = 0xFF;
    for &byte in data {
        fcs ^= byte;
        for _ in 0..8 {
            if fcs & 0x01 != 0 {
                fcs = (fcs >> 1) ^ 0x8C;
            } else {
                fcs >>= 1;
            }
        }
    }
    !fcs
}
```

### DLCI каналы

| DLCI | Назначение | Использование |
|------|-----------|--------------|
| 0 | Управление мультиплексором | MSC (Modem Status Command) — flow control |
| 1 | AT команды | SMS, конфигурация, GPRS attach |
| 2 | PPP данные | LCP, PAP, IPCP, IP пакеты |

### Последовательность установки CMUX

```
STM32                              SIM800L
  │                                    │
  │──── AT+CMUX=0,1,5,128,10,3,30,10,2 ────→│  Включить CMUX
  │←─── OK ───────────────────────────────│
  │                                    │
  │──── SABM DLC0 ────────────────────→│  Установить DLC0
  │←─── UA DLC0 ──────────────────────│
  │                                    │
  │──── SABM DLC1 ────────────────────→│  Установить DLC1
  │←─── UA DLC1 ──────────────────────│
  │                                    │
  │──── SABM DLC2 ────────────────────→│  Установить DLC2
  │←─── UA DLC2 ──────────────────────│
  │                                    │
  │     ═══ CMUX активен ═══          │
  │                                    │
  │── UIH DLC1 [AT cmd] ────────────→│  AT команда
  │←── UIH DLC1 [AT resp] ───────────│  AT ответ
  │                                    │
  │── UIH DLC2 [PPP frame] ─────────→│  PPP данные
  │←── UIH DLC2 [PPP frame] ────────│  PPP данные
```

### Параметры AT+CMUX

```
AT+CMUX=0,1,5,128,10,3,30,10,2
       │ │ │  │   │ │  │  │ └── N2: макс. ретрансмиссий
       │ │ │  │   │ │  │  └──── T2: таймаут acknowledgment (×10ms)
       │ │ │  │   │ │  └────── T1: таймаут (×100ms)
       │ │ │  │   │ └──────── N1: макс. размер кадра
       │ │ │  │   └────────── TI: таймер (×100ms)
       │ │ │  └──────────── M: макс. кадров в окне
       │ │ └─────────────── F: формат Basic Mode = 5
       │ └────────────────- Subtype: 1 = Basic
       └────────────────── Mode: 0 = Basic
```

### MSC (Modem Status Command) — DLC0

Управление потоком данных на конкретном DLCI:

```
MSC SET (FC=1): остановить передачу на DLCI
MSC SET (FC=0): возобновить передачу на DLCI
```

Формат MSC данных:
```
┌───────┬──────────────┬────────┬───────────────┐
│ 0xFD  │ DLCI address │ 0x01   │ Signals byte  │
│ MSC   │ (EA=1,CR=1)  │ Len=1  │ FC|RTC|RTR|DV │
└───────┴──────────────┴────────┴───────────────┘

Signals: FC(бит1) RTC(бит2) RTR(бит3) DV(бит4)
FC=1 → flow control stop
FC=0 → flow control resume
```

### Декодер CmuxDecoder

Побайтовый state machine для парсинга входящих кадров из UART:

```
┌─────────┐  0xF9   ┌──────────┐  данные  ┌───────────┐  0xF9   ┌────────┐
│  IDLE   │────────→│ IN_FRAME │────────→│ IN_FRAME  │────────→│ PARSE  │
│         │←────────│          │←────────│           │←────────│        │
└─────────┘  timeout └──────────┘ overflow └───────────┘  empty  └────────┘
```

Методы:
- `feed_byte(byte)` → `Option<CmuxFrame>` — побайтовый ввод
- `feed_slice(bytes)` → `Vec<CmuxFrame, 4>` — пакетный ввод

## AT команды — DLC1

### atat интеграция

В текущей реализации используется **ручное формирование** AT команд
(без derive макросов atat). Причина: ограниченный набор команд SIM800L,
нестандартные ответы, CMUX обёртка.

Класс `CmuxAtChannel` — обёртка:
- `encode_at_cmd(cmd)` — закодировать AT команду в CMUX UIH кадр DLC1
- `feed_rx_data(data)` — добавить данные из DLC1 в буфер
- `parse_response()` → `Option<AtResponse>` — найти OK/ERROR/Prompt
- `parse_urc()` → `Option<Urc>` — найти unsolicited result code

### Обработка ответов

```
AtResponse::Ok           ← "OK"
AtResponse::Error        ← "ERROR"
AtResponse::CmeError(n)  ← "+CME ERROR: n"
AtResponse::CmsError(n)  ← "+CMS ERROR: n"
AtResponse::Prompt       ← ">"
AtResponse::Timeout      ← нет ответа за 5с
```

### URC (Unsolicited Result Codes)

SIM800L отправляет URC асинхронно:

| URC | Описание |
|-----|----------|
| `+CMT:` | Входящее SMS |
| `Call Ready` | Модем готов |
| `NORMAL POWER DOWN` | Выключение |
| `UNDER-VOLTAGE WARNING` | Низкое напряжение |
| `OVER-VOLTAGE WARNING` | Высокое напряжение |
| `+CMGS: id` | SMS отправлен |
| `+CGEV:` | GPRS событие |

### SMS отправка через DLC1

```
STM32                              SIM800L
  │──── UIH DLC1 [AT+CMGS="number"\r] ──→│
  │←─── UIH DLC1 [>] ──────────────────│  Prompt
  │──── UIH DLC1 [text + 0x1A] ────────→│  Текст + Ctrl+Z
  │←─── UIH DLC1 [+CMGS: id\r\nOK] ──│  Подтверждение
```

## PPP — DLC2

### Обзор

PPP (Point-to-Point Protocol) поверх CMUX DLC2 обеспечивает GPRS интернет.

State machine:
```
Dead ──→ Establish ──→ Authenticate ──→ Network ──→ Open
          (LCP)          (PAP)          (IPCP)      (IP data)
  ↑                                                    │
  └─────── LCP Terminate-Req ←─────────────────────────┘
```

**Важно:** При работе через CMUX PPP кадры передаются БЕЗ HDLC framing
(флаги 0x7E, FCS-16, byte-stuffing). CMUX обеспечивает целостность доставки.

### LCP (Link Control Protocol)

#### LCP Configure-Request (от нас)

Опции:
| Опция | Тип | Значение | Назначение |
|-------|-----|----------|-----------|
| MRU | 1 | 296 | Maximum Receive Unit |
| Auth Protocol | 3 | 0xC023 (PAP) | Метод аутентификации |
| Magic Number | 5 | 0x00001234 | Loop detection |
| ACCM | 8 | 0x00000000 | Async Control Character Map |

Формат LCP пакета:
```
┌───────┬──────┬──────────┬────────────┐
│ Code  │  ID  │ Length   │ Options    │
│ 1 B   │ 1 B  │ 2 B      │ Variable   │
└───────┴──────┴──────────┴────────────┘

Code = 1 (Configure-Request)
Length = общий размер (header + options)
```

#### LCP коды ответов

| Code | Значение | Действие |
|------|----------|----------|
| 1 | Configure-Request | Запрос конфигурации |
| 2 | Configure-Ack | Принять все опции |
| 3 | Configure-Nak | Отклонить, предложить другие значения |
| 4 | Configure-Reject | Отклонить, опция не распознана |
| 5 | Terminate-Request | Закрыть соединение |
| 6 | Terminate-Ack | Подтвердить закрытие |
| 9 | Echo-Request | Проверка связи |
| 10 | Echo-Reply | Ответ на Echo-Request |

### PAP (Password Authentication Protocol)

SIM800L использует PAP с **пустым логином и паролем**.

Формат PAP Authenticate-Request:
```
┌───────┬──────┬──────────┬──────────────┬────────────────┬──────────────┐
│ Code  │  ID  │ Length   │ Peer-ID-Len  │ Password-Len   │ (empty)      │
│ 1 B   │ 1 B  │ 2 B      │ 1 B (=0)     │ 1 B (=0)       │              │
└───────┴──────┴──────────┴──────────────┴────────────────┴──────────────┘

Code = 1 (Authenticate-Request)
Peer-ID-Length = 0 (пустой логин)
Password-Length = 0 (пустой пароль)
```

Ответы:
- Code = 2 → Authenticate-Ack (успех)
- Code = 3 → Authenticate-Nak (отказ)

### IPCP (Internet Protocol Control Protocol)

#### IPCP Configure-Request (от нас)

| Опция | Тип | Значение | Назначение |
|-------|-----|----------|-----------|
| IP-Address | 3 | 0.0.0.0 | Запросить IP у сервера |
| Primary DNS | 129 | 0.0.0.0 | Запросить DNS |

#### IPCP Configure-Nak (от сервера)

Сервер отвечает Nak с назначенным IP адресом:

```
┌───────┬──────┬──────────┬──────────────────────────┐
│ Code=3│  ID  │ Length   │ Options:                │
│       │      │          │  Type=3, Len=6, IP addr │
└───────┴──────┴──────────┴──────────────────────────┘
```

Парсинг: ищем опцию Type=3, Len=6, читаем 4 байта IP.

### PPP frame через CMUX

При инкапсуляции PPP в CMUX:

```
CMUX frame:
  F9 | Address(DLCI=2) | Control(UIH) | Length | PPP_data | FCS | F9

PPP_data (без HDLC framing):
  Protocol(2B) + LCP/IPCP/PAP payload

Пример LCP Configure-Request:
  Protocol = 0xC0 0x21 (LCP)
  Payload  = Code=1, ID=x, Length, Options...
```

## SIM800L Init Sequence — полная

### Этапы инициализации

```
┌─────────────────────────────────────────────────────────┐
│ Этап 1: Питание                                         │
│                                                         │
│ PWRKEY → HIGH (100мс)                                   │
│ PWRKEY → LOW (1с)       ← pulse для включения           │
│ PWRKEY → HIGH (3с)                                      │
│ Проверка STATUS = HIGH                                  │
│                                                         │
│ Ждём "Call Ready" URC (до 15с)                          │
├─────────────────────────────────────────────────────────┤
│ Этап 2: AT команды                                      │
│                                                         │
│ ATE0\r              → OK  (выключить эхо)               │
│ AT+CMGF=1\r         → OK  (текстовый режим SMS)         │
│ AT+CNMI=2,2,0,0,0\r → OK  (прямая доставка SMS)         │
│ AT+CSMP=17,167,0,0\r → OK  (параметры SMS)              │
│ AT+CCLK?\r          → OK  (время модема)                │
├─────────────────────────────────────────────────────────┤
│ Этап 3: CMUX                                            │
│                                                         │
│ AT+CMUX=0,1,5,128,10,3,30,10,2\r → OK                  │
│ Ждём 200мс                                              │
│ SABM DLC0 → UA                                         │
│ SABM DLC1 → UA                                         │
│ SABM DLC2 → UA                                         │
├─────────────────────────────────────────────────────────┤
│ Этап 4: GPRS                                            │
│                                                         │
│ [DLC1] AT+CGATT=1\r    → OK  (GPRS attach)             │
│ [DLC1] AT+CSTT="internet"\r → OK  (APN)                │
│ [DLC1] AT+CIICR\r      → OK  (GPRS connect)             │
│ [DLC1] AT+CIFSR\r      → 10.x.x.x  (IP адрес)          │
├─────────────────────────────────────────────────────────┤
│ Этап 5: PPP (опционально)                               │
│                                                         │
│ [DLC1] ATD*99***1#\r   → CONNECT                       │
│ [DLC2] LCP Configure-Request → LCP Configure-Ack       │
│ [DLC2] PAP Authenticate-Request → PAP Authenticate-Ack │
│ [DLC2] IPCP Configure-Request → IPCP Configure-Nak     │
│         (Nak содержит назначенный IP)                   │
│ [DLC2] IPCP Configure-Request (с IP) → IPCP Conf-Ack   │
│                                                         │
│ ═══ PPP OPEN — можно отправлять IP пакеты ═══           │
└─────────────────────────────────────────────────────────┘
```

### Таймауты

| Операция | Таймаут | Примечание |
|----------|---------|-----------|
| PWRKEY pulse | 1с | Минимум 100мс по даташиту |
| Call Ready | 15с | Зависит от SIM и сети |
| AT команда | 5с | Обычный ответ |
| SMS отправка | 10с | Дольше из-за сети |
| GPRS attach | 60с | Проверка каждые 60с |
| LCP negotiation | 3с × 5 попыток | 15с максимум |
| PAP auth | 5с | Обычно мгновенно |
| IPCP negotiation | 5с × 5 попыток | 25с максимум |

### Обработка ошибок

| Ошибка | Действие |
|--------|----------|
| PWRKEY — STATUS не HIGH | Повторить pulse, до 3 попыток |
| AT — NoResponse | Проверить UART, повторить |
| AT — CME Error | Зависит от кода: SIM? сеть? |
| CMUX SABM → DM | Повторить SABM, до 3 раз |
| PPP LCP timeout | Переподключить PPP |
| PPP PAP Nak | Несовместимая конфигурация |
| IPCP timeout | Переподключить GPRS |

## Реализация в коде

### Файлы

| Файл | Назначение |
|------|-----------|
| `src/gsm/cmux.rs` | Кодер/декодер CMUX кадров |
| `src/gsm/at_channel.rs` | AT канал поверх DLC1 |
| `src/gsm/ppp_channel.rs` | PPP state machine поверх DLC2 |
| `src/gsm/sms.rs` | SMS AT команды |
| `src/gsm/mod.rs` | GSM менеджер: питание, init, GPRS |

### Ключевые структуры

```rust
// CMUX декодер — побайтовый парсер
CmuxDecoder::feed_byte(byte) → Option<CmuxFrame>

// CMUX кодер — построение кадров
cmux::encode_cmux_frame(dlci, control, data) → Vec<u8, 300>
cmux::encode_sabm(dlci) → Vec<u8, 300>
cmux::encode_ua(dlci) → Vec<u8, 300>

// AT канал
CmuxAtChannel::encode_at_cmd(cmd) → Vec<u8, 300>
CmuxAtChannel::parse_response() → Option<AtResponse>
CmuxAtChannel::parse_urc() → Option<Urc>

// PPP канал
PppChannel::connect() → Result<(), PppError>
PppChannel::disconnect() → ()
PppChannel::process_incoming(data) → Option<Vec<u8, 256>>
PppChannel::send_ip_packet(packet) → Result<(), PppError>
```

### Тестирование CMUX

Для тестирования без железа можно использовать mock UART:

```rust
// Пример: закодировать и распарсить SABM DLC1
let frame = cmux::encode_sabm(1);
let mut decoder = CmuxDecoder::new();
for &byte in &frame {
    if let Some(parsed) = decoder.feed_byte(byte) {
        assert_eq!(parsed.dlci, 1);
        assert_eq!(parsed.control, SABM);
    }
}
```