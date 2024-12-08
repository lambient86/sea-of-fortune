use bevy::prelude::*;

//setting window constants
pub const WIN_W: f32 = 1280.;
pub const WIN_H: f32 = 720.;

pub const WIN_W_CENTER: f32 = WIN_W / 2.;
pub const WIN_H_CENTER: f32 = WIN_H / 2.;

//setting level constants
pub const TILE_SIZE: u32 = 32;

//REMEMBER TO CHANGE THIS WHEN WE CHANGE MAP SIZE
pub const OCEAN_LEVEL_H: f32 = 4000.;
pub const OCEAN_LEVEL_W: f32 = 4000.;
pub const OCEAN_H_CENTER: f32 = OCEAN_LEVEL_H / 2.;
pub const OCEAN_W_CENTER: f32 = OCEAN_LEVEL_W / 2.;

pub const SAND_LEVEL_H: f32 = 3000.;
pub const SAND_LEVEL_W: f32 = 3000.;
pub const SAND_H_CENTER: f32 = SAND_LEVEL_H / 2.;
pub const SAND_W_CENTER: f32 = SAND_LEVEL_W / 2.;

pub const DUNGEON_LEVEL_H: f32 = 16000.;
pub const DUNGEON_LEVEL_W: f32 = 16000.;
pub const DUNGEON_H_CENTER: f32 = DUNGEON_LEVEL_H / 2.;
pub const DUNGEON_W_CENTER: f32 = DUNGEON_LEVEL_W / 2.;

//for boat (change later, should not have bounds determined
//in different ways for different entities)
pub const BOUNDS: Vec2 = Vec2::new(OCEAN_LEVEL_W, OCEAN_LEVEL_H);
