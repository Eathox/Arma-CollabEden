use pretty_assertions::{assert_eq, assert_matches};

use coden::{ClientOutput, InstanceManager, ManagerBuilder, ServerOutput};

mod common;

use common::{recv, setup_test_network, LOCAL_ADDR};

#[test]
fn connect() {
    let (server, server_out) = ManagerBuilder::default()
        .host_on(LOCAL_ADDR)
        .with_ping(None)
        .startup()
        .unwrap();

    let (client, client_out) = ManagerBuilder::default()
        .connect_to(server.addr())
        .with_ping(None)
        .startup()
        .unwrap();

    assert_matches!(
        recv(&server_out),
        ServerOutput::ClientConnected(conn) if conn.addr() == client.addr()
    );
    assert_eq!(recv(&client_out), ClientOutput::ServerConnected(true));
}

#[test]
fn disconnect() {
    let ((server, server_out), mut clients) = setup_test_network(2, false);
    let (_client_b, client_b_out) = clients.pop().unwrap();
    let (client_a, client_a_out) = clients.pop().unwrap();

    client_a.disconnect();
    assert_matches!(
        recv(&server_out),
        ServerOutput::ClientDisconnected(conn) if conn.addr() == client_a.addr()
    );

    server.disconnect();
    assert_matches!(recv(&client_b_out), ClientOutput::ServerDisconnected);

    assert_eq!(server_out.len(), 0);
    assert_eq!(client_a_out.len(), 0);
    assert_eq!(client_b_out.len(), 0);
}

#[test]
fn lost_connection() {
    let ((server, server_out), mut clients) = setup_test_network(2, false);
    let (_client_b, client_b_out) = clients.pop().unwrap();
    let (client_a, client_a_out) = clients.pop().unwrap();

    let addr = client_a.addr();
    drop(client_a);
    assert_matches!(
        recv(&server_out),
        ServerOutput::LostConnection(conn) if conn.addr() == addr
    );

    drop(server);
    assert_matches!(recv(&client_b_out), ClientOutput::LostConnection);

    assert_eq!(server_out.len(), 0);
    assert_eq!(client_a_out.len(), 0);
    assert_eq!(client_b_out.len(), 0);
}
