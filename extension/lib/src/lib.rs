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
mod id;
mod network;

use handlers::{client, server, ArmaEvent};
use id::EntityIdMap;
use network::{ListenerLifetime, NetworkController, NetworkHandler};

pub use builder::ManagerBuilder;
pub use error::{Error, Result};
pub use handlers::{client::Output as ClientOutput, server::Output as ServerOutput};
pub use id::{LocalEntityId, NetEntityId};
pub use network::ConnectionId;

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
                self.shared
                    .controller
                    .command($module::Command::Disconnect, None);
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
    entity_map: EntityIdMap,
}

impl_shared_manager!(ClientManager, client);

impl ClientManager {
    const fn new(shared: SharedManager<client::Handler>, entity_map: EntityIdMap) -> Self {
        Self { shared, entity_map }
    }

    /// Reserve a unused unique entity network id, the id is returned by [`ClientOutput::EntityNetId`].
    pub fn reserve_net_id(&self) {
        self.shared
            .controller
            .command(client::Command::RequestEntityNetId, None);
    }

    /// Add entity to the network mapping, requires a unique net id, see [`Self::reserve_net_id`].
    pub fn add_entity(&mut self, net_id: NetEntityId, local_id: LocalEntityId) {
        self.entity_map.add(net_id, local_id);
    }

    /// Remove entity from the network mapping.
    pub fn remove_entity(&mut self, net_id: NetEntityId) {
        self.entity_map.remove(net_id);
    }

    /// Get the network id from the local id.
    #[must_use]
    pub fn get_net_id(&self, local_id: LocalEntityId) -> Option<NetEntityId> {
        self.entity_map.get_net_id(local_id)
    }

    /// Get the local id from the network id.
    #[must_use]
    pub fn get_local_id(&self, net_id: NetEntityId) -> Option<LocalEntityId> {
        self.entity_map.get_local_id(net_id)
    }

    /// Broadcast an arma event over the network.
    pub fn arma_event(&self, name: &str, data: arma_rs::Value) {
        let event = ArmaEvent::Event {
            name: name.to_owned(),
            params: data,
        };
        let command = client::Command::ArmaEvent(event);
        self.shared.controller.command(command, None);
    }

    /// Broadcast an arma entity event over the network.
    pub fn arma_entity_event(&self, net_id: NetEntityId, name: &str, data: arma_rs::Value) {
        let event = ArmaEvent::EntityEvent {
            id: net_id,
            name: name.to_owned(),
            params: data,
        };
        let command = client::Command::ArmaEvent(event);
        self.shared.controller.command(command, None);
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
