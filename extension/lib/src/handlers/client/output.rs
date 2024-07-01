use std::time::Duration;

use super::ArmaEvent;
use crate::NetEntityId;

/// Client output events.
#[derive(Debug, PartialEq)]
pub enum Output {
    /// Unique entity net id reserved by this client.
    ReservedNetId(NetEntityId),

    /// Arma event.
    ArmaEvent {
        /// Event name.
        name: String,
        /// Event params.
        params: arma_rs::Value,
    },
    /// Arma entity event.
    ArmaEntityEvent {
        /// Entity net id.
        id: NetEntityId,
        /// Event name.
        name: String,
        /// Event params.
        params: arma_rs::Value,
    },

    /// Attempted to connect to the server, `true` if connected.
    ServerConnected(bool),
    /// Server disconnected.
    ServerDisconnected,
    /// Lost connection to the server.
    LostConnection,
    /// Ping to the server.
    Ping(Duration),
}

impl From<ArmaEvent> for Output {
    fn from(event: ArmaEvent) -> Self {
        match event {
            ArmaEvent::Event { name, params } => Self::ArmaEvent { name, params },
            ArmaEvent::EntityEvent { id, name, params } => {
                Self::ArmaEntityEvent { id, name, params }
            }
        }
    }
}
