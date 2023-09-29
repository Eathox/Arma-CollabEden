use std::net::SocketAddr;

use arma_rs::Value;
use crossbeam_channel::Sender;
use message_io::network::{Endpoint, NetEvent, Transport};
use message_io::node::{self, NodeEvent, NodeHandler};

use super::{deserialize_message, Handler, NetworkInterface, ServerMessage};
use crate::{Callback, Error, Result};

// TODO: add more boilerplate messages like one to send before disconnecting, to determent if a connection is lost or was willingly disconnected
// TODO: add support fot checking addons are the same between clients
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum ClientMessage {
    Ping,
    Pong,
    ArmaEvent(String, Value),
}

pub enum ClientSignal {
    LostConnection,
    Disconnect,
    Ping,
    SendArmaEvent(String, Value),
}

struct Client {
    addr: SocketAddr,
    server_addr: SocketAddr,
    callback_sender: Sender<Callback>,
    handler: NodeHandler<ClientSignal>,
    server: Endpoint,
}

impl NetworkInterface<'_, ClientSignal, ClientMessage> for Client {
    fn local_addr(&self) -> &SocketAddr {
        &self.addr
    }

    fn handler(&self) -> &NodeHandler<ClientSignal> {
        &self.handler
    }

    fn callback_sender(&self) -> &Sender<Callback> {
        &self.callback_sender
    }

    fn handle_event(&mut self, event: NodeEvent<ClientSignal>) {
        match event {
            NodeEvent::Network(net_event) => match net_event {
                NetEvent::Connected(_, succeeded) => {
                    self.callback(Callback::ClientConnected(succeeded));
                    if !succeeded {
                        self.handler.stop();
                    }
                }
                NetEvent::Accepted(_, _) => unreachable!("clients don't accept connections"),
                NetEvent::Message(server, data) => {
                    if server != self.server {
                        error!("received message from unknown server: {server}");
                        return;
                    }

                    match deserialize_message(data) {
                        Ok(message) => {
                            info!("received: {message:?}");
                            self.handle_message(message);
                        }
                        Err(err) => {
                            error!("failed to deserialize message: {err}");
                        }
                    }
                }
                NetEvent::Disconnected(_) => {
                    self.handler.signals().send(ClientSignal::LostConnection);
                }
            },
            NodeEvent::Signal(signal) => self.handle_signal(signal),
        }
    }
}

impl Client {
    fn handle_message(&self, message: ServerMessage) {
        match message {
            ServerMessage::Ping => self.send_message(self.server, ClientMessage::Pong),
            ServerMessage::Pong => {}
            ServerMessage::ArmaEvent(event, data) => {
                self.callback(Callback::ReceivedEvent(event, data));
            }
        }
    }

    fn handle_signal(&self, signal: ClientSignal) {
        match signal {
            ClientSignal::LostConnection | ClientSignal::Disconnect => {
                let lost_connection = matches!(signal, ClientSignal::LostConnection);
                self.callback(Callback::ClientDisconnected(lost_connection));
                self.handler.stop();
            }
            ClientSignal::Ping => self.send_message(self.server, ClientMessage::Ping),
            ClientSignal::SendArmaEvent(event, data) => {
                self.send_message(self.server, ClientMessage::ArmaEvent(event, data));
            }
        }
    }
}

pub fn start(
    callback_sender: Sender<Callback>,
    server_addr: SocketAddr,
) -> Result<Handler<ClientSignal>> {
    let (handler, listener) = node::split::<ClientSignal>();
    let (server, addr) = handler
        .network()
        .connect(Transport::FramedTcp, server_addr)
        .map_err(Error::generic)?;
    let mut client = Client {
        addr,
        server_addr,
        callback_sender,
        handler: handler.clone(),
        server,
    };
    Ok(Handler::new(
        addr,
        handler,
        listener.for_each_async(move |event| client.handle_event(event)),
    ))
}
