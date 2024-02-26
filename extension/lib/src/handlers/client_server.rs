#![allow(dead_code, unused)] // WIP

use super::{ClientCommand, Message, OutputReceiver, OutputSender, ServerCommand};
use crate::network::{Endpoint, NetworkController, NetworkEvent, NetworkHandler};

#[derive(Debug)]
pub enum ClientServerCommand {
    Client(ClientCommand),
    Server(ServerCommand),
}

/// Client hosted server output event, received through [`Manager::output`].
///
/// [`Manager::output`]: crate::Manager::output
#[derive(Debug)]
pub enum ClientServerOutput {}

impl From<ClientCommand> for ClientServerCommand {
    fn from(command: ClientCommand) -> Self {
        Self::Client(command)
    }
}

impl From<ServerCommand> for ClientServerCommand {
    fn from(command: ServerCommand) -> Self {
        Self::Server(command)
    }
}

pub struct ClientServerHandler {
    network: NetworkController<Self>,
    output: OutputSender<ClientServerOutput>,
    server: Endpoint,
}

impl ClientServerHandler {
    pub fn new(
        network: NetworkController<Self>,
        server: Endpoint,
        enable_output: bool,
    ) -> (Self, OutputReceiver<ClientServerOutput>) {
        let (mut sender, receiver) = OutputSender::new();
        if !enable_output {
            sender.disable();
        }

        (
            Self {
                network,
                server,
                output: sender,
            },
            receiver,
        )
    }

    pub fn conn_is_server(&self, conn: &Endpoint) -> bool {
        conn.addr() == self.server.addr()
    }
}

impl NetworkHandler for ClientServerHandler {
    type Message = Message;
    type Command = ClientServerCommand;
    type Output = ClientServerOutput;

    fn handle_event(&mut self, event: NetworkEvent) {
        todo!()
    }

    fn handle_message(&mut self, conn: Endpoint, message: &Self::Message) {
        todo!()
    }

    fn handle_command(&mut self, command: &Self::Command) {
        todo!()
    }
}
