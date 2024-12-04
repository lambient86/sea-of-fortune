mod level;
mod network;

use bevy::prelude::*;
use std::net::{TcpStream, UdpSocket};

use crate::level::components::*;
use crate::level::systems::*;
use crate::network::systems::*;

fn main() {
    println!("Starting Client");

    //connect to server
    let udp_addr = "127.0.0.1:4000";
    let tcp_addr = "127.0.0.1:8000";

    let udp_socket = UdpSocket::bind(udp_addr).unwrap();

    println!(
        "UDP: Client listening on {}",
        udp_socket.local_addr().unwrap()
    );

    let mut buf = [0; 1024];

    //starting tcp connection with server
    let mut tcp_stream = TcpStream::connect(tcp_addr);

    loop {
        match tcp_stream {
            Ok(ref t) => {
                println!("TCP: Stream connected!");
                break;
            }
            Err(ref e) => {
                eprintln!("Something happened: {}", e);
                tcp_stream = TcpStream::connect(tcp_addr);
            }
        }
    }

    App::new();
    //.add_systems(Startup, listener);

    /*loop {
        let result = socket.recv_from(&mut buf);
        match result {
            Ok((size, src)) => {
                println!("Recieved {} bytes from {}", size, src);

                let json_str = String::from_utf8_lossy(&buf[..size]);
                println!("Received JSON packet: {}", json_str);

                let deserialize: Packet<OceanTile> = serde_json::from_slice(&buf[..size]).unwrap();

                ocean_map.push(deserialize.payload);

                if ocean_map.len() >= 100000 {
                    break;
                }

                //let result = socket.send_to(&buf[..size], "127.0.0.1:8000");
            }
            Err(e) => {
                eprintln!("Something happened: {}", e);
            }
        }
    }*/
}
