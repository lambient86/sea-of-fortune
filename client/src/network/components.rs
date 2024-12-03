use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::net::UdpSocket;

#[derive(Serialize, Deserialize)]
pub struct Packet<T> {
    pub message: String,
    pub payload: T,
}