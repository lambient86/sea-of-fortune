mod bat;
mod boat;
mod components;
mod controls;
mod data;
mod enemies;
mod hitbox_system;
mod kraken;
mod level;
mod player;
mod shop;
mod systems;
mod transition_box;

use bat::BatPlugin;
use bevy::{prelude::*, window::PresentMode};
use boat::systems::move_boat;
use boat::BoatPlugin;
use components::GameState;
use components::GameworldState;
use controls::*;
use data::gameworld_data::*;
use enemies::*;
use hitbox_system::HitboxPlugin;
use kraken::KrakenPlugin;
use level::LevelPlugin;
use player::systems::move_player;
use player::PlayerPlugin;
use shop::ShopPlugin;
use systems::*;

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
        .add_plugins(ShopPlugin)
        .add_plugins(LevelPlugin)
        .add_systems(
            Update,
            move_player_camera.after(move_player).run_if(
                in_state(GameworldState::Island).or_else(in_state(GameworldState::Dungeon)),
            ),
        )
        .add_systems(
            Update,
            move_boat_camera
                .after(move_boat)
                .run_if(in_state(GameworldState::Ocean)),
        )
        .add_systems(Update, change_gameworld_state)
        .add_systems(Update, change_game_state)
        .add_systems(Update, update_mouse_pos)
        .insert_state(GameworldState::MainMenu)
        .insert_state(GameState::Running)
        .run();
}