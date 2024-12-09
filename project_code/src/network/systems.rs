use crate::network::components::*;

use bevy::prelude::*;
use bevy::window::PresentMode;
use core::panic;
use rand::Rng;
use serde::*;
use std::net::*;
use std::sync::{Arc, Mutex};

pub fn create_env<T: Serialize>(message: String, object: T) -> String {
    let packet: Packet<T> = Packet { payload: object };

    serde_json::to_string(&Envelope {
        message: message,
        packet: serde_json::to_string(&packet).unwrap(),
    })
    .unwrap()
}
