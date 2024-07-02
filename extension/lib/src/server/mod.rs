use std::time::Duration;

use super::client::Message as ClientMessage;
use crate::{
    entity_id::NetEntityIdGen,
    networking::NetworkSerde,
    networking::{ConnectionId, NetworkController, NetworkEvent, NetworkHandler},
    CommonMessage, NetEntityId, OutputSender, PingTimer,
};

mod manager;
mod output;

pub use manager::{Command, Manager};
pub use output::Output;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Message {
    ReservedEntityNetId(NetEntityId),

    Common(CommonMessage),
}

impl NetworkSerde for Message {}

impl From<CommonMessage> for Message {
    fn from(message: CommonMessage) -> Self {
        Self::Common(message)
    }
}

pub struct Handler {
    network: NetworkController<Self>,
    output: OutputSender<Output>,
    clients: Vec<ConnectionId>,
    net_id: NetEntityIdGen,
}

impl NetworkHandler for Handler {
    type RecvMessage = ClientMessage;
    type SendMessage = Message;
    type Command = Command;

    fn handle_event(&mut self, event: NetworkEvent) {
        match event {
            NetworkEvent::NewConnection(conn) => {
                self.output.send(Output::ClientConnected(conn));
                self.clients.push(conn);
            }

            NetworkEvent::ConnectionLost(conn, disconnected) => {
                self.output.send(if disconnected {
                    Output::ClientDisconnected(conn)
                } else {
                    Output::LostConnection(conn)
                });
                self.clients.retain(|c| c != &conn);
            }

            NetworkEvent::ConnectionAttempt(_, _) => {
                unreachable!("Servers cant attempt to connect")
            }
        }
    }

    fn handle_message(&mut self, conn: ConnectionId, message: Self::RecvMessage) {
        match message {
            ClientMessage::RequestEntityNetId => {
                let net_id = self.net_id.next();
                self.network
                    .send(conn, Message::ReservedEntityNetId(net_id));
            }

            ClientMessage::Common(message) => match message {
                CommonMessage::ArmaEvent(event) => {
                    self.propagate(conn, || CommonMessage::ArmaEvent(event.clone()).into());
                }

                CommonMessage::Ping(timer) => {
                    self.network.send(conn, CommonMessage::Pong(timer).into());
                }
                CommonMessage::Pong(timer) => {
                    self.output.send(Output::Ping(conn, timer.elapsed()));
                }
            },
        }
    }

    fn handle_command(&mut self, command: Self::Command) {
        match command {
            Command::PingLoop(interval) => self.ping_loop(interval),
            Command::Shutdown => self.shutdown(),
        }
    }
}

impl Handler {
    pub fn new(
        network: NetworkController<Self>,
        output: OutputSender<Output>,
        ping: Option<Duration>,
    ) -> Self {
        let handler = Self {
            network,
            output,
            clients: vec![],
            net_id: NetEntityIdGen::new(),
        };

        if let Some(ping) = ping {
            handler.ping_loop(ping);
        }

        handler
    }

    fn propagate(&self, origin: ConnectionId, f: impl Fn() -> Message) {
        for client in &self.clients {
            if client != &origin {
                self.network.send(*client, f());
            }
        }
    }

    fn ping_loop(&self, interval: Duration) {
        for client in &self.clients {
            let message = CommonMessage::Ping(PingTimer::new());
            self.network.send(*client, message.into());
        }

        self.network
            .command(Command::PingLoop(interval), Some(interval));
    }

    fn shutdown(&self) {
        for client in &self.clients {
            self.network.remove(*client);
        }
        self.network.stop();
        self.output.send(Output::Shutdown);
    }
}
