use crate::components::BoundingBox;
use bevy::prelude::*;
use serde::*;

pub const OCEAN_LENGTH: i32 = 15625;

#[derive(Component)]
pub struct OceanTile;

#[derive(Component, Serialize, Deserialize)]
pub struct OceanT {
    pub translation: Vec3,
    pub tile_index: usize,
}

#[derive(Resource)]
pub struct Ocean {
    pub map: Vec<OceanT>,
}

#[derive(Component)]
pub struct SandTile;

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum IslandType {
    Start,
    Level1,
    Level2,
    Level3,
    Boss,
}

#[derive(Component)]
pub struct Island {
    pub aabb: BoundingBox,
    pub island_type: IslandType,
}

#[derive(Component)]
pub struct Dungeon {
    pub aabb: BoundingBox,
    pub dungeon_type: IslandType,
    pub size: Vec2,
}

#[derive(Component)]
pub struct OceanDoor {
    pub aabb: BoundingBox,
}

#[derive(Resource)]
pub struct OceanTileSheet(pub Handle<Image>, pub Handle<TextureAtlasLayout>);

#[derive(Resource)]
pub struct SandTileSheet(pub Handle<Image>, pub Handle<TextureAtlasLayout>);

#[derive(Resource)]
pub struct IslandTileSheet(pub Handle<Image>);

#[derive(Resource)]
pub struct DungeonSheet(
    pub Handle<Image>,
    pub Handle<Image>,
    pub Handle<Image>,
    pub Handle<Image>,
);

#[derive(Resource)]
pub struct OceanDoorHandle(pub Handle<Image>);
