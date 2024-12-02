use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::net::UdpSocket;

#[derive(Serialize, Deserialize)]
pub struct Packet<T> {
    pub message: String,
    pub payload: T,
}

#[derive(Component, Serialize, Deserialize)]
pub struct OceanTile {
    translation: Vec3,
    tile_index: usize,
}

/// implementation for ocean tile
impl OceanTile {
    // constructor for ocean tile
    pub fn new(t: Vec3, ti: usize) -> Self {
        OceanTile {
            translation: t,
            tile_index: ti,
        }
    }
}

fn main() {
    println!("Starting Client");

    //connect to server
    let socket = UdpSocket::bind("127.0.0.1:4000").unwrap();

    println!("Client listening on {}", socket.local_addr().unwrap());

    let server = "127.0.0.1:4000";

    let mut buf = [0; 1024];

    let mut ocean_map: Vec<OceanTile> = Vec::new();

    loop {
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
    }

    App::new();
    //.add_systems(Startup, listener);
}
