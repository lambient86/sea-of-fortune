mod data;
mod level;
mod network;

use bevy::prelude::*;
use level::components::*;
use std::net::{TcpListener, TcpStream, UdpSocket};
use std::sync::{Arc, Mutex};

use crate::level::systems::*;
use crate::network::components::*;
use crate::network::systems::*;

fn main() {
    println!("Starting Server");

    // Creating UDP socket connecion
    let udp_socket = UdpSocket::bind("127.0.0.1:8000").unwrap();

    // Creating ocean level
    let mut ocean_map = build_ocean();

    let tcpconnections = TcpConnections {
        streams: Vec::new(),
    };

    //creating a shared and thread safe TcpConnections resource
    let connections = Arc::new(Mutex::new(tcpconnections));

    //starting tcp server in seperate thread
    start_tcp_server(connections);

    App::new();

    let mut size = ocean_map.len();

    for tile in ocean_map {
        let packet = Packet {
            message: String::from("load_ocean"),
            payload: &tile,
        };

        let serialized = serde_json::to_string(&packet);

        let result = udp_socket.send_to(&serialized.unwrap().as_bytes(), "127.0.0.1:4000");

        //println!("{}", size);
        //size -= 1;
    }

    //println!("Done")
}
