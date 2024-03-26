use std::time::Duration;

use super::{server::Message as ServerMessage, ArmaEvent, PingTimer, SharedMessage, PING_INTERVAL};
use crate::{
    id::NetEntityId,
    network::{ConnectionId, NetworkController, NetworkEvent, NetworkHandler, NetworkSerde},
    OutputSender,
};

/// Client output events.
#[derive(Debug, PartialEq)]
pub enum Output {
    /// Unique entity net id reserved by this client.
    EntityNetId(NetEntityId),

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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Message {
    RequestEntityNetId,

    Shared(SharedMessage),
}

impl NetworkSerde for Message {}

impl From<SharedMessage> for Message {
    fn from(message: SharedMessage) -> Self {
        Self::Shared(message)
    }
}

#[derive(Debug)]
pub enum Command {
    RequestEntityNetId,
    ArmaEvent(ArmaEvent),
    Disconnect,
    PingLoop,
}

pub struct Handler {
    network: NetworkController<Self>,
    output: OutputSender<Output>,
    server: ConnectionId,
}

impl NetworkHandler for Handler {
    type RecvMessage = ServerMessage;
    type SendMessage = Message;
    type Command = Command;

    fn handle_event(&mut self, event: NetworkEvent) {
        match event {
            NetworkEvent::ConnectionAttempt(_, succeeded) => {
                self.output.send(Output::ServerConnected(succeeded));
                if !succeeded {
                    self.network.stop();
                }
            }
            NetworkEvent::ConnectionLost(_, disconnected) => {
                self.output.send(if disconnected {
                    Output::ServerDisconnected
                } else {
                    Output::LostConnection
                });
                self.network.stop();
            }
            NetworkEvent::NewConnection(_) => unreachable!("Clients cant accept new connections"),
        }
    }

    fn handle_message(&mut self, conn: ConnectionId, message: Self::RecvMessage) {
        match message {
            ServerMessage::EntityNetId(id) => {
                self.output.send(Output::EntityNetId(id));
            }

            ServerMessage::Shared(message) => match message {
                SharedMessage::ArmaEvent(event) => {
                    self.output.send(event.into());
                }

                SharedMessage::Ping(timer) => {
                    self.network.send(conn, SharedMessage::Pong(timer).into());
                }
                SharedMessage::Pong(timer) => {
                    self.output.send(Output::Ping(timer.elapsed()));
                }
            },
        }
    }

    fn handle_command(&mut self, command: Self::Command) {
        match command {
            Command::RequestEntityNetId => {
                self.network.send(self.server, Message::RequestEntityNetId);
            }
            Command::ArmaEvent(event) => {
                let message = SharedMessage::ArmaEvent(event);
                self.network.send(self.server, message.into());
            }

            Command::Disconnect => self.disconnect(),
            Command::PingLoop => self.ping(true),
        }
    }
}

impl Handler {
    pub fn new(
        network: NetworkController<Self>,
        output: OutputSender<Output>,
        server: ConnectionId,
        ping_loop: bool,
    ) -> Self {
        if ping_loop {
            network.command(Command::PingLoop, Some(PING_INTERVAL));
        };

        Self {
            network,
            output,
            server,
        }
    }

    fn disconnect(&self) {
        self.network.remove(self.server);
        self.network.stop();
    }

    fn ping(&self, repeat: bool) {
        let message = SharedMessage::Ping(PingTimer::new());
        self.network.send(self.server, message.into());

        if repeat {
            self.network.command(Command::PingLoop, Some(PING_INTERVAL));
        }
    }
}
