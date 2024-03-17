use std::time::{Duration, Instant};

pub mod client;
pub mod server;

const PING_INTERVAL: Duration = Duration::from_millis(500);

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct PingPayload(#[serde(with = "instant_serde")] Instant);

impl PingPayload {
    pub fn new() -> Self {
        Self(Instant::now())
    }

    pub fn elapsed(&self) -> Duration {
        self.0.elapsed()
    }
}

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
