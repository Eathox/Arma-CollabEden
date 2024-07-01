use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

pub mod client;
mod id;
pub mod server;

pub use id::NetEntityId;

const PING_INTERVAL: Duration = Duration::from_millis(500);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ArmaEvent {
    Event {
        name: String,
        params: arma_rs::Value,
    },
    EntityEvent {
        id: NetEntityId,
        name: String,
        params: arma_rs::Value,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CommonMessage {
    ArmaEvent(ArmaEvent),

    Ping(PingTimer),
    Pong(PingTimer),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PingTimer(#[serde(with = "instant_serde")] Instant);

impl PingTimer {
    pub fn new() -> Self {
        Self(Instant::now())
    }

    pub fn elapsed(&self) -> Duration {
        self.0.elapsed()
    }
}

/// Serde impls for [`std::time::Instant`] to be used with `#[serde(with = "instant_serde")]`.
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
