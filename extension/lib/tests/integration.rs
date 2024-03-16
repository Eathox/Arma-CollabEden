use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::Duration,
};

use crossbeam_channel::Receiver;
use pretty_assertions::{assert_eq, assert_matches};

use coden::{ClientOutput, InstanceManager, ManagerBuilder, ServerOutput};

const LOCAL_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0);
const TIMEOUT: Duration = Duration::from_millis(10);

fn recv_output<O>(output: &Receiver<O>) -> O {
    output.recv_timeout(TIMEOUT).unwrap()
}

#[test]
fn disconnect() {
    let (server, server_output) = ManagerBuilder::new()
        .without_ping()
        .host_on(LOCAL_ADDR)
        .startup()
        .unwrap();
    let (client, client_output) = ManagerBuilder::new()
        .without_ping()
        .connect_to(server.server_addr())
        .startup()
        .unwrap();

    assert_matches!(
        recv_output(&server_output),
        ServerOutput::ClientConnected(conn) if conn.addr() == client.addr()
    );
    assert_eq!(recv_output(&client_output), ClientOutput::Connected(true));

    client.disconnect();

    assert_matches!(
        recv_output(&server_output),
        ServerOutput::ClientDisconnected(_)
    );
    assert_eq!(client_output.len(), 0);
}

#[test]
fn server_disconnect() {
    let (server, server_output) = ManagerBuilder::new()
        .without_ping()
        .host_on(LOCAL_ADDR)
        .startup()
        .unwrap();
    let (client, client_output) = ManagerBuilder::new()
        .without_ping()
        .connect_to(server.server_addr())
        .startup()
        .unwrap();

    assert_matches!(
        recv_output(&server_output),
        ServerOutput::ClientConnected(conn) if conn.addr() == client.addr()
    );
    assert_eq!(recv_output(&client_output), ClientOutput::Connected(true));

    server.disconnect();

    assert_matches!(
        recv_output(&client_output),
        ClientOutput::ServerDisconnected
    );
    assert_eq!(client_output.len(), 0);
}
