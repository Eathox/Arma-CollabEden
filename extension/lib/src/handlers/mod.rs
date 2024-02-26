use std::time::Instant;

pub mod client;
pub mod client_server;
pub mod server;

use crate::network::NetworkSerde;
pub use client::{ClientCommand, ClientHandler};
pub use client_server::ClientServerHandler;
pub use server::{ServerCommand, ServerHandler};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Message {
    #[serde(with = "instant_serde")]
    Ping(Instant),
    #[serde(with = "instant_serde")]
    Pong(Instant),

    ArmaEvent(String, arma_rs::Value),
}

impl NetworkSerde for Message {}

/// Serde impls for [`std::time::Instant`] to be used with `#[serde(with = "instant_serde")]`. Implemented by converting to and from [`std::time::Duration`].
mod instant_serde {
    use std::time::{Duration, Instant};

    use serde::{de::Error, Deserialize, Serialize};

    pub fn serialize<S: serde::Serializer>(
        instant: &Instant,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let duration = instant.elapsed();
        duration.serialize(serializer)
    }

    pub fn deserialize<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Instant, D::Error> {
        let duration = Duration::deserialize(deserializer)?;
        Instant::now()
            .checked_sub(duration)
            .ok_or_else(|| Error::custom("instant is out of bounds"))
    }
}
