use std::time::{Duration, Instant};

use super::{SharedMessage, PING_INTERVAL};
use crate::{
    network::{ConnectionId, NetworkController, NetworkEvent, NetworkHandler},
    OutputSender,
};

/// Server output events.
#[derive(Debug, PartialEq, Eq)]
pub enum ServerOutput {
    /// New client connected.
    ClientConnected(ConnectionId),
    /// Client disconnected.
    ClientDisconnected(ConnectionId),
    /// Lost connection to the client.
    LostConnection(ConnectionId),

    /// Ping.
    Ping(Duration),
}

#[derive(Debug)]
pub enum ServerCommand {
    Disconnect,
    PingLoop,
}

pub struct ServerHandler {
    network: NetworkController<Self>,
    output: OutputSender<ServerOutput>,
    clients: Vec<ConnectionId>,
}

impl ServerHandler {
    pub fn new(
        network: NetworkController<Self>,
        output: OutputSender<ServerOutput>,
        ping_loop: bool,
    ) -> Self {
        if ping_loop {
            network.delayed_command(ServerCommand::PingLoop, PING_INTERVAL);
        };

        Self {
            network,
            output,
            clients: vec![],
        }
    }

    fn output(&mut self, output: ServerOutput) {
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
                .send(*client, SharedMessage::Ping(Instant::now()));
        }

        if repeat {
            self.network
                .delayed_command(ServerCommand::PingLoop, PING_INTERVAL);
        }
    }
}

impl NetworkHandler for ServerHandler {
    type Message = SharedMessage;
    type Command = ServerCommand;

    fn handle_event(&mut self, event: NetworkEvent) {
        match event {
            NetworkEvent::NewConnection(conn) => {
                self.output(ServerOutput::ClientConnected(conn));
                self.clients.push(conn);
            }
            NetworkEvent::ConnectionLost(conn, disconnected) => {
                self.output(if disconnected {
                    ServerOutput::ClientDisconnected(conn)
                } else {
                    ServerOutput::LostConnection(conn)
                });
                self.clients.retain(|c| c != &conn);
            }
            NetworkEvent::ConnectionAttempt(_, _) => {
                unreachable!("Servers cant attempt to connect")
            }
        }
    }

    fn handle_message(&mut self, conn: ConnectionId, message: &Self::Message) {
        match message {
            SharedMessage::Ping(instant) => {
                self.network.send(conn, SharedMessage::Pong(*instant));
            }
            SharedMessage::Pong(instant) => {
                self.output(ServerOutput::Ping(instant.elapsed()));
            }
            SharedMessage::ArmaEvent(_) => todo!(),
        }
    }

    fn handle_command(&mut self, command: &Self::Command) {
        match command {
            ServerCommand::Disconnect => self.disconnect(),
            ServerCommand::PingLoop => self.ping(true),
        }
    }
}
