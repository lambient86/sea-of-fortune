use bevy::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::io::*;

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
            let mut buffer = [0u8; 1024];

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

