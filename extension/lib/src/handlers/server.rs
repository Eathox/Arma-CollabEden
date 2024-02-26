use std::time::Instant;

use super::{Message, OutputReceiver, OutputSender};
use crate::network::{Endpoint, NetworkController, NetworkEvent, NetworkHandler};

#[derive(Debug)]
pub enum ServerCommand {
    Disconnect,
    Ping,
}

/// Server output event, received through [`Manager::output`].
///
/// [`Manager::output`]: crate::Manager::output
#[derive(Debug)]
pub enum ServerOutput {}

pub struct ServerHandler {
    network: NetworkController<Self>,
    output: OutputSender<ServerOutput>,
    clients: Vec<Endpoint>,
}

impl ServerHandler {
    pub fn new(
        network: NetworkController<Self>,
        enable_output: bool,
    ) -> (Self, OutputReceiver<ServerOutput>) {
        let (mut sender, receiver) = OutputSender::new();
        if !enable_output {
            sender.disable();
        }

        (
            Self {
                network,
                output: sender,
                clients: vec![],
            },
            receiver,
        )
    }

    fn disconnect(&self) {
        for client in &self.clients {
            self.network.remove(*client);
        }
        self.network.stop();
    }

    fn ping(&self) {
        for client in &self.clients {
            self.network.send(*client, Message::Ping(Instant::now()));
        }
    }
}

impl NetworkHandler for ServerHandler {
    type Message = Message;
    type Command = ServerCommand;
    type Output = ServerOutput;

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
            ServerCommand::Disconnect => self.disconnect(),
            ServerCommand::Ping => self.ping(),
        }
    }
}
