use bevy::prelude::*;
use std::io::{Read, Write};
use std::net::*;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::network::components::*;

/*   START_TCP_SERVER FUNCTION   */
///Function that handles TCP connections seperately
pub fn start_tcp_server(mut connections: Arc<Mutex<TcpConnections>>) {
    //spawning thread to handle connections
    thread::spawn(move || {
        let tcp_listener = TcpListener::bind("127.0.0.1:80").unwrap();
        println!("Server listening on 127.0.0.1:80");

        //Accepting incoming connection
        for stream in tcp_listener.incoming() {
            match stream {
                //checking it tcp connection was successfully established
                Ok(stream) => {
                    println!(
                        "Establishing a new TCP connection from {:?}",
                        stream.peer_addr()
                    );
                    connections.add_connection(stream);
                }
                Err(e) => {
                    println!("Failed to accept connection: {:?}", e);
                }
            }
        }
    });
}

/*   RECEIVE_UDP FUCNTION   */
/// Receives and handles UDP packets
pub fn receive_udp(socket: UdpSocket) {}

/*   HANDLE_TCP_CONNECTIONS FUNCTION   */
/// System to periodically check and handle active tcp connections
fn handle_connections(mut tcpconnections: ResMut<TcpConnections>) {
    tcpconnections.handle_connections();
}
