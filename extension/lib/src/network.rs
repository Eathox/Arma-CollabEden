use std::{net::SocketAddr, sync::Arc};

use message_io::network::{NetEvent, Transport};
use message_io::node::{self, NodeEvent, NodeHandler, NodeTask};

use crate::{Message, MessageSerde};

/// Endpoint of a connection used to identify connections.
pub type Endpoint = message_io::network::Endpoint;

/// Events that can occur on the network.
#[derive(Debug)]
pub enum Event {
    /// Accepted new connection from endpoint. Only emitted for Servers.
    NewConnection(Endpoint),
    /// Attempted to connect to endpoint, boolean is true if succeeded. Only emitted for Clients.
    ConnectionAttempt(Endpoint, bool),
    /// Lost connection to endpoint, not emitted on explicit disconnect.
    ConnectionLost(Endpoint),
    /// Message from endpoint.
    Message(Endpoint, Message),
}

/// Event handler used to implement actual networking logic for client and server.
pub trait Handler<Command>: Send + 'static
where
    Command: Send + 'static,
{
    /// Handle an network event from the event loop.
    fn handle_net(&mut self, io: &NetworkIO<Command>, event: Event);

    /// Handle a command sent via the [`NetworkIO::command`]
    fn handle_command(&mut self, io: &NetworkIO<Command>, command: Command);
}

/// Core of the networking IO, handles event loop and sending and receiving messages.
pub struct NetworkIO<Command>
where
    Command: Send + 'static,
{
    node: NodeHandler<Command>,
    node_task: Option<Arc<NodeTask>>,
}

impl<Command> Clone for NetworkIO<Command>
where
    Command: Send + 'static,
{
    fn clone(&self) -> Self {
        Self {
            node: self.node.clone(),
            node_task: self.node_task.clone(),
        }
    }
}

impl<Command> NetworkIO<Command>
where
    Command: Send + 'static,
{
    /// Create a new network IO, with the given handler. Starts the event loop.
    pub fn startup(mut handler: impl Handler<Command>) -> Self {
        let (node, listener) = node::split::<Command>();
        let mut network = Self {
            node,
            node_task: None,
        };

        let io = network.clone();
        let task = listener.for_each_async(move |event| match event {
            NodeEvent::Network(net_event) => {
                let event = match net_event {
                    NetEvent::Accepted(endpoint, _) => Ok(Event::NewConnection(endpoint)),
                    NetEvent::Connected(endpoint, succeeded) => {
                        Ok(Event::ConnectionAttempt(endpoint, succeeded))
                    }
                    NetEvent::Disconnected(endpoint) => Ok(Event::ConnectionLost(endpoint)),
                    NetEvent::Message(endpoint, bytes) => match Message::from_bytes(bytes) {
                        Ok(message) => Ok(Event::Message(endpoint, message)),
                        Err(err) => {
                            Err(format!("received invalid message from({endpoint}): {err}"))
                        }
                    },
                };

                match event {
                    Ok(event) => handler.handle_net(&io, event),
                    Err(err) => error!("{err}"),
                }
            }
            NodeEvent::Signal(command) => {
                handler.handle_command(&io, command);
            }
        });

        network.node_task = Some(Arc::new(task));
        network
    }

    /// Stop the event loop.
    pub fn stop(&self) {
        self.node.stop();
    }

    /// Listen on the given address.
    pub fn listen(&self, addr: SocketAddr) -> std::io::Result<SocketAddr> {
        let (_, addr) = self.node.network().listen(Transport::FramedTcp, addr)?;
        Ok(addr)
    }

    /// Connect to the given address.
    pub fn connect(&self, addr: SocketAddr) -> std::io::Result<SocketAddr> {
        let (_, addr) = self.node.network().connect(Transport::FramedTcp, addr)?;
        Ok(addr)
    }

    /// Remove the given endpoint. This does not emit a [`Event::ConnectionLost`] to the event loop.
    ///
    /// Returns `false` if the endpoint is not connected.
    pub fn remove(&self, endpoint: Endpoint) -> bool {
        self.node.network().remove(endpoint.resource_id())
    }

    /// Send a message to the given endpoint.
    pub fn send(&self, endpoint: Endpoint, message: &Message) {
        match message.to_bytes() {
            Ok(bytes) => {
                self.node.network().send(endpoint, &bytes);
            }
            Err(err) => error!("failed to send message: {err}"),
        }
    }

    /// Send a command to the [`Handler`]. These commands are a way to instruct the handler to perform an handler defined action.
    pub fn command(&self, command: Command) {
        self.node.signals().send(command);
    }
}

impl<Command> Drop for NetworkIO<Command>
where
    Command: Send + 'static,
{
    fn drop(&mut self) {
        self.stop();
    }
}
