use std::time::Instant;

use super::SharedMessage;
use crate::{
    network::{ConnectionId, NetworkController, NetworkEvent, NetworkHandler},
    OutputSender,
};

/// Client output events.
#[derive(Debug, PartialEq, Eq)]
pub enum ClientOutput {
    /// Attempted to connect to the server, `true` if connected.
    Connected(bool),
    /// Server disconnected.
    ServerDisconnected,
    /// Lost connection to the server.
    LostConnection,
}

#[derive(Debug)]
pub enum ClientCommand {
    Disconnect,
    Ping,
}

pub struct ClientHandler {
    network: NetworkController<Self>,
    output: OutputSender<ClientOutput>,
    server: ConnectionId,
}

impl ClientHandler {
    pub const fn new(
        network: NetworkController<Self>,
        output: OutputSender<ClientOutput>,
        server: ConnectionId,
    ) -> Self {
        Self {
            network,
            output,
            server,
        }
    }

    fn output(&mut self, output: ClientOutput) {
        self.output.send(output);
    }

    fn disconnect(&self) {
        self.network.remove(self.server);
        self.network.stop();
    }

    fn ping(&self) {
        self.network
            .send(self.server, SharedMessage::Ping(Instant::now()));
    }
}

impl NetworkHandler for ClientHandler {
    type Message = SharedMessage;
    type Command = ClientCommand;

    fn handle_event(&mut self, event: NetworkEvent) {
        match event {
            NetworkEvent::ConnectionAttempt(_, succeeded) => {
                self.output(ClientOutput::Connected(succeeded));
                if !succeeded {
                    self.network.stop();
                }
            }
            NetworkEvent::ConnectionLost(_, disconnected) => {
                self.output(if disconnected {
                    ClientOutput::ServerDisconnected
                } else {
                    ClientOutput::LostConnection
                });
                self.network.stop();
            }
            NetworkEvent::NewConnection(_) => unreachable!("Clients cant accept new connections"),
        }
    }

    fn handle_message(&mut self, conn: ConnectionId, message: &Self::Message) {
        if let SharedMessage::Ping(elapsed) = message {
            self.network.send(conn, SharedMessage::Pong(*elapsed));
        }

        if let SharedMessage::Pong(elapsed) = message {
            println!("[Client] Pong from {conn:?} in {:?}", elapsed.elapsed());
        } else {
            println!("[Client] Got message: {message:?} from {conn:?}");
        }
    }

    fn handle_command(&mut self, command: &Self::Command) {
        match command {
            ClientCommand::Disconnect => self.disconnect(),
            ClientCommand::Ping => self.ping(),
        }
    }
}
