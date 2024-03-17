use std::time::Duration;

use super::{server, PingPayload, PING_INTERVAL};
use crate::{
    network::{ConnectionId, NetworkController, NetworkEvent, NetworkHandler, NetworkSerde},
    OutputSender,
};

/// Client output events.
#[derive(Debug, PartialEq, Eq)]
pub enum Output {
    /// Attempted to connect to the server, `true` if connected.
    Connected(bool),
    /// Server disconnected.
    ServerDisconnected,
    /// Lost connection to the server.
    LostConnection,

    /// Ping to the server.
    Ping(Duration),
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
    server: ConnectionId,
}

impl Handler {
    pub fn new(
        network: NetworkController<Self>,
        output: OutputSender<Output>,
        server: ConnectionId,
        ping_loop: bool,
    ) -> Self {
        if ping_loop {
            network.delayed_command(Command::PingLoop, PING_INTERVAL);
        };

        Self {
            network,
            output,
            server,
        }
    }

    fn output(&mut self, output: Output) {
        self.output.send(output);
    }

    fn disconnect(&self) {
        self.network.remove(self.server);
        self.network.stop();
    }

    fn ping(&self, repeat: bool) {
        self.network
            .send(self.server, Message::Ping(PingPayload::new()));

        if repeat {
            self.network
                .delayed_command(Command::PingLoop, PING_INTERVAL);
        }
    }
}

impl NetworkHandler for Handler {
    type SendMessage = Message;
    type RecvMessage = server::Message;
    type Command = Command;

    fn handle_event(&mut self, event: NetworkEvent) {
        match event {
            NetworkEvent::ConnectionAttempt(_, succeeded) => {
                self.output(Output::Connected(succeeded));
                if !succeeded {
                    self.network.stop();
                }
            }
            NetworkEvent::ConnectionLost(_, disconnected) => {
                self.output(if disconnected {
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
            Self::RecvMessage::Ping(payload) => {
                self.network.send(conn, Message::Pong(payload));
            }
            Self::RecvMessage::Pong(payload) => {
                self.output(Output::Ping(payload.elapsed()));
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
