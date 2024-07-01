use super::{server::Message as ServerMessage, ArmaEvent, CommonMessage, PingTimer, PING_INTERVAL};
use crate::{
    network::{ConnectionId, NetworkController, NetworkEvent, NetworkHandler, NetworkSerde},
    OutputSender,
};

mod output;

pub use output::Output;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum Message {
    ReserveEntityNetId,

    Common(CommonMessage),
}

impl NetworkSerde for Message {}

impl From<CommonMessage> for Message {
    fn from(message: CommonMessage) -> Self {
        Self::Common(message)
    }
}

#[derive(Debug)]
pub enum Command {
    RequestEntityNetId,
    ArmaEvent(ArmaEvent),
    Disconnect,
    PingLoop,
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
                self.output.send(Output::ServerConnected(succeeded));
                if !succeeded {
                    self.network.stop();
                }
            }
            NetworkEvent::ConnectionLost(_, disconnected) => {
                self.output.send(if disconnected {
                    Output::ServerDisconnected
                } else {
                    Output::LostConnection
                });
                self.network.stop();
            }
            NetworkEvent::NewConnection(_) => unreachable!("Clients cant accept new connections"),
        }
    }

    fn handle_message(&mut self, conn: ConnectionId, message: Self::RecvMessage) {
        match message {
            ServerMessage::ReservedNetId(id) => {
                self.output.send(Output::ReservedNetId(id));
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
                self.network.send(self.server, Message::ReserveEntityNetId);
            }
            Command::ArmaEvent(event) => {
                let message = CommonMessage::ArmaEvent(event);
                self.network.send(self.server, message.into());
            }

            Command::Disconnect => self.disconnect(),
            Command::PingLoop => self.ping(true),
        }
    }
}

impl Handler {
    pub fn new(
        network: NetworkController<Self>,
        output: OutputSender<Output>,
        server: ConnectionId,
        ping_loop: bool,
    ) -> Self {
        if ping_loop {
            network.command(Command::PingLoop, Some(PING_INTERVAL));
        };

        Self {
            network,
            output,
            server,
        }
    }

    fn disconnect(&self) {
        self.network.remove(self.server);
        self.network.stop();
    }

    fn ping(&self, repeat: bool) {
        let message = CommonMessage::Ping(PingTimer::new());
        self.network.send(self.server, message.into());

        if repeat {
            self.network.command(Command::PingLoop, Some(PING_INTERVAL));
        }
    }
}
