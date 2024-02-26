#![allow(dead_code, unused)] // WIP

use super::{ClientCommand, Message, ServerCommand};
use crate::network::{Endpoint, NetworkController, NetworkEvent, NetworkHandler};

pub enum ClientServerCommand {
    Client(ClientCommand),
    Server(ServerCommand),
}

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
    server: Endpoint,
}

impl ClientServerHandler {
    pub const fn new(network: NetworkController<Self>, server: Endpoint) -> Self {
        Self { network, server }
    }

    pub fn conn_is_server(&self, conn: &Endpoint) -> bool {
        conn.addr() == self.server.addr()
    }
}

impl NetworkHandler for ClientServerHandler {
    type Message = Message;
    type Command = ClientServerCommand;

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
