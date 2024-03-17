#![deny(clippy::all, clippy::nursery)]
#![warn(missing_docs, clippy::pedantic, clippy::unwrap_used)]
#![allow(clippy::similar_names, clippy::module_name_repetitions)]

//! Networking library to add multiplayer capabilities to Arma 3s Eden Editor.

#[allow(unused_imports)]
#[macro_use]
extern crate log;

use std::net::SocketAddr;

use crossbeam_channel::{unbounded, Receiver, Sender};

mod builder;
mod error;
mod handlers;
mod network;

pub use builder::ManagerBuilder;
pub use error::{Error, Result};
use handlers::{client, server};
pub use handlers::{client::Output as ClientOutput, server::Output as ServerOutput};
pub use network::ConnectionId;
use network::{ListenerLifetime, NetworkController, NetworkHandler};

/// Manager responsible for a networking instance, constructed with [`ManagerBuilder`].
/// Can be configured to be either a server, client or a client hosted server.
pub trait InstanceManager {
    /// Local address used by the instance.
    #[must_use]
    fn addr(&self) -> SocketAddr;

    /// Address of the server the instance is connected to, for host instances this is the same as [`Self::addr`].
    #[must_use]
    fn server_addr(&self) -> SocketAddr;

    /// Disconnect the instance.
    fn disconnect(&self);

    /// Stop the instance.
    fn stop(&self);
}

struct SharedManager<H: NetworkHandler> {
    _lifetime: ListenerLifetime,
    controller: NetworkController<H>,
    addr: SocketAddr,
    server_addr: SocketAddr,
}

macro_rules! impl_shared_manager {
    ($manager:ident, $module:ident) => {
        impl InstanceManager for $manager {
            #[inline]
            #[must_use]
            fn addr(&self) -> SocketAddr {
                self.shared.addr
            }

            #[inline]
            #[must_use]
            fn server_addr(&self) -> SocketAddr {
                self.shared.server_addr
            }

            #[inline]
            fn disconnect(&self) {
                self.shared.controller.command($module::Command::Disconnect);
            }

            #[inline]
            fn stop(&self) {
                self.shared.controller.stop();
            }
        }

        impl Drop for $manager {
            /// Call [`Self::stop`] when dropped.
            fn drop(&mut self) {
                self.stop();
            }
        }
    };
}

/// Dedicated server network manager.
#[must_use]
pub struct ServerManager {
    shared: SharedManager<server::Handler>,
}

impl_shared_manager!(ServerManager, server);

impl ServerManager {
    const fn new(shared: SharedManager<server::Handler>) -> Self {
        Self { shared }
    }
}

/// Client network manager.
#[must_use]
pub struct ClientManager {
    shared: SharedManager<client::Handler>,
}

impl_shared_manager!(ClientManager, client);

impl ClientManager {
    const fn new(shared: SharedManager<client::Handler>) -> Self {
        Self { shared }
    }
}

type OutputReceiver<O> = Receiver<O>;

struct OutputSender<O> {
    output: Sender<O>,
    output_enabled: bool,
}

impl<O> OutputSender<O> {
    fn new() -> (Self, OutputReceiver<O>) {
        let (sender, receiver) = unbounded();
        (
            Self {
                output: sender,
                output_enabled: true,
            },
            receiver,
        )
    }

    fn disable(&mut self) {
        info!("Disabling output");
        self.output_enabled = false;
    }

    fn send(&mut self, output: O) {
        if !self.output_enabled {
            return;
        };

        if self.output.send(output).is_err() {
            self.disable();
            error!("Output channel is disconnected");
        };
    }
}
