use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::*;
use std::net::{TcpListener, TcpStream};

/// Struct to represent the TCP connections
#[derive(Resource)]
pub struct TcpConnections {
    pub streams: Vec<TcpStream>,
}

impl TcpConnections {
    /// Adds a connection to the list of TCP connections
    pub fn add_connection(&mut self, stream: TcpStream) {
        self.streams.push(stream);
    }

    /// Handles the tcp connections
    pub fn handle_connections(&mut self) {
        // Iterates through all streams and checks for any new data
        for stream in self.streams.iter_mut() {
            let mut buffer = [0; 1024];

            match stream.read(&mut buffer) {
                Ok(size) => {
                    if size > 0 {
                        //process received data
                        println!("Received data!");
                    }
                }
                Err(e) => {
                    //error encountered
                    println!("Error reading from stream");
                }
            }
        }
    }
}

/// Enumerator that represents different udp packet types
pub enum PacketType {
    PlayerJoin,
    PlayerUpdate,
    EnemyUpdate,
    Unknown,
}

#[derive(Serialize, Deserialize)]
pub struct Packet<T> {
    pub message: String,
    pub payload: T,
}
