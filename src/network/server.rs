use std::net::SocketAddr;

use arma_rs::Value;
use crossbeam_channel::Sender;
use message_io::network::{Endpoint, NetEvent, Transport};
use message_io::node::{self, NodeEvent, NodeHandler};

use super::{deserialize_message, ClientMessage, Handler, NetworkInterface};
use crate::{Callback, Error, Result};

// TODO: add more boilerplate messages like one to send before disconnecting, to determent if a connection is lost or was willingly disconnected
// TODO: add support fot checking addons are the same between clients
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum ServerMessage {
    Ping,
    Pong,
    ArmaEvent(String, Value),
}

pub enum ServerSignal {
    Stop,
    Ping,
    SendArmaEvent(String, Value, Option<Endpoint>),
}

struct Server {
    addr: SocketAddr,
    callback_sender: Sender<Callback>,
    handler: NodeHandler<ServerSignal>,
    all_clients: Vec<Endpoint>,
}

impl NetworkInterface<'_, ServerSignal, ServerMessage> for Server {
    fn local_addr(&self) -> &SocketAddr {
        &self.addr
    }

    fn handler(&self) -> &NodeHandler<ServerSignal> {
        &self.handler
    }

    fn callback_sender(&self) -> &Sender<Callback> {
        &self.callback_sender
    }

    fn handle_event(&mut self, event: NodeEvent<ServerSignal>) {
        match event {
            NodeEvent::Network(net_event) => match net_event {
                NetEvent::Connected(_, _) => unreachable!("servers don't request connections"),
                NetEvent::Accepted(client, _) => {
                    self.callback(Callback::ServerClientConnected(client.resource_id()));
                    self.all_clients.push(client);
                }
                NetEvent::Message(client, data) => {
                    if !self.all_clients.contains(&client) {
                        error!("received message from unknown client: {client}");
                        return;
                    }

                    match deserialize_message(data) {
                        Ok(message) => {
                            info!("received: {message:?}");
                            self.handle_message(client, message);
                        }
                        Err(err) => {
                            error!("failed to deserialize message: {err}");
                        }
                    }
                }
                NetEvent::Disconnected(client) => {
                    self.callback(Callback::ServerClientDisconnected(client.resource_id()));
                    self.all_clients.retain(|e| e != &client);
                }
            },
            NodeEvent::Signal(signal) => self.handle_signal(signal),
        }
    }
}

impl Server {
    fn handle_message(&self, client: Endpoint, message: ClientMessage) {
        match message {
            ClientMessage::Ping => self.send_message(client, ServerMessage::Pong),
            ClientMessage::Pong => {}
            ClientMessage::ArmaEvent(event, data) => {
                self.callback(Callback::ReceivedEvent(event.clone(), data.clone()));

                // TODO: dont directly send arma events to clients. Works fine with 2 instances. But with more the translation tables will be fucked
                self.handler
                    .signals()
                    .send(ServerSignal::SendArmaEvent(event, data, Some(client)));
            }
        }
    }

    fn handle_signal(&self, signal: ServerSignal) {
        match signal {
            ServerSignal::Stop => {
                self.callback(Callback::ServerStopped);
                self.handler.stop();
            }
            ServerSignal::Ping => {
                for client in &self.all_clients {
                    self.send_message(*client, ServerMessage::Ping);
                }
            }
            ServerSignal::SendArmaEvent(event, data, origin) => {
                for client in &self.all_clients {
                    if Some(*client) != origin {
                        self.send_message(
                            *client,
                            ServerMessage::ArmaEvent(event.clone(), data.clone()),
                        );
                    }
                }
            }
        }
    }
}

pub fn start(callback_sender: Sender<Callback>, addr: SocketAddr) -> Result<Handler<ServerSignal>> {
    let (handler, listener) = node::split::<ServerSignal>();
    let (_, addr) = handler
        .network()
        .listen(Transport::FramedTcp, addr)
        .map_err(Error::generic)?;
    let mut server = Server {
        addr,
        callback_sender,
        handler: handler.clone(),
        all_clients: Vec::new(),
    };
    server.callback(Callback::ServerStarted);
    Ok(Handler::new(
        addr,
        handler,
        listener.for_each_async(move |event| server.handle_event(event)),
    ))
}
