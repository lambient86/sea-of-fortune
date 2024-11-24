use bevy::prelude::*;

#[derive(Component)]
pub struct OceanTile;

#[derive(Component)]
pub struct SandTile;

// #[derive(Component)]
// pub struct DungeonTile;

#[derive(Resource)]
pub struct OceanTileSheet(pub Handle<Image>, pub Handle<TextureAtlasLayout>);

#[derive(Resource)]
pub struct SandTileSheet(pub Handle<Image>, pub Handle<TextureAtlasLayout>);


