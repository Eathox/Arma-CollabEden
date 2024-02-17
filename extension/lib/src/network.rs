use std::{net::SocketAddr, sync::Arc};

use message_io::network::{NetEvent, Transport};
use message_io::node::{self, NodeEvent, NodeHandler, NodeTask};

use crate::{Message, MessageSerde, Result};

pub type Endpoint = message_io::network::Endpoint;

/// Events that can occur on the network.
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

pub trait Handler<Command>: Send + 'static
where
    Command: Clone + Send + 'static,
{
    fn handle_net(&mut self, io: &NetworkIO<Command>, event: Event);
    fn command_handler(&mut self, io: &NetworkIO<Command>, command: Command);
}

#[derive(Clone)]
pub struct NetworkIO<Command>
where
    Command: Clone + Send + 'static,
{
    node: NodeHandler<Command>,
    node_task: Option<Arc<NodeTask>>,
}

impl<Command> NetworkIO<Command>
where
    Command: Clone + Send + 'static,
{
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
                handler.command_handler(&io, command);
            }
        });

        network.node_task = Some(Arc::new(task));
        network
    }

    pub fn stop(&self) {
        self.node.stop();
    }

    pub fn listen(&self, addr: SocketAddr) -> Result<SocketAddr> {
        let (_, addr) = self.node.network().listen(Transport::FramedTcp, addr)?;
        Ok(addr)
    }

    pub fn connect(&self, addr: SocketAddr) -> Result<SocketAddr> {
        let (_, addr) = self.node.network().connect(Transport::FramedTcp, addr)?;
        Ok(addr)
    }

    pub fn remove(&self, endpoint: Endpoint) -> bool {
        self.node.network().remove(endpoint.resource_id())
    }

    pub fn send(&self, endpoint: Endpoint, message: &Message) {
        match message.to_bytes() {
            Ok(bytes) => {
                self.node.network().send(endpoint, &bytes);
            }
            Err(err) => error!("failed to send message: {err}"),
        }
    }

    pub fn command(&self, command: Command) {
        self.node.signals().send(command);
    }
}
