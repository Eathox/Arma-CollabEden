use std::time::Instant;

use super::Message;
use crate::network::{Endpoint, NetworkController, NetworkEvent, NetworkHandler};

#[derive(Debug)]
pub enum ServerCommand {
    Ping,
}

pub struct ServerHandler {
    network: NetworkController<Self>,
    clients: Vec<Endpoint>,
}

impl ServerHandler {
    pub const fn new(network: NetworkController<Self>) -> Self {
        Self {
            network,
            clients: vec![],
        }
    }
}

impl NetworkHandler for ServerHandler {
    type Message = Message;
    type Command = ServerCommand;

    fn handle_event(&mut self, event: NetworkEvent) {
        match event {
            NetworkEvent::NewConnection(conn) => {
                println!("[Server] New connection from: {conn:?}");
                self.clients.push(conn);
            }
            NetworkEvent::ConnectionLost(conn, disconnected) => {
                if disconnected {
                    println!("[Server] Client disconnected: {conn:?}");
                } else {
                    println!("[Server] Lost connection to client: {conn:?}");
                }
                self.clients.retain(|c| c != &conn);
            }
            NetworkEvent::ConnectionAttempt(_, _) => {
                unreachable!("Servers cant attempt to connect")
            }
        }
    }

    fn handle_message(&mut self, conn: Endpoint, message: &Self::Message) {
        if let Message::Ping(elapsed) = message {
            self.network.send(conn, Message::Pong(*elapsed));
        }

        if let Message::Pong(elapsed) = message {
            println!("[Server] Pong from {conn:?} in {:?}", elapsed.elapsed());
        } else {
            println!("[Server] Got message: {message:?} from {conn:?}");
        }
    }

    fn handle_command(&mut self, command: &Self::Command) {
        match command {
            ServerCommand::Ping => {
                for client in &self.clients {
                    self.network.send(*client, Message::Ping(Instant::now()));
                }
            }
        }
    }
}
