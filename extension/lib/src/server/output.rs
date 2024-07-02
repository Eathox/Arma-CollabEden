use std::time::Duration;

use crate::ConnectionId;

/// Server output events.
#[derive(Debug, PartialEq, Eq)]
pub enum Output {
    /// Ping result from the client.
    Ping(ConnectionId, Duration),
    /// New client connected.
    ClientConnected(ConnectionId),
    /// Client disconnected.
    ClientDisconnected(ConnectionId),

    /// Lost connection to the client.
    LostConnection(ConnectionId),
    /// Server shutdown.
    Shutdown,
}
