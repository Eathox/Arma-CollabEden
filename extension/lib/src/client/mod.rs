use std::time::Duration;

use super::server::Message as ServerMessage;
use crate::{
    networking::NetworkSerde,
    networking::{ConnectionId, NetworkController, NetworkEvent, NetworkHandler},
    ArmaEvent, CommonMessage, OutputSender, PingTimer,
};

mod manager;
mod output;

pub use manager::{Command, Manager};
pub use output::Output;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Message {
    RequestEntityNetId,

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
    server: ConnectionId,
}

impl NetworkHandler for Handler {
    type RecvMessage = ServerMessage;
    type SendMessage = Message;
    type Command = Command;

    fn handle_event(&mut self, event: NetworkEvent) {
        match event {
            NetworkEvent::ConnectionAttempt(_, succeeded) => {
                self.output.send(if succeeded {
                    Output::Connected
                } else {
                    Output::FailedToConnect
                });
            }

            NetworkEvent::ConnectionLost(_, disconnected) => {
                self.output.send(if disconnected {
                    Output::ServerDisconnected
                } else {
                    Output::LostConnection
                });
            }

            NetworkEvent::NewConnection(_) => {
                unreachable!("Clients cant accept new connections");
            }
        }
    }

    fn handle_message(&mut self, conn: ConnectionId, message: Self::RecvMessage) {
        match message {
            ServerMessage::ReservedEntityNetId(id) => {
                self.output.send(Output::ReservedEntityNetId(id));
            }

            ServerMessage::Common(message) => match message {
                CommonMessage::ArmaEvent(event) => {
                    self.output.send(event.into());
                }

                CommonMessage::Ping(timer) => {
                    self.network.send(conn, CommonMessage::Pong(timer).into());
                }
                CommonMessage::Pong(timer) => {
                    self.output.send(Output::Ping(timer.elapsed()));
                }
            },
        }
    }

    fn handle_command(&mut self, command: Self::Command) {
        match command {
            Command::RequestEntityNetId => {
                self.network.send(self.server, Message::RequestEntityNetId);
            }
            Command::SendArmaEvent(event) => {
                let message = CommonMessage::ArmaEvent(event);
                self.network.send(self.server, message.into());
            }
            Command::PingLoop(interval) => self.ping_loop(interval),
            Command::Shutdown => self.shutdown(),
        }
    }
}

impl Handler {
    pub fn new(
        network: NetworkController<Self>,
        output: OutputSender<Output>,
        server: ConnectionId,
        ping: Option<Duration>,
    ) -> Self {
        let handler = Self {
            network,
            output,
            server,
        };

        if let Some(ping) = ping {
            handler.ping_loop(ping);
        }

        handler
    }

    fn ping_loop(&self, interval: Duration) {
        let message = CommonMessage::Ping(PingTimer::new());
        self.network.send(self.server, message.into());

        self.network
            .command(Command::PingLoop(interval), Some(interval));
    }

    fn shutdown(&self) {
        self.network.remove(self.server);
        self.network.stop();
        self.output.send(Output::Shutdown);
    }
}
