mod level;
mod data;
mod network;

use std::net::{UdpSocket, TcpListener, TcpStream};
use bevy::prelude::*;
use std::sync::{Arc, Mutex};

use crate::level::systems::*;
use crate::network::systems::*;
use crate::network::components::*;


fn main() {
    println!("Starting Server");

    // Creating UDP socket connecion
    let udp_socket = UdpSocket::bind("127.0.0.1:4000").unwrap();

    // Creating ocean level
    let ocean_map = build_ocean();

    //creating a shared and thread safe TcpConnections resource
    let connections = Arc::new(Mutex::new(
        TcpConnections {
            streams: Vec::new(),
        }
    ));

    //starting tcp server in seperate thread
    start_tcp_server(connections.clone());

    App::new()
        .insert_resource(connections.clone());
}