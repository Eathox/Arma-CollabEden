#![deny(clippy::all, clippy::nursery)] // missing_docs
#![warn(clippy::pedantic, clippy::unwrap_used)]
#![allow(clippy::similar_names, clippy::module_name_repetitions)]

#[allow(unused_imports)]
#[macro_use]
extern crate log;

// TODO:
// document everything (withing reason)
// big refactor make this a library with two bins: arma extension and standalone cli/app
// add translation/conversion table of entity ids
// allow three instance types, client only, server only (standalone from arma), client and server (local host)
// Aim for setup where all clients connect to same server, need to origin of arma event to do id translation
// Allow for different session on server only hosting

use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, Mutex};

use arma_rs::Value;
use arma_rs::{arma, Context, ContextState, Extension};
use callback::start_callback_handler;
use crossbeam_channel::{unbounded, Sender};
use local_ip_address::local_ip;

mod callback;
mod error;
mod logger;
mod network;

pub use callback::{Callback, CALLBACK_NAME};
pub use error::{Error, Result};
use network::{client, server};
pub use network::{ClientSignal, NetworkHandler, ServerSignal};

#[derive(Clone)]
pub struct CurrentHandler(Arc<Mutex<Option<NetworkHandler>>>); // Consider RwLock

impl CurrentHandler {
    pub fn none() -> Self {
        Self(Arc::new(Mutex::new(None)))
    }

    pub fn set(&self, handler: Option<NetworkHandler>) {
        *self.0.lock().unwrap() = handler;
    }

    #[must_use]
    pub fn get(&self) -> Option<NetworkHandler> {
        self.0.lock().unwrap().clone()
    }

    pub fn try_get(&self) -> Result<NetworkHandler> {
        self.get()
            .ok_or_else(|| Error::generic("not hosting or connected to a server"))
    }
}

pub fn host(ctx: Context, ip: String, port: u16) -> Result<()> {
    let current_handler = ctx
        .global()
        .get::<CurrentHandler>()
        .expect("current handler is set on extension creation");
    if current_handler.get().is_some() {
        return Err(Error::generic("already hosting or connected to a server"));
    }

    let callback_sender = ctx
        .global()
        .get::<Sender<Callback>>()
        .expect("callback sender is set on extension creation");
    let ip: IpAddr = ip.parse().or_else(|_| local_ip()).map_err(Error::generic)?;
    let addr = SocketAddr::new(ip, port);
    let server_handler = server::start(callback_sender.clone(), addr)?;
    current_handler.set(Some(NetworkHandler::Server(server_handler)));
    Ok(())
}

fn join(ctx: Context, ip: String, port: u16) -> Result<()> {
    let current_handler = ctx
        .global()
        .get::<CurrentHandler>()
        .expect("current handler is set on extension creation");
    if current_handler.get().is_some() {
        return Err(Error::generic("already hosting or connected to a server"));
    }

    let callback_sender = ctx
        .global()
        .get::<Sender<Callback>>()
        .expect("callback sender is set on extension creation");
    let ip: IpAddr = ip.parse().map_err(Error::generic)?;
    let client_handler = client::start(callback_sender.clone(), SocketAddr::new(ip, port))?;
    current_handler.set(Some(NetworkHandler::Client(client_handler)));
    Ok(())
}

fn stop(ctx: Context) -> Result<()> {
    let current_handler = ctx
        .global()
        .get::<CurrentHandler>()
        .expect("current handler is set on extension creation");

    match current_handler.try_get()? {
        NetworkHandler::Server(server_handler) => {
            server_handler.signals().send(ServerSignal::Stop);
        }
        NetworkHandler::Client(client_handler) => {
            client_handler.signals().send(ClientSignal::Disconnect);
        }
    }
    Ok(())
}

fn ping(ctx: Context) -> Result<()> {
    let current_handler = ctx
        .global()
        .get::<CurrentHandler>()
        .expect("current handler is set on extension creation");

    match current_handler.try_get()? {
        NetworkHandler::Server(server_handler) => {
            server_handler.signals().send(ServerSignal::Ping);
        }
        NetworkHandler::Client(client_handler) => {
            client_handler.signals().send(ClientSignal::Ping);
        }
    }
    Ok(())
}

fn send_arma_event(ctx: Context, name: String, data: Value) -> Result<()> {
    let current_handler = ctx
        .global()
        .get::<CurrentHandler>()
        .expect("current handler is set on extension creation");

    match current_handler.try_get()? {
        NetworkHandler::Server(server_handler) => {
            server_handler
                .signals()
                .send(ServerSignal::SendArmaEvent(name, data, None));
        }
        NetworkHandler::Client(client_handler) => {
            client_handler
                .signals()
                .send(ClientSignal::SendArmaEvent(name, data));
        }
    }
    Ok(())
}

#[arma]
fn init() -> Extension {
    let (callback_send, callback_recv) = unbounded();
    let ext = Extension::build()
        .version(std::env!("CARGO_PKG_VERSION").to_string())
        .state::<CurrentHandler>(CurrentHandler::none())
        .state::<Sender<Callback>>(callback_send)
        .command("host", host)
        .command("join", join)
        .command("stop", stop)
        .command("send_event", send_arma_event)
        .freeze_state()
        .finish();
    start_callback_handler(ext.context(), callback_recv);
    if cfg!(debug_assertions) {
        logger::init(ext.context(), log::Level::Debug);
    } else {
        logger::init(ext.context(), log::Level::Warn);
    }
    ext
}
