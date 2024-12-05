use bevy::prelude::*;
use serde::Serialize;

//struct that holds all of the information for an ocean tile
#[derive(Component, Serialize)]
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

///struct that holds the ocean map as a resource
#[derive(Resource)]
pub struct OceanMap {
    pub map: Vec<OceanTile>,
}
