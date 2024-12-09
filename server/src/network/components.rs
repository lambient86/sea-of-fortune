use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::*;
use std::net::*;
use std::sync::{Arc, Mutex};

#[derive(Resource)]
pub struct UDP {
    pub socket: UdpSocket,
}

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
        Counter { count: 5 }
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

#[derive(Resource, Serialize, Deserialize, Clone)]
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

#[derive(Resource, Clone, Serialize, Deserialize)]
pub struct Projectiles {
    pub list: Vec<Projectile>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Projectile {
    pub owner_id: i32,
    pub velocity: Velocity,
    pub translation: Vec3,
    pub lifetime: f32,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Velocity {
    pub v: Vec2,
}

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct EnemyLists {
    pub new: Enemies,
    pub update: Enemies,
    pub dead: Enemies,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Enemies {
    pub list: Vec<Enemy>,
}

impl Enemies {
    pub fn init() -> Enemies {
        Enemies { list: Vec::new() }
    }
}

#[derive(Serialize, Deserialize, Clone)]

pub struct Enemy {
    pub id: i32,
    pub etype: i32,
    pub pos: Vec3,
    pub animation_index: usize,
    pub hp: f32,
    pub alive: bool,
    pub target_id: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Damage {
    pub target_id: i32,
    pub dmg: f32,
}

#[derive(Resource, Clone)]
pub struct Cooldowns {
    pub list: Vec<CD>,
}

#[derive(Clone)]
pub struct CD {
    pub enemy_id: i32,
    pub og: f32,
    pub timer: Timer,
}
