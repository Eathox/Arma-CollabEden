use std::time::Duration;

use crate::ConnectionId;

/// Server output events.
#[derive(Debug, PartialEq, Eq)]
pub enum Output {
    /// New client connected.
    ClientConnected(ConnectionId),
    /// Client disconnected.
    ClientDisconnected(ConnectionId),
    /// Lost connection to the client.
    LostConnection(ConnectionId),
    /// Ping to the client.
    Ping(ConnectionId, Duration),
}
