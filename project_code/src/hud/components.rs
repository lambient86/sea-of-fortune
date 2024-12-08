use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerHPText;

#[derive(Component)]
pub struct ShipHPText;

#[derive(Component)]
pub struct GoldText;

#[derive(Component)]
pub struct WindText;

#[derive(Resource)]
pub struct ShipStats {
    pub hp: f32,
    pub gold: u32,
    pub wind: CardinalDirection,
}

#[derive(Resource)]
pub struct PlayerStats {
    pub hp: f32,
    pub gold: u32,
}

#[derive(Component)]
pub enum CardinalDirection {
    NORTH,
    NORTHWEST,
    WEST,
    SOUTHWEST,
    SOUTH,
    SOUTHEAST,
    EAST,
    NORTHEAST,
}

#[derive(Component)]
pub struct PlayerHUD;
