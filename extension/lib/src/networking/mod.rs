use std::net::SocketAddr;

use message_io::network::Endpoint;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::Result;

mod controller;
mod listener;

pub use controller::NetworkController;
pub use listener::ListenerLifetime;
use listener::NetworkListener;

/// Id of a connection on the network.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConnectionId(Endpoint);

impl ConnectionId {
    /// Unique ID of the connection.
    #[inline]
    #[must_use]
    pub fn id(&self) -> usize {
        self.0.resource_id().base_value()
    }

    /// Address of the connection.
    #[inline]
    #[must_use]
    pub fn addr(&self) -> SocketAddr {
        self.0.addr()
    }
}

impl std::fmt::Debug for ConnectionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConnectionId")
            .field("id", &self.id())
            .field("addr", &self.addr())
            .finish()
    }
}

impl std::fmt::Display for ConnectionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Conn:{}", self.id())
    }
}

/// Events that can occur on the network interface.
#[derive(Debug, Clone)]
pub enum NetworkEvent {
    /// Accepted new connection. Only emitted for Servers.
    NewConnection(ConnectionId),
    /// Attempted to establish a connection, bool is true if succeeded. Only emitted for Clients.
    ConnectionAttempt(ConnectionId, bool),
    /// Lost connection. bool is true if the connection was explicitly disconnected.
    ConnectionLost(ConnectionId, bool),
}

/// Handler used for implementing actual program logic on top of a [`NetworkListener`].
pub trait NetworkHandler: Sized + Send + 'static {
    /// Message received by this handler.
    type RecvMessage: NetworkSerde + Send + 'static;

    /// Message send by this handler.
    type SendMessage: NetworkSerde + Send + 'static;

    /// Command thats send to the handler from outside the listener loop using [`NetworkController::command`].
    type Command: Send + 'static;

    fn handle_event(&mut self, event: NetworkEvent);
    fn handle_message(&mut self, conn: ConnectionId, message: Self::RecvMessage);
    fn handle_command(&mut self, command: Self::Command);
}

/// Type that can be sent over network interface.
pub trait NetworkSerde: Serialize + DeserializeOwned {
    fn to_net(&self) -> Result<Vec<u8>, ciborium::ser::Error<std::io::Error>> {
        let mut buffer = Vec::new();
        ciborium::into_writer(self, &mut buffer)?;
        Ok(buffer)
    }

    fn from_net(bytes: &[u8]) -> Result<Self, ciborium::de::Error<std::io::Error>> {
        ciborium::from_reader(bytes)
    }
}

#[derive(Serialize, Deserialize)]
enum InternalMessage<M> {
    Disconnected,
    Handler(M),
}

impl NetworkSerde for () {}
impl<M: NetworkSerde> NetworkSerde for InternalMessage<M> {}
