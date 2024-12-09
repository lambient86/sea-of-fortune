use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerHPText;

#[derive(Component)]
pub struct ShipHPText;

#[derive(Component)]
pub struct GoldText;

#[derive(Component)]
pub struct Arrow;

#[derive(Resource)]
pub struct ShipStats {
    pub hp: f32,
    pub gold: u32,
    pub wind_dir: Vec2,
}

#[derive(Resource)]
pub struct ArrowTS(pub Handle<Image>);

#[derive(Resource)]
pub struct PlayerStats {
    pub hp: f32,
    pub gold: u32,
}

#[derive(Component)]
pub struct PlayerHUD;

#[derive(Component)]
pub struct ShipHUD;

#[derive(Component)]
pub struct WindText;
