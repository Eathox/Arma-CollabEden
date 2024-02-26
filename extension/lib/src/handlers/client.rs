use std::time::Instant;

use super::Message;
use crate::network::{Endpoint, NetworkController, NetworkEvent, NetworkHandler};

#[derive(Debug)]
pub enum ClientCommand {
    Ping,
}

pub struct ClientHandler {
    network: NetworkController<Self>,
    server: Endpoint,
}

impl ClientHandler {
    pub const fn new(network: NetworkController<Self>, server: Endpoint) -> Self {
        Self { network, server }
    }

    pub const fn server(&self) -> &Endpoint {
        &self.server
    }
}

impl NetworkHandler for ClientHandler {
    type Message = Message;
    type Command = ClientCommand;

    fn handle_event(&mut self, event: NetworkEvent) {
        match event {
            NetworkEvent::ConnectionAttempt(_, succeeded) => {
                println!("[Client] Connection attempt to server: {succeeded}");
                if !succeeded {
                    self.network.stop();
                }
            }
            NetworkEvent::ConnectionLost(_, disconnected) => {
                if disconnected {
                    println!("[Client] Server disconnected");
                } else {
                    println!("[Client] Lost connection to server");
                }
                self.network.stop();
            }
            NetworkEvent::NewConnection(_) => unreachable!("Clients cant accept new connections"),
        }
    }

    fn handle_message(&mut self, conn: Endpoint, message: &Self::Message) {
        if let Message::Ping(elapsed) = message {
            self.network.send(conn, Message::Pong(*elapsed));
        }

        if let Message::Pong(elapsed) = message {
            println!("[Client] Pong from {conn:?} in {:?}", elapsed.elapsed());
        } else {
            println!("[Client] Got message: {message:?} from {conn:?}");
        }
    }

    fn handle_command(&mut self, command: &Self::Command) {
        match command {
            ClientCommand::Ping => {
                self.network
                    .send(self.server, Message::Ping(Instant::now()));
            }
        }
    }
}
