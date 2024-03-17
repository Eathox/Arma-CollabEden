use std::{collections::HashSet, net::SocketAddr, time::Duration};

use message_io::{
    network::{Endpoint, NetEvent, Transport},
    node::{self, NodeEvent, NodeHandler, NodeListener},
};
use serde::{de::DeserializeOwned, Serialize};

use crate::{Error, Result};

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
        write!(f, "#{}", self.id())
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

    /// Handle a network event.
    fn handle_event(&mut self, event: NetworkEvent);

    /// Handle a network message.
    fn handle_message(&mut self, conn: ConnectionId, message: Self::RecvMessage);

    /// Handle a command.
    fn handle_command(&mut self, command: Self::Command);
}

/// Type that can be sent over network interface.
pub trait NetworkSerde: Serialize + DeserializeOwned {
    /// Serialize into a byte vector for sending over a network interface.
    ///
    /// # Errors
    /// An error is returned if the instance failed to serialize.
    fn to_net(&self) -> Result<Vec<u8>, ciborium::ser::Error<std::io::Error>> {
        let mut buffer = Vec::new();
        ciborium::into_writer(self, &mut buffer)?;
        Ok(buffer)
    }

    /// Deserialize from a byte slice send over a network interface.
    ///
    /// # Errors
    /// An error is returned if the instance failed to deserialize.
    fn from_net(bytes: &[u8]) -> Result<Self, ciborium::de::Error<std::io::Error>> {
        ciborium::from_reader(bytes)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
enum InternalMessage<M> {
    Disconnected,
    Handler(M),
}

impl NetworkSerde for () {}
impl<M: NetworkSerde> NetworkSerde for InternalMessage<M> {}

/// Create a new network interface returning its [`NetworkController`] and [`NetworkListener`].
pub fn new_network_interface<H: NetworkHandler>() -> (NetworkController<H>, NetworkListener<H>) {
    let (node, listener) = node::split::<H::Command>();
    (NetworkController(node), NetworkListener(listener))
}

/// Controller used to connect, remove and send messages over the network, can safely be shared between threads.
pub struct NetworkController<H: NetworkHandler>(NodeHandler<H::Command>);

impl<H: NetworkHandler> Clone for NetworkController<H> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<H: NetworkHandler> NetworkController<H> {
    /// Listen on the given address. Returns the actual address listening on, since port can be passed `0` for automatic selection.
    ///
    /// # Errors
    /// Returns an error if unable to listen on the given address.
    pub fn listen(&self, addr: SocketAddr) -> Result<SocketAddr> {
        let (_, new_addr) = self
            .0
            .network()
            .listen(Transport::FramedTcp, addr)
            .map_err(|err| Error::Listen(addr, err))?;
        let new_addr = SocketAddr::new(addr.ip(), new_addr.port()); // Fix bug where sometimes 0.0.0.0 is returned with automatic port selection
        Ok(new_addr)
    }

    /// Connect to the given address. Returns id used to identify the connection and own address its connected from.
    ///
    /// # Errors
    /// Returns an error if the address couldn't be used for a connection attempt.\
    /// *Note: this will never error if the connection failed, instead that is reflected in [`NetworkEvent::ConnectionAttempt`].*
    pub fn connect(&self, addr: SocketAddr) -> Result<(ConnectionId, SocketAddr)> {
        let (conn, new_addr) = self
            .0
            .network()
            .connect(Transport::FramedTcp, addr)
            .map_err(|err| Error::ConnectAttempt(addr, err))?;
        let new_addr = SocketAddr::new(addr.ip(), new_addr.port()); // Fix bug where sometimes 0.0.0.0 is returned with automatic port selection
        Ok((ConnectionId(conn), new_addr))
    }

    /// Remove the given connection. This does not emit a [`NetworkEvent::ConnectionLost`] to the event loop.
    ///
    /// Returns `false` if the connection is already removed.
    pub fn remove(&self, conn: ConnectionId) -> bool {
        let id = conn.0.resource_id();
        if self.0.network().is_ready(id) == Some(true) {
            self.send_internal(conn, &InternalMessage::Disconnected);
            self.0.network().remove(id)
        } else {
            false
        }
    }

    /// Send a message to the given connection.
    pub fn send(&self, conn: ConnectionId, message: H::SendMessage) {
        self.send_internal(conn, &InternalMessage::Handler(message));
    }

    fn send_internal(&self, conn: ConnectionId, message: &InternalMessage<H::SendMessage>) {
        match message.to_net() {
            Ok(bytes) => {
                self.0.network().send(conn.0, &bytes);
            }
            Err(err) => error!("failed to send message: {err}"),
        }
    }

    /// Send a command to the [`NetworkHandler`].
    pub fn command(&self, command: H::Command) {
        self.0.signals().send(command);
    }

    /// Send a delayed command to the [`NetworkHandler`].
    pub fn delayed_command(&self, command: H::Command, wait: Duration) {
        self.0.signals().send_with_timer(command, wait);
    }

    /// Shut down the controller's corresponding [`NetworkListener`].
    /// Has no effect if the listener isn't running.
    pub fn stop(&self) {
        self.0.stop();
    }
}

/// Listener that queues and serves [`NetworkEvent`], [`NetworkHandler::Command`] and [`NetworkHandler::Message`] to a [`NetworkHandler`].
pub struct NetworkListener<H: NetworkHandler>(NodeListener<H::Command>);

/// Listener loop lifetime.
pub type ListenerLifetime = message_io::node::NodeTask;

impl<H: NetworkHandler> NetworkListener<H> {
    /// Start the [`NetworkListener`]s loop with to the given [`NetworkHandler`].
    /// Any events generated before the listener started are queued and will be processed first once started.
    pub fn start(self, mut handler: H) -> ListenerLifetime {
        let mut disconnects = HashSet::new();

        self.0.for_each_async(move |event| {
            let mut map_message = |conn, bytes| match InternalMessage::from_net(bytes) {
                Err(err) => error!("received invalid message from({conn}): {err}"),
                Ok(message) => match message {
                    InternalMessage::Disconnected => {
                        disconnects.insert(conn);
                    }
                    InternalMessage::Handler(message) => {
                        handler.handle_message(conn, message);
                    }
                },
            };

            match event {
                NodeEvent::Network(net_event) => {
                    let event = match net_event {
                        NetEvent::Message(conn, bytes) => {
                            map_message(ConnectionId(conn), bytes);
                            return;
                        }

                        NetEvent::Accepted(conn, _) => {
                            NetworkEvent::NewConnection(ConnectionId(conn))
                        }
                        NetEvent::Connected(conn, ok) => {
                            NetworkEvent::ConnectionAttempt(ConnectionId(conn), ok)
                        }
                        NetEvent::Disconnected(conn) => {
                            let conn = ConnectionId(conn);
                            let disconnected = disconnects.remove(&conn);
                            NetworkEvent::ConnectionLost(conn, disconnected)
                        }
                    };
                    handler.handle_event(event);
                }
                NodeEvent::Signal(command) => {
                    handler.handle_command(command);
                }
            }
        })
    }
}
