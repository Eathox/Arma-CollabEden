use std::time::Duration;

use super::{client, PingPayload, PING_INTERVAL};
use crate::{
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
    Ping(PingPayload),
    Pong(PingPayload),
}

impl NetworkSerde for Message {}

#[derive(Debug)]
pub enum Command {
    Disconnect,
    PingLoop,
}

pub struct Handler {
    network: NetworkController<Self>,
    output: OutputSender<Output>,
    clients: Vec<ConnectionId>,
}

impl Handler {
    pub fn new(
        network: NetworkController<Self>,
        output: OutputSender<Output>,
        ping_loop: bool,
    ) -> Self {
        if ping_loop {
            network.delayed_command(Command::PingLoop, PING_INTERVAL);
        };

        Self {
            network,
            output,
            clients: vec![],
        }
    }

    fn output(&mut self, output: Output) {
        self.output.send(output);
    }

    fn disconnect(&self) {
        for client in &self.clients {
            self.network.remove(*client);
        }
        self.network.stop();
    }

    fn ping(&self, repeat: bool) {
        for client in &self.clients {
            self.network
                .send(*client, Message::Ping(PingPayload::new()));
        }

        if repeat {
            self.network
                .delayed_command(Command::PingLoop, PING_INTERVAL);
        }
    }
}

impl NetworkHandler for Handler {
    type RecvMessage = client::Message;
    type SendMessage = Message;
    type Command = Command;

    fn handle_event(&mut self, event: NetworkEvent) {
        match event {
            NetworkEvent::NewConnection(conn) => {
                self.output(Output::ClientConnected(conn));
                self.clients.push(conn);
            }
            NetworkEvent::ConnectionLost(conn, disconnected) => {
                self.output(if disconnected {
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
            Self::RecvMessage::Ping(payload) => {
                self.network.send(conn, Message::Pong(payload));
            }
            Self::RecvMessage::Pong(payload) => {
                self.output(Output::Ping(conn, payload.elapsed()));
            }
        }
    }

    fn handle_command(&mut self, command: Self::Command) {
        match command {
            Command::Disconnect => self.disconnect(),
            Command::PingLoop => self.ping(true),
        }
    }
}
