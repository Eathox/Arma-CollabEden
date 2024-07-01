use std::collections::HashSet;

use message_io::{
    network::NetEvent,
    node::{NodeEvent, NodeListener},
};

use super::{ConnectionId, InternalMessage, NetworkEvent, NetworkHandler, NetworkSerde};

/// Listener that queues and serves [`NetworkEvent`], [`NetworkHandler::Command`] and [`NetworkHandler::RecvMessage`] to a [`NetworkHandler`].
pub struct NetworkListener<H: NetworkHandler>(NodeListener<H::Command>);
pub type ListenerLifetime = message_io::node::NodeTask;

impl<H: NetworkHandler> NetworkListener<H> {
    pub fn new(listener: NodeListener<H::Command>) -> Self {
        Self(listener)
    }

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
