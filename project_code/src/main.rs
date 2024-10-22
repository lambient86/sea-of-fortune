mod player;
mod bat;
mod kraken;
mod boat;
mod data;
mod systems;
mod components;
mod hitbox_system;
mod controls;
mod transition_box;

use controls::*;
use components::GameworldState;
use components::GameState;
use data::gameworld_data::*;
use bevy::{prelude::*, window::PresentMode};
use player::PlayerPlugin;
use boat::BoatPlugin;
use hitbox_system::HitboxPlugin;
use bat::BatPlugin;
use kraken::KrakenPlugin;
use systems::*;
use player::systems::move_player;
use boat::systems::move_boat;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Sea of Fortune | Build 0.2".into(),
                resolution: (WIN_W, WIN_H).into(),
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        .init_resource::<CurrMousePos>()
        .add_systems(Startup, setup_gameworld)
        .add_plugins(PlayerPlugin)
        .add_plugins(BoatPlugin)
        .add_plugins(BatPlugin)
        .add_plugins(KrakenPlugin)
        .add_plugins(HitboxPlugin)
        .add_systems(Update, move_player_camera.after(move_player)
                .run_if(in_state(GameworldState::Island)))
        .add_systems(Update, move_boat_camera.after(move_boat)
                .run_if(in_state(GameworldState::Ocean)))
        .add_systems(Update, change_gameworld_state)
        .add_systems(Update, change_game_state)
        .add_systems(Update, update_mouse_pos)
        .insert_state(GameworldState::MainMenu)
        .insert_state(GameState::Running)
        .run();
}