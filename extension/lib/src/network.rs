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
}

/// Event handler used to implement actual networking logic for client and server.
pub trait EventHandler: Sized + Send + 'static {
    /// Custom command that can be sent from outside the event loop.
    type Command: Send + 'static;

    /// Handle network events send from the event loop.
    fn handle_net(&mut self, io: &NetworkIO<Self>, event: Event);

    /// Handle messages received from remote endpoints.
    fn handle_message(&mut self, io: &NetworkIO<Self>, endpoint: Endpoint, message: &Message);

    /// Handle commands sent via [`NetworkIO::command`].
    fn handle_command(&mut self, io: &NetworkIO<Self>, command: Self::Command);
}

/// Core of the networking IO, handles event loop and sending and receiving messages.
pub struct NetworkIO<Handler: EventHandler> {
    node: NodeHandler<Handler::Command>,
    node_task: Option<Arc<NodeTask>>,
}

impl<Handler: EventHandler> Clone for NetworkIO<Handler> {
    fn clone(&self) -> Self {
        Self {
            node: self.node.clone(),
            node_task: self.node_task.clone(),
        }
    }
}

impl<Handler: EventHandler> NetworkIO<Handler> {
    /// Create a new network IO, with the given handler, starting the event loop.
    pub fn startup(mut handler: Handler) -> Self {
        let (node, listener) = node::split::<Handler::Command>();
        let mut network = Self {
            node,
            node_task: None,
        };

        let io = network.clone();
        let task = listener.for_each_async(move |event| match event {
            NodeEvent::Network(net_event) => {
                if let NetEvent::Message(endpoint, bytes) = net_event {
                    match Message::from_bytes(bytes) {
                        Ok(message) => handler.handle_message(&io, endpoint, &message),
                        Err(err) => error!("received invalid message from({endpoint}): {err}"),
                    }
                    return;
                }

                let event = match net_event {
                    NetEvent::Accepted(endpoint, _) => Event::NewConnection(endpoint),
                    NetEvent::Connected(endpoint, ok) => Event::ConnectionAttempt(endpoint, ok),
                    NetEvent::Disconnected(endpoint) => Event::ConnectionLost(endpoint),
                    NetEvent::Message(_, _) => unreachable!("Message should be handled above"),
                };
                handler.handle_net(&io, event);
            }
            NodeEvent::Signal(command) => {
                handler.handle_command(&io, command);
            }
        });

        network.node_task = Some(Arc::new(task));
        network
    }

    /// Shut down the event loop.
    pub fn shutdown(&self) {
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

    /// Send a command to the [`EventHandler`].
    pub fn command(&self, command: Handler::Command) {
        self.node.signals().send(command);
    }
}

impl<Handler: EventHandler> Drop for NetworkIO<Handler> {
    fn drop(&mut self) {
        self.shutdown();
    }
}
