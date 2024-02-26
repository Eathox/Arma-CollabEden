#![allow(dead_code, unused)] // WIP

use super::{Message, OutputReceiver, OutputSender};
use crate::network::{Endpoint, NetworkController, NetworkEvent, NetworkHandler};

#[derive(Debug)]
pub enum ClientHostCommand {}

/// Client hosted server output event, received through [`Manager::output`].
///
/// [`Manager::output`]: crate::Manager::output
#[derive(Debug)]
pub enum ClientHostOutput {}

pub struct ClientHostHandler {
    network: NetworkController<Self>,
    output: OutputSender<ClientHostOutput>,
    server: Endpoint,
}

impl ClientHostHandler {
    pub fn new(
        network: NetworkController<Self>,
        server: Endpoint,
        enable_output: bool,
    ) -> (Self, OutputReceiver<ClientHostOutput>) {
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

impl NetworkHandler for ClientHostHandler {
    type Message = Message;
    type Command = ClientHostCommand;
    type Output = ClientHostOutput;

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
