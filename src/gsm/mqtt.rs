//! MQTT клиент — rust-mqtt (MQTT v5.0) поверх embassy-net TCP
//!
//! Используем crate `rust-mqtt` для MQTT v5.0 протокола.
//! Транспорт: embassy-net TCP сокет → rust-mqtt Client.
//!
//! Архитектура:
//!   - MqttConfig хранит конфигурацию (broker, machine_id)
//!   - publish_once() — разовая публикация: connect → publish → disconnect
//!   - subscribe_and_listen() — подписка и приём настроек
//!   - Для постоянного соединения нужен отдельный embassy task
//!
//! Жизненный цикл (разовая публикация):
//!   1. DNS резолвинг брокера
//!   2. TCP соединение через embassy-net
//!   3. MQTT CONNECT (v5.0, clean start, keepalive 60с)
//!   4. PUBLISH с QoS 0 или 1
//!   5. MQTT DISCONNECT

use core::fmt::Write;

use embassy_net::tcp::TcpSocket;
use embassy_net::Stack;
use embassy_time::{Duration, Instant};

use rust_mqtt::buffer::BufferProvider;
use rust_mqtt::client::event::Event;
use rust_mqtt::client::options::{
    ConnectOptions, PublicationOptions, SubscriptionOptions, TopicReference,
};
use rust_mqtt::client::Client;
use rust_mqtt::config::KeepAlive;
use rust_mqtt::types::{MqttString, QoS, TopicFilter, TopicName};

use crate::error::MqttError;
use crate::gsm::dns::DnsResolver;
use crate::gsm::mqtt_topics;

// ── Константы ────────────────────────────────────────────────────────────

/// Размер TCP rx буфера
const TCP_RX_BUF_SIZE: usize = 1024;
/// Размер TCP tx буфера
const TCP_TX_BUF_SIZE: usize = 1024;
/// Размер bump buffer для rust-mqtt
const MQTT_BUFFER_SIZE: usize = 512;
/// Порт MQTT брокера
pub const MQTT_PORT: u16 = 1883;
/// MQTT Client ID максимальная длина
const CLIENT_ID_LEN: usize = 32;

// ── BumpBuffer для rust-mqtt ─────────────────────────────────────────────

/// Bump аллокатор для rust-mqtt BufferProvider
pub struct BumpBuffer {
    buffer: [u8; MQTT_BUFFER_SIZE],
    cursor: usize,
}

impl Default for BumpBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl BumpBuffer {
    /// Создать новый пустой буфер
    pub const fn new() -> Self {
        Self {
            buffer: [0u8; MQTT_BUFFER_SIZE],
            cursor: 0,
        }
    }

    /// Сбросить курсор
    pub fn reset(&mut self) {
        self.cursor = 0;
    }
}

impl<'a> BufferProvider<'a> for BumpBuffer {
    type Buffer = &'a mut [u8];
    type ProvisionError = ();

    fn provide_buffer(&mut self, len: usize) -> Result<Self::Buffer, Self::ProvisionError> {
        if self.cursor + len > MQTT_BUFFER_SIZE {
            return Err(());
        }
        let start = self.cursor;
        self.cursor += len;
        // Safety: rust-mqtt гарантирует, что буфер используется только
        // внутри одного вызова и живёт не дольше &mut self.
        Ok(unsafe { &mut *(&mut self.buffer[start..start + len] as *mut [u8]) })
    }
}

// ── TcpTransport — адаптер embassy-net TCP → rust-mqtt Transport ────────

/// Обёртка TcpSocket, реализующая Read + Write (Transport)
pub struct TcpTransport<'a> {
    socket: TcpSocket<'a>,
}

impl<'a> TcpTransport<'a> {
    /// Создать транспорт из TCP сокета
    pub fn new(socket: TcpSocket<'a>) -> Self {
        Self { socket }
    }
}

impl embedded_io_async::ErrorType for TcpTransport<'_> {
    type Error = embassy_net::tcp::Error;
}

impl embedded_io_async::Read for TcpTransport<'_> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.socket.read(buf).await
    }
}

impl embedded_io_async::Write for TcpTransport<'_> {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.socket.write(buf).await
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        self.socket.flush().await
    }
}

// ── MqttConfig — конфигурация MQTT клиента ─────────────────────────────

/// Конфигурация MQTT клиента (без состояния соединения)
///
/// Хранит настройки брокера и machine_id.
/// Не владеет TCP сокетами или буферами —
/// они аллоцируются при подключении.
pub struct MqttConfig {
    /// MQTT broker hostname
    pub broker_host: heapless::String<48>,
    /// MQTT broker fallback IP (если DNS не работает)
    pub broker_fallback_ip: [u8; 4],
    /// Machine ID для topic построения
    pub machine_id: i32,
}

impl MqttConfig {
    /// Создать конфигурацию MQTT клиента
    pub fn new(broker_host: &str, broker_fallback_ip: [u8; 4], machine_id: i32) -> Self {
        let mut host = heapless::String::<48>::new();
        host.push_str(broker_host).ok();
        Self {
            broker_host: host,
            broker_fallback_ip,
            machine_id,
        }
    }

    /// Построить Client ID из machine_id
    pub fn client_id(&self) -> heapless::String<CLIENT_ID_LEN> {
        let mut id = heapless::String::new();
        let _ = write!(id, "bahilizator_{}", self.machine_id);
        id
    }
}

// ── publish_once — разовая публикация ────────────────────────────────────

/// Подключиться к брокеру, опубликовать одно сообщение, отключиться.
///
/// Параметры:
///   - stack: embassy-net Stack (PPP поднят)
///   - dns: DNS резолвер для брокера
///   - config: конфигурация MQTT (broker, machine_id)
///   - topic: полный топик (например "bahilizator/1/state")
///   - payload: JSON payload (до 256 байт)
///   - qos: QoS уровень (0, 1 или 2)
pub async fn publish_once(
    stack: Stack<'_>,
    dns: &mut DnsResolver<'_>,
    config: &MqttConfig,
    topic: &str,
    payload: &str,
    qos: QoS,
) -> Result<(), MqttError> {
    // DNS резолвинг брокера
    let broker_ip = dns
        .resolve_or_fallback(&config.broker_host, config.broker_fallback_ip)
        .await;

    defmt::debug!(
        "MQTT: connecting to {}.{}.{}.{}:{}",
        broker_ip[0],
        broker_ip[1],
        broker_ip[2],
        broker_ip[3],
        MQTT_PORT
    );

    // TCP соединение
    let mut rx_buf = [0u8; TCP_RX_BUF_SIZE];
    let mut tx_buf = [0u8; TCP_TX_BUF_SIZE];
    let mut socket = TcpSocket::new(stack, &mut rx_buf, &mut tx_buf);

    let remote = (
        embassy_net::Ipv4Address::new(broker_ip[0], broker_ip[1], broker_ip[2], broker_ip[3]),
        MQTT_PORT,
    );
    socket
        .connect(remote)
        .await
        .map_err(|_| MqttError::TcpConnectFailed)?;

    defmt::debug!("MQTT: TCP connected");

    // MQTT CONNECT
    let transport = TcpTransport::new(socket);
    let mut buffer = BumpBuffer::new();

    let mut client = Client::<TcpTransport<'_>, BumpBuffer, 1, 1, 1, 0>::new(&mut buffer);

    let keepalive = KeepAlive::Seconds(
        core::num::NonZero::new(mqtt_topics::MQTT_KEEPALIVE_SECS)
            .unwrap_or_else(|| core::num::NonZero::new(60).unwrap_or_else(|| unreachable!())),
    );

    let connect_options = ConnectOptions::new().clean_start().keep_alive(keepalive);

    let client_id_str = config.client_id();
    let client_id = MqttString::from_str(&client_id_str).map_err(|_| MqttError::BufferTooSmall)?;

    client
        .connect(transport, &connect_options, Some(client_id))
        .await
        .map_err(|e| {
            defmt::error!("MQTT: connect error: {:?}", e);
            MqttError::ConnectFailed
        })?;

    defmt::debug!("MQTT: CONNECT OK");

    // PUBLISH
    let topic_name =
        TopicName::new(MqttString::from_str(topic).map_err(|_| MqttError::BufferTooSmall)?)
            .ok_or(MqttError::BufferTooSmall)?;

    let pub_options = match qos {
        QoS::AtMostOnce => PublicationOptions::new(TopicReference::Name(topic_name.as_borrowed())),
        QoS::AtLeastOnce => {
            PublicationOptions::new(TopicReference::Name(topic_name.as_borrowed())).at_least_once()
        }
        QoS::ExactlyOnce => {
            PublicationOptions::new(TopicReference::Name(topic_name.as_borrowed())).exactly_once()
        }
    };

    client
        .publish(&pub_options, payload.into())
        .await
        .map_err(|_| MqttError::PublishFailed)?;

    // Ждём PUBACK для QoS 1+
    if qos != QoS::AtMostOnce {
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            if Instant::now() >= deadline {
                break;
            }
            match client.poll().await {
                Ok(Event::PublishAcknowledged(_)) => break,
                Ok(_) => continue,
                Err(_) => break,
            }
        }
    }

    // DISCONNECT
    let _ = client
        .disconnect(&rust_mqtt::client::options::DisconnectOptions::new())
        .await;

    defmt::debug!("MQTT: publish done, disconnected");
    Ok(())
}

// ── subscribe_and_listen — подключение с подпиской ───────────────────────

/// Подключиться к брокеру, подписаться на топик, ждать входящие сообщения.
///
/// Только для приёма настроек от сервера.
/// В интеграции вызывается в отдельной embassy task.
pub async fn subscribe_and_listen(
    stack: Stack<'_>,
    dns: &mut DnsResolver<'_>,
    config: &MqttConfig,
    topic: &str,
) -> Result<(), MqttError> {
    // DNS
    let broker_ip = dns
        .resolve_or_fallback(&config.broker_host, config.broker_fallback_ip)
        .await;

    // TCP
    let mut rx_buf = [0u8; TCP_RX_BUF_SIZE];
    let mut tx_buf = [0u8; TCP_TX_BUF_SIZE];
    let mut socket = TcpSocket::new(stack, &mut rx_buf, &mut tx_buf);

    let remote = (
        embassy_net::Ipv4Address::new(broker_ip[0], broker_ip[1], broker_ip[2], broker_ip[3]),
        MQTT_PORT,
    );
    socket
        .connect(remote)
        .await
        .map_err(|_| MqttError::TcpConnectFailed)?;

    // MQTT
    let transport = TcpTransport::new(socket);
    let mut buffer = BumpBuffer::new();
    let mut client = Client::<TcpTransport<'_>, BumpBuffer, 2, 2, 2, 0>::new(&mut buffer);

    let keepalive = KeepAlive::Seconds(
        core::num::NonZero::new(mqtt_topics::MQTT_KEEPALIVE_SECS)
            .unwrap_or_else(|| core::num::NonZero::new(60).unwrap_or_else(|| unreachable!())),
    );

    let connect_options = ConnectOptions::new().clean_start().keep_alive(keepalive);

    let client_id_str = config.client_id();
    let client_id = MqttString::from_str(&client_id_str).map_err(|_| MqttError::BufferTooSmall)?;

    client
        .connect(transport, &connect_options, Some(client_id))
        .await
        .map_err(|_| MqttError::ConnectFailed)?;

    // SUBSCRIBE
    let topic_filter =
        TopicFilter::new(MqttString::from_str(topic).map_err(|_| MqttError::BufferTooSmall)?)
            .ok_or(MqttError::SubscribeFailed)?;

    client
        .subscribe(
            topic_filter.as_borrowed(),
            SubscriptionOptions::new().at_least_once(),
        )
        .await
        .map_err(|_| MqttError::SubscribeFailed)?;

    defmt::debug!("MQTT: SUBSCRIBE OK on {:?}", topic);

    // Event loop — обрабатываем входящие сообщения
    loop {
        match client.poll().await {
            Ok(Event::Publish(publish)) => {
                // Входящее сообщение — настройки от сервера
                defmt::debug!(
                    "MQTT: received on {:?}: {} bytes",
                    publish.topic,
                    publish.message.len()
                );
                // TODO: обработать payload (настройки JSON)
            }
            Ok(Event::Pingresp) => {
                defmt::trace!("MQTT: PINGRESP");
            }
            Ok(Event::Suback(_)) => {
                defmt::trace!("MQTT: SUBACK");
            }
            Ok(_) => {
                defmt::trace!("MQTT: other event");
            }
            Err(e) => {
                defmt::error!("MQTT: poll error: {:?}", e);
                break Err(MqttError::Disconnected);
            }
        }

        // Keepalive: rust-mqtt отправляет PINGREQ автоматически при poll()
        // Но если нет входящих данных — спим 1 секунду
        embassy_time::Timer::after_secs(1).await;
    }
}
