use std::time::Instant;

use super::{Message, OutputReceiver, OutputSender};
use crate::network::{Endpoint, NetworkController, NetworkEvent, NetworkHandler};

#[derive(Debug)]
pub enum ClientCommand {
    Disconnect,
    Ping,
}

/// Client output event, received through [`Manager::output`].
///
/// [`Manager::output`]: crate::Manager::output
#[derive(Debug)]
pub enum ClientOutput {}

pub struct ClientHandler {
    network: NetworkController<Self>,
    output: OutputSender<ClientOutput>,
    server: Endpoint,
}

impl ClientHandler {
    pub fn new(
        network: NetworkController<Self>,
        server: Endpoint,
        enable_output: bool,
    ) -> (Self, OutputReceiver<ClientOutput>) {
        let (mut sender, receiver) = OutputSender::new();
        if !enable_output {
            sender.disable();
        }

        (
            Self {
                network,
                output: sender,
                server,
            },
            receiver,
        )
    }

    fn disconnect(&self) {
        self.network.remove(self.server);
        self.network.stop();
    }

    fn ping(&self) {
        self.network
            .send(self.server, Message::Ping(Instant::now()));
    }
}

impl NetworkHandler for ClientHandler {
    type Message = Message;
    type Command = ClientCommand;
    type Output = ClientOutput;

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
            ClientCommand::Disconnect => self.disconnect(),
            ClientCommand::Ping => self.ping(),
        }
    }
}
