use bevy::prelude::*;

//struct that holds all of the information for an ocean tile
#[derive(Component)]
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