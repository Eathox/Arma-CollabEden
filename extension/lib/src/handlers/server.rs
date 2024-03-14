use std::time::Instant;

use super::SharedMessage;
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
}

#[derive(Debug)]
pub enum ServerCommand {
    Disconnect,
    Ping,
}

pub struct ServerHandler {
    network: NetworkController<Self>,
    output: OutputSender<ServerOutput>,
    clients: Vec<ConnectionId>,
}

impl ServerHandler {
    pub const fn new(network: NetworkController<Self>, output: OutputSender<ServerOutput>) -> Self {
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

    fn ping(&self) {
        for client in &self.clients {
            self.network
                .send(*client, SharedMessage::Ping(Instant::now()));
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
        if let SharedMessage::Ping(elapsed) = message {
            self.network.send(conn, SharedMessage::Pong(*elapsed));
        }

        if let SharedMessage::Pong(elapsed) = message {
            println!("[Server] Pong from {conn:?} in {:?}", elapsed.elapsed());
        } else {
            println!("[Server] Got message: {message:?} from {conn:?}");
        }
    }

    fn handle_command(&mut self, command: &Self::Command) {
        match command {
            ServerCommand::Disconnect => self.disconnect(),
            ServerCommand::Ping => self.ping(),
        }
    }
}
