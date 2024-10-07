mod player;
mod bat;
mod boat;
mod data;
mod systems;
mod components;

use data::gameworld_data::*;
use bevy::{prelude::*, window::PresentMode};
use player::PlayerPlugin;
use systems::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Work in progress".into(),
                resolution: (WIN_W, WIN_H).into(),
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_gameworld)
        .add_plugins(PlayerPlugin)
        .add_systems(Update, move_camera)
        .run();
}
