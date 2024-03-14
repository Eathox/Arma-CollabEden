use std::net::SocketAddr;

use crate::{
    handlers::{ClientHandler, ClientOutput, ServerHandler, ServerOutput},
    network::new_network_interface,
    ClientManager, OutputReceiver, OutputSender, Result, ServerManager, SharedManager,
};

pub struct Connect(SocketAddr);
pub struct Host(SocketAddr);
pub struct None;

/// Builder for [`InstanceManager`].
///
/// [`InstanceManager`]: crate::InstanceManager
pub struct ManagerBuilder<H, C> {
    host: H,
    connect: C,
}

impl ManagerBuilder<None, None> {
    /// Start building a new network manager.
    #[allow(private_interfaces)]
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            host: None,
            connect: None,
        }
    }

    /// Configure to be a client connecting to the given address.
    #[inline]
    #[must_use]
    pub const fn connect_to(self, remote: SocketAddr) -> ManagerBuilder<None, Connect> {
        ManagerBuilder {
            host: None,
            connect: Connect(remote),
        }
    }

    /// Configure to be a server hosting on the given address.
    #[inline]
    #[must_use]
    pub const fn host_on(self, local: SocketAddr) -> ManagerBuilder<Host, None> {
        ManagerBuilder {
            host: Host(local),
            connect: None,
        }
    }
}

// Server
impl ManagerBuilder<Host, None> {
    /// Complete the configuration and boot up the server.
    /// Returns both the server manager and the mangers output receiver.
    ///
    /// # Errors
    /// Returns an error if the address is unable to be used to listen on.
    #[inline]
    pub fn startup(&self) -> Result<(ServerManager, OutputReceiver<ServerOutput>)> {
        let (controller, listener) = new_network_interface();
        let server_addr = controller.listen(self.host.0)?;

        let (sender, receiver) = OutputSender::new();
        let handler = ServerHandler::new(controller.clone(), sender);

        let manager = ServerManager::new(SharedManager {
            _lifetime: listener.start(handler),
            controller,
            addr: server_addr,
            server_addr,
        });
        Ok((manager, receiver))
    }
}

// Client
impl ManagerBuilder<None, Connect> {
    /// Complete the configuration and boot up the client.
    /// Returns both the client manager and the mangers output receiver.
    ///
    /// # Errors
    /// Returns an error if the address is unable to be used to connect to.
    #[inline]
    pub fn startup(&self) -> Result<(ClientManager, OutputReceiver<ClientOutput>)> {
        let (controller, listener) = new_network_interface();
        let (conn, addr) = controller.connect(self.connect.0)?;

        let (sender, receiver) = OutputSender::new();
        let handler = ClientHandler::new(controller.clone(), sender, conn);

        let manager = ClientManager::new(SharedManager {
            _lifetime: listener.start(handler),
            controller,
            addr,
            server_addr: conn.addr(),
        });
        Ok((manager, receiver))
    }
}
