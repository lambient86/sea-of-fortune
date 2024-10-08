mod player;
mod bat;
mod boat;
mod data;
mod systems;
mod components;
mod hitbox_system;
mod controls;

use data::gameworld_data::*;
use bevy::{prelude::*, window::PresentMode};
use player::PlayerPlugin;
use hitbox_system::HitboxPlugin;
use bat::BatPlugin;
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
        .add_plugins(HitboxPlugin)
        .add_plugins(BatPlugin)
        .add_systems(Update, move_camera)
        .run();
}