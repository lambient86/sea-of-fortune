use crate::components::*;
use crate::data::gameworld_data::*;
use crate::hitbox_system::{Hitbox, Hurtbox};
use crate::level::components::{Dungeon, IslandType, OceanDoor};
use crate::player::components::*;
use crate::{boat::components::*, level::components::Island};
use bevy::math::bounding::IntersectsVolume;
use bevy::{prelude::*, window::PresentMode};

/*   MOVE_CAMERA FUNCTIONS  */
/// Updates the cameras position to center the current player
/// and tracks the player wherever they go
pub fn move_player_camera(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let pt = player.single();
    let mut ct = camera.single_mut();

    let x_bound = SAND_LEVEL_W / 2. - WIN_W / 2.;
    let y_bound = SAND_LEVEL_H / 2. - WIN_H / 2.;
    ct.translation.x = pt.translation.x.clamp(-x_bound, x_bound);
    ct.translation.y = pt.translation.y.clamp(-y_bound, y_bound);
}

/// Updates the cameras position to the center of the current
/// players boats and track it wherever they go
pub fn move_boat_camera(
    boat: Query<&Transform, With<Boat>>,
    mut camera: Query<&mut Transform, (Without<Boat>, With<Camera>)>,
) {
    let bt = boat.single();
    let mut ct = camera.single_mut();

    let x_bound = OCEAN_LEVEL_W / 2. - WIN_W / 2.;
    let y_bound = OCEAN_LEVEL_H / 2. - WIN_H / 2.;
    ct.translation.x = bt.translation.x.clamp(-x_bound, x_bound);
    ct.translation.y = bt.translation.y.clamp(-y_bound, y_bound);
}

/*   SETUP_GAMEWORLD FUCNTION   */
/// Sets up the gameworld
pub fn setup_gameworld(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    //getting background texture
    let bg_texture_handle = asset_server.load("bg_main_menu.png");

    //spawning new background sprite
    commands
        .spawn(SpriteBundle {
            texture: bg_texture_handle.clone(),
            transform: Transform::from_xyz(0., 0., -1.),
            ..default()
        })
        .insert(Background);
}

pub fn change_gameworld_state(
    mut next_state: ResMut<NextState<GameworldState>>,
    islands_query: Query<&mut Island, With<Island>>,
    dungeon_query: Query<&mut Dungeon, With<Dungeon>>,
    gameworld_state: Res<State<GameworldState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Player, With<Player>>,
    mut boat_query: Query<&mut Boat, With<Boat>>,
    door_query: Query<&mut OceanDoor, With<OceanDoor>>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter)
        && *gameworld_state.get() != GameworldState::Ocean
        && *gameworld_state.get() != GameworldState::Island
        && *gameworld_state.get() != GameworldState::Dungeon
    {
        next_state.set(GameworldState::Ocean);
    }
    //  CASE: OCEAN --> ISLAND
    if *gameworld_state.get() == GameworldState::Ocean {
        let boat = boat_query.single_mut();
        for island in islands_query.iter() {
            if island.aabb.aabb.intersects(&boat.aabb.aabb) {
                println!("going to the island!");
                next_state.set(GameworldState::Island);
            }
        }
    } else if *gameworld_state.get() == GameworldState::Island {
        let player = player_query.single_mut();

        // case: island --> dungeon
        for dungeon in dungeon_query.iter() {
            if dungeon.aabb.aabb.intersects(&player.aabb.aabb) {
                println!("going to a dungeon!");
                next_state.set(GameworldState::Dungeon);
            }
        }

        // case: island --> ocean
        let ocean_door = door_query.single();
        if ocean_door.aabb.aabb.intersects(&player.aabb.aabb) {
            println!("going to the ocean!");
            next_state.set(GameworldState::Ocean);
        }
    } else if *gameworld_state.get() == GameworldState::Dungeon {
        let player = player_query.single_mut();
        for dungeon in dungeon_query.iter() {
            if dungeon.aabb.aabb.intersects(&player.aabb.aabb) {
                println!("going to a dungeon!");
                next_state.set(GameworldState::Island);
            }
        }
    }

    // for (entity, island_type) in islands_query.iter() {
    //     match island_type {
    //         IslandType::Level1 => {
    //             println!("Found Level 1 Island");
    //         }
    //         IslandType::Level2 => {
    //             println!("Found Level 2 Island");
    //         }
    //         IslandType::Level3 => {
    //             println!("Found Level 3 Island");
    //         }
    //         IslandType::Boss => {
    //             println!("Found Boss Island");
    //         }
    //     }
    // }
}

/*   CHANGE_GAME_STATE FUNCTION   */
/// Changes the state of the game. Such as a switch between running and paused
/// DEBUG: On keypress, the game state will switch
/// * E - if Running, to InShop, if InShop, to Running
pub fn change_game_state(
    game_state: Res<State<GameState>>,
    gameworld_state: Res<State<GameworldState>>,
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if *game_state.get() == GameState::Running
        && (*gameworld_state.get() == GameworldState::Island
            || *gameworld_state.get() == GameworldState::Dungeon)
        && keyboard_input.just_pressed(KeyCode::KeyE)
    {
        next_state.set(GameState::InShop)
    } else if *game_state.get() == GameState::InShop && keyboard_input.just_pressed(KeyCode::KeyE) {
        next_state.set(GameState::Running)
    }
}
