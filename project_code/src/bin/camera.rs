use bevy::{prelude::*, window::PresentMode};
use std::convert::From;
mod player_movement;
// mod gameworld_info;

//setting window constants
const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

//setting level constants
const TILE_SIZE: f32 = 32.;
const LEVEL_H: f32 = 256.;
const LEVEL_W: f32 = 1440.;

//camera velocity
// impl player_movement::Velocity {
//     fn new() -> Self {
//         Self {
//             //setting x and y velocity to 0
//             velocity: Vec2::splat(0.)
//         }
//     }
// }


impl From<Vec2> for player_movement::Velocity {
    fn from(velocity: Vec2) -> Self {
        Self { velocity }
    }
}

pub fn move_camera(
    player: Query<&Transform, With<player_movement::Player>>,
    mut camera: Query<&mut Transform, (Without<player_movement::Player>, With<Camera>)>,
) {
    let pt = player.single();
    let mut ct = camera.single_mut();

    //getting bounds for x and y
    // let x_bound = 
}