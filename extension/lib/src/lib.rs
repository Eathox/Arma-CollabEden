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

use handlers::{client, server, ArmaEvent};
use network::{ListenerLifetime, NetworkController, NetworkHandler};

pub use builder::ManagerBuilder;
pub use error::{Error, Result};
pub use handlers::{client::Output as ClientOutput, server::Output as ServerOutput};
pub use network::ConnectionId;

/// Unique network entity id.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    arma_rs::IntoArma,
    arma_rs::FromArma,
)]
pub struct NetEntityId(u32);

impl NetEntityId {
    /// Create new network entity id for testing.
    #[cfg(debug_assertions)]
    #[inline]
    #[must_use]
    pub const fn test(id: u32) -> Self {
        Self(id)
    }
}

impl std::fmt::Display for NetEntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Net:{}", self.0)
    }
}

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

struct CommonManager<H: NetworkHandler> {
    _lifetime: ListenerLifetime,
    controller: NetworkController<H>,
    addr: SocketAddr,
    server_addr: SocketAddr,
}

macro_rules! impl_common_manager {
    ($manager:ident, $module:ident) => {
        impl InstanceManager for $manager {
            #[inline]
            #[must_use]
            fn addr(&self) -> SocketAddr {
                self.common.addr
            }

            #[inline]
            #[must_use]
            fn server_addr(&self) -> SocketAddr {
                self.common.server_addr
            }

            #[inline]
            fn disconnect(&self) {
                self.common
                    .controller
                    .command($module::Command::Disconnect, None);
            }

            #[inline]
            fn stop(&self) {
                self.common.controller.stop();
            }
        }

        impl Drop for $manager {
            fn drop(&mut self) {
                self.stop();
            }
        }
    };
}

/// Dedicated server network manager.
#[must_use]
pub struct ServerManager {
    common: CommonManager<server::Handler>,
}

impl_common_manager!(ServerManager, server);

impl ServerManager {
    const fn new(common: CommonManager<server::Handler>) -> Self {
        Self { common }
    }
}

/// Client network manager.
#[must_use]
pub struct ClientManager {
    common: CommonManager<client::Handler>,
}

impl_common_manager!(ClientManager, client);

impl ClientManager {
    const fn new(common: CommonManager<client::Handler>) -> Self {
        Self { common }
    }

    /// Reserve a unused unique entity network id, the id is returned by [`ClientOutput::EntityNetId`].
    pub fn reserve_net_id(&self) {
        self.common
            .controller
            .command(client::Command::RequestEntityNetId, None);
    }

    /// Broadcast an arma event over the network.
    pub fn arma_event(&self, name: &str, data: arma_rs::Value) {
        let event = ArmaEvent::Event {
            name: name.to_owned(),
            params: data,
        };
        let command = client::Command::ArmaEvent(event);
        self.common.controller.command(command, None);
    }

    /// Broadcast an arma entity event over the network.
    pub fn arma_entity_event(&self, net_id: NetEntityId, name: &str, data: arma_rs::Value) {
        let event = ArmaEvent::EntityEvent {
            id: net_id,
            name: name.to_owned(),
            params: data,
        };
        let command = client::Command::ArmaEvent(event);
        self.common.controller.command(command, None);
    }
}

/// Output channel for the network manager.
pub type OutputReceiver<O> = Receiver<O>;

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
