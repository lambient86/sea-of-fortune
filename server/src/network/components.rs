use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::*;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

/// Struct to represent the TCP connections
#[derive(Resource)]
pub struct TcpConnections {
    pub streams: Vec<TcpStream>,
}

#[derive(Resource)]
pub struct TcpResource {
    pub streams: Arc<Mutex<TcpConnections>>,
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

#[derive(Serialize, Deserialize)]
pub struct Envelope {
    pub message: String,
    pub packet: String,
}

#[derive(Serialize, Deserialize)]
pub struct Packet<T> {
    pub payload: T,
}

#[derive(Resource)]
pub struct Counter {
    pub count: i32,
}

impl Counter {
    pub fn init() -> Counter {
        Counter { count: 0 }
    }

    pub fn next(&mut self) -> i32 {
        self.count += 1;
        self.count
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: i32,
    pub addr: String,
    pub pos: Vec3,
    pub rot: Quat,
    pub boat: bool,
    pub used: bool,
}

impl Player {
    pub fn default() -> Player {
        Player {
            id: -1,
            addr: "null".to_string(),
            pos: Vec3::splat(0.),
            rot: Quat::from_rotation_x((90.0_f32).to_radians()),
            boat: true,
            used: false,
        }
    }
}

#[derive(Resource)]
pub struct Players {
    pub player_array: [Player; 4],
}

impl Players {
    pub fn init() -> Players {
        Players {
            player_array: [
                Player::default(),
                Player::default(),
                Player::default(),
                Player::default(),
            ],
        }
    }
}

#[derive(Resource)]
pub struct Projectiles {
    pub list: Vec<Projectile>,
}

pub struct Projectile {
    pub velocity: Velocity,
    pub tranform: Transform,
}

pub struct Velocity {
    pub v: Vec2,
}

#[derive(Resource)]
pub struct Enemies {
    pub list: Vec<Enemy>,
}

impl Enemies {
    pub fn init() -> Enemies {
        Enemies { list: Vec::new() }
    }
}

#[derive(Serialize, Deserialize)]
pub enum EType {
    Bat,
    Kraken,
    GhostShip,
    Rock,
    RSkeleton,
    MSkeleton,
}

#[derive(Serialize, Deserialize)]

pub struct Enemy {
    pub id: i32,
    pub etype: EType,
    pub translation: Vec3,
    pub animation_index: usize,
    pub alive: bool,
}
