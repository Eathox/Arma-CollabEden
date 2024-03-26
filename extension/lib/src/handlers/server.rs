use std::time::Duration;

use super::{client::Message as ClientMessage, PingTimer, SharedMessage, PING_INTERVAL};
use crate::{
    id::{NetEntityId, NetIdGenerator},
    network::{ConnectionId, NetworkController, NetworkEvent, NetworkHandler, NetworkSerde},
    OutputSender,
};

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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Message {
    EntityNetId(NetEntityId),

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
    Disconnect,
    PingLoop,
}

pub struct Handler {
    network: NetworkController<Self>,
    output: OutputSender<Output>,
    clients: Vec<ConnectionId>,
    net_id: NetIdGenerator,
}

impl NetworkHandler for Handler {
    type RecvMessage = ClientMessage;
    type SendMessage = Message;
    type Command = Command;

    fn handle_event(&mut self, event: NetworkEvent) {
        match event {
            NetworkEvent::NewConnection(conn) => {
                self.output.send(Output::ClientConnected(conn));
                self.clients.push(conn);
            }
            NetworkEvent::ConnectionLost(conn, disconnected) => {
                self.output.send(if disconnected {
                    Output::ClientDisconnected(conn)
                } else {
                    Output::LostConnection(conn)
                });
                self.clients.retain(|c| c != &conn);
            }
            NetworkEvent::ConnectionAttempt(_, _) => {
                unreachable!("Servers cant attempt to connect")
            }
        }
    }

    fn handle_message(&mut self, conn: ConnectionId, message: Self::RecvMessage) {
        match message {
            ClientMessage::RequestEntityNetId => {
                let net_id = self.net_id.next();
                self.network.send(conn, Message::EntityNetId(net_id));
            }

            ClientMessage::Shared(message) => match message {
                SharedMessage::ArmaEvent(event) => {
                    self.propagate(conn, || SharedMessage::ArmaEvent(event.clone()).into());
                }

                SharedMessage::Ping(timer) => {
                    self.network.send(conn, SharedMessage::Pong(timer).into());
                }
                SharedMessage::Pong(timer) => {
                    self.output.send(Output::Ping(conn, timer.elapsed()));
                }
            },
        }
    }

    fn handle_command(&mut self, command: Self::Command) {
        match command {
            Command::Disconnect => self.disconnect(),
            Command::PingLoop => self.ping(true),
        }
    }
}

impl Handler {
    pub fn new(
        network: NetworkController<Self>,
        output: OutputSender<Output>,
        ping_loop: bool,
    ) -> Self {
        if ping_loop {
            network.command(Command::PingLoop, Some(PING_INTERVAL));
        };

        Self {
            network,
            output,
            clients: vec![],
            net_id: NetIdGenerator::new(),
        }
    }

    fn propagate(&self, origin: ConnectionId, f: impl Fn() -> Message) {
        for client in &self.clients {
            if client != &origin {
                self.network.send(*client, f());
            }
        }
    }

    fn disconnect(&self) {
        for client in &self.clients {
            self.network.remove(*client);
        }
        self.network.stop();
    }

    fn ping(&self, repeat: bool) {
        for client in &self.clients {
            let message = SharedMessage::Ping(PingTimer::new());
            self.network.send(*client, message.into());
        }

        if repeat {
            self.network.command(Command::PingLoop, Some(PING_INTERVAL));
        }
    }
}
