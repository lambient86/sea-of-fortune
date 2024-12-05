use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    io::Read,
    net::{TcpStream, UdpSocket},
};

use level::components::*;

use crate::level;

//setting window constants
pub const WIN_W: f32 = 1280.;
pub const WIN_H: f32 = 720.;

pub const WIN_W_CENTER: f32 = WIN_W / 2.;
pub const WIN_H_CENTER: f32 = WIN_H / 2.;

#[derive(Serialize, Deserialize)]
pub struct Packet<T> {
    pub message: String,
    pub payload: T,
}

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

#[derive(Resource)]
pub struct Ocean {
    pub map: Vec<OceanTile>,
}

#[derive(Resource)]
pub struct UDP {
    pub socket: UdpSocket,
}

//setting level constants
pub const TILE_SIZE: u32 = 32;

//REMEMBER TO CHANGE THIS WHEN WE CHANGE MAP SIZE
pub const OCEAN_LEVEL_H: f32 = 32000.;
pub const OCEAN_LEVEL_W: f32 = 32000.;
pub const OCEAN_H_CENTER: f32 = OCEAN_LEVEL_H / 2.;
pub const OCEAN_W_CENTER: f32 = OCEAN_LEVEL_W / 2.;

pub const SAND_LEVEL_H: f32 = 3000.;
pub const SAND_LEVEL_W: f32 = 3000.;
pub const SAND_H_CENTER: f32 = SAND_LEVEL_H / 2.;
pub const SAND_W_CENTER: f32 = SAND_LEVEL_W / 2.;

pub const DUNGEON_LEVEL_H: f32 = 16000.;
pub const DUNGEON_LEVEL_W: f32 = 16000.;
pub const DUNGEON_H_CENTER: f32 = DUNGEON_LEVEL_H / 2.;
pub const DUNGEON_W_CENTER: f32 = DUNGEON_LEVEL_W / 2.;

//for boat (change later, should not have bounds determined
//in different ways for different entities)
pub const BOUNDS: Vec2 = Vec2::new(OCEAN_LEVEL_W, OCEAN_LEVEL_H);
