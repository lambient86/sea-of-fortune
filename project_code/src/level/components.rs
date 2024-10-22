use bevy::prelude::*;

#[derive(Component)]
pub struct BGTile;

#[derive(Component)]
pub struct Background;

#[derive(Resource)]
pub struct BGTileSheet(pub Handle<Image>, pub Handle<TextureAtlasLayout>);