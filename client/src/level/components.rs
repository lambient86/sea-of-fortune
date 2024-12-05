use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize)]
pub struct OceanTile {
    pub translation: Vec3,
    pub tile_index: usize,
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
