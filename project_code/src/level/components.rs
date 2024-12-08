use crate::components::BoundingBox;
use bevy::prelude::*;

#[derive(Component)]
pub struct OceanTile;

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
