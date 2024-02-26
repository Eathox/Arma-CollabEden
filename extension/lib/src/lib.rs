#![deny(missing_docs, clippy::all, clippy::nursery)]
#![warn(clippy::pedantic, clippy::unwrap_used)]
#![allow(clippy::similar_names, clippy::module_name_repetitions)]

//! Networking library to add multiplayer capabilities to Arma 3s Eden Editor.

#[allow(unused_imports)]
#[macro_use]
extern crate log;

use std::net::SocketAddr;

mod error;
mod handlers;
mod network;

pub use error::{Error, Result};
use handlers::{ClientHandler, ClientServerHandler, ServerHandler};
use network::{new_network_interface, ListenerLifetime, NetworkController, NetworkHandler};

/// Manager responsible for networking, constructed with [`ManagerBuilder`].
/// Can be configured to be either a server, client or a client hosted server.
pub struct Manager<H: NetworkHandler> {
    addr: SocketAddr,
    controller: NetworkController<H>,
    _lifetime: ListenerLifetime,
}

impl<H: NetworkHandler> Manager<H> {
    /// For clients the address its connected to, for servers the address its listening on.
    #[inline]
    #[must_use]
    pub const fn server_addr(&self) -> SocketAddr {
        self.addr
    }
}

impl<H: NetworkHandler> Drop for Manager<H> {
    fn drop(&mut self) {
        self.controller.stop();
    }
}

struct Addr(SocketAddr);
struct NoAddr;

/// Builder for [`Manager`].
pub struct ManagerBuilder<HostOn, ConnectTo> {
    host: HostOn,
    connect: ConnectTo,
}

impl ManagerBuilder<NoAddr, NoAddr> {
    /// Start building a new network manager.
    #[allow(private_interfaces)]
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            host: NoAddr,
            connect: NoAddr,
        }
    }

    /// Configure to be a client connecting to the given address.
    #[inline]
    #[must_use]
    pub const fn connect_to(self, remote: SocketAddr) -> ManagerBuilder<NoAddr, Addr> {
        ManagerBuilder {
            host: NoAddr,
            connect: Addr(remote),
        }
    }

    /// Configure to be a server hosting on the given address.
    #[inline]
    #[must_use]
    pub const fn host_on(self, local: SocketAddr) -> ManagerBuilder<Addr, NoAddr> {
        ManagerBuilder {
            host: Addr(local),
            connect: NoAddr,
        }
    }
}

// Server
impl ManagerBuilder<Addr, NoAddr> {
    /// Convert to a client hosted server.
    #[inline]
    #[must_use]
    pub const fn as_client(self) -> ManagerBuilder<Addr, Addr> {
        ManagerBuilder {
            host: Addr(self.host.0),
            connect: Addr(self.host.0),
        }
    }

    /// Complete the configuration and boot up the server.
    ///
    /// # Errors
    /// Returns an error if the address is unable to be used to listen on.
    #[inline]
    pub fn startup(self) -> Result<Manager<ServerHandler>> {
        let (controller, listener) = new_network_interface();
        let addr = controller.listen(self.host.0)?;

        let handler = ServerHandler::new(controller.clone());
        let lifetime = listener.start(handler);
        Ok(Manager {
            addr,
            controller,
            _lifetime: lifetime,
        })
    }
}

// Client
impl ManagerBuilder<NoAddr, Addr> {
    /// Complete the configuration and boot up the client.
    ///
    /// # Errors
    /// Returns an error if the address is unable to be used to connect to.
    #[inline]
    pub fn startup(self) -> Result<Manager<ClientHandler>> {
        let (controller, listener) = new_network_interface();
        let conn = controller.connect(self.connect.0)?;

        let handler = ClientHandler::new(controller.clone(), conn);
        let lifetime = listener.start(handler);
        Ok(Manager {
            addr: conn.addr(),
            controller,
            _lifetime: lifetime,
        })
    }
}

// ClientServer
impl ManagerBuilder<Addr, Addr> {
    /// Complete the configuration and boot up the client server.
    ///
    /// # Errors
    /// Returns an error if the address is unable to be used to listen on.
    #[inline]
    pub fn startup(self) -> Result<Manager<ClientServerHandler>> {
        let (controller, listener) = new_network_interface();
        let addr = controller.listen(self.host.0)?;
        let conn = controller.connect(addr)?;

        let handler = ClientServerHandler::new(controller.clone(), conn);
        let lifetime = listener.start(handler);
        Ok(Manager {
            addr,
            controller,
            _lifetime: lifetime,
        })
    }
}
