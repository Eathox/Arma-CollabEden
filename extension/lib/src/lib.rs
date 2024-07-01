#![deny(clippy::all, clippy::nursery)]
#![warn(missing_docs, clippy::pedantic, clippy::unwrap_used)]
#![allow(clippy::similar_names, clippy::module_name_repetitions)]

//! Networking library to add multiplayer capabilities to Arma 3s Eden Editor.

#[allow(unused_imports)]
#[macro_use]
extern crate log;

use std::{
    net::SocketAddr,
    time::{Duration, Instant},
};

use serde::{Deserialize, Serialize};

mod builder;
mod client;
mod entity_id;
mod error;
mod networking;
mod output;
mod server;

pub use builder::ManagerBuilder;
pub use client::{Manager as ClientManager, Output as ClientOutput};
pub use entity_id::NetEntityId;
pub use error::{Error, Result};
pub use networking::ConnectionId;
pub use output::OutputReceiver;
use output::OutputSender;
pub use server::{Manager as ServerManager, Output as ServerOutput};

/// Manager responsible for a networking instance, constructed with [`ManagerBuilder`].
/// Can be configured to be either a server, client or a client hosted server.
pub trait InstanceManager {
    /// Local address used by the instance.
    #[must_use]
    fn addr(&self) -> SocketAddr;

    /// Address of the server the instance is connected to, for host instances this is the same as [`Self::addr`].
    #[must_use]
    fn server_addr(&self) -> SocketAddr;

    /// Disconnect the instance.
    fn disconnect(&self);

    /// Stop the instance.
    fn stop(&self);
}

#[derive(Debug, Clone, Deserialize, Serialize)]
enum ArmaEvent {
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
enum CommonMessage {
    ArmaEvent(ArmaEvent),

    Ping(PingTimer),
    Pong(PingTimer),
}

#[derive(Debug, Serialize, Deserialize)]
struct PingTimer(#[serde(with = "instant_serde")] Instant);

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
