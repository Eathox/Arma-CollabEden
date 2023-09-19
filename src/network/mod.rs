use std::net::SocketAddr;
use std::sync::Arc;

use crossbeam_channel::Sender;
use message_io::network::Endpoint;

use message_io::events::EventSender;
use message_io::node::{NodeEvent, NodeHandler, NodeTask};
use serde::{Deserialize, Serialize};

pub mod client;
pub mod server;

pub use client::{ClientMessage, ClientSignal};
pub use server::{ServerMessage, ServerSignal};

use crate::{Callback, Error, Result};

pub struct Handler<S: Send + 'static> {
    addr: SocketAddr,
    handler: NodeHandler<S>,
    listen_task: Arc<NodeTask>,
}

impl<S: Send + 'static> Handler<S> {
    pub fn new(addr: SocketAddr, handler: NodeHandler<S>, listen_task: NodeTask) -> Self {
        Self {
            addr,
            handler,
            listen_task: Arc::new(listen_task),
        }
    }

    #[must_use]
    pub const fn address(&self) -> SocketAddr {
        self.addr
    }

    #[must_use]
    pub fn signals(&self) -> &EventSender<S> {
        self.handler.signals()
    }
}

impl<S: Send + 'static> Clone for Handler<S> {
    fn clone(&self) -> Self {
        Self {
            addr: self.addr,
            handler: self.handler.clone(),
            listen_task: self.listen_task.clone(),
        }
    }
}

#[derive(Clone)]
pub enum NetworkHandler {
    Server(Handler<ServerSignal>),
    Client(Handler<ClientSignal>),
}

fn serialize_message<M>(msg: &M) -> Result<Vec<u8>>
where
    M: Serialize,
{
    // bincode::DefaultOptions::new()
    //     .serialize(msg)
    //     .map_err(Error::generic)
    serde_json::to_vec(msg).map_err(Error::generic)
}

fn deserialize_message<'a, M>(data: &'a [u8]) -> Result<M>
where
    M: Deserialize<'a>,
{
    // bincode::DefaultOptions::new()
    //     .deserialize(data)
    //     .map_err(Error::generic)
    serde_json::from_slice(data).map_err(Error::generic)
}

trait NetworkInterface<'a, S, M>
where
    M: Deserialize<'a> + Serialize + std::fmt::Debug,
{
    fn local_addr(&self) -> &SocketAddr;
    fn callback_sender(&self) -> &Sender<Callback>;
    fn handler(&self) -> &NodeHandler<S>;
    fn handle_event(&mut self, event: NodeEvent<S>);

    fn send_message(&self, endpoint: Endpoint, message: M) {
        info!("sending: {message:?}");
        let data = serialize_message(&message).unwrap();
        self.handler().network().send(endpoint, &data);
    }
    fn callback(&self, callback: Callback) {
        self.callback_sender().send(callback).unwrap();
    }
}
