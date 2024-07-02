#![allow(dead_code)]

use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::Duration,
};

use pretty_assertions::{assert_eq, assert_matches};

use coden::{
    ClientManager, ClientOutput, InstanceManager, ManagerBuilder, OutputReceiver, ServerManager,
    ServerOutput,
};

pub const TEST_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0);

#[allow(clippy::type_complexity)]
pub fn setup_test_network(
    client_count: usize,
    allow_ping: bool,
) -> (
    (ServerManager, OutputReceiver<ServerOutput>),
    Vec<(ClientManager, OutputReceiver<ClientOutput>)>,
) {
    let mut server_builder = ManagerBuilder::default().host_on(TEST_ADDR);
    if !allow_ping {
        server_builder = server_builder.with_ping(None);
    }

    let (server, server_out) = server_builder.startup().unwrap();

    let mut clients = Vec::with_capacity(client_count);
    let mut client_builder = ManagerBuilder::default().connect_to(server.server_addr());
    if !allow_ping {
        client_builder = client_builder.with_ping(None);
    }

    for _ in 0..client_count {
        let (client, client_out) = client_builder.startup().unwrap();
        assert_matches!(
            recv(&server_out),
            ServerOutput::ClientConnected(conn) if conn.addr() == client.addr()
        );
        assert_eq!(recv(&client_out), ClientOutput::Connected);

        clients.push((client, client_out));
    }

    ((server, server_out), clients)
}

// Receive but with a long default timeout to avoid hanging tests
pub fn recv<O>(out: &OutputReceiver<O>) -> O {
    out.recv_timeout(Duration::from_secs(6)).unwrap()
}
