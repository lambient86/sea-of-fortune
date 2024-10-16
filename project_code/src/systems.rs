use crate::boat::components::*;
use crate::components::*;
use crate::data::gameworld_data::*;
use crate::player::components::*;
use bevy::{prelude::*, window::PresentMode};

/*   MOVE_CAMERA_ FUNCTIONS  */
/// Updates the cameras position to center the current player
/// and tracks the player wherever they go
pub fn move_player_camera(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let pt = player.single();
    let mut ct = camera.single_mut();

    let x_bound = LEVEL_W / 2. - WIN_W / 2.;
    let y_bound = LEVEL_H / 2. - WIN_H / 2.;
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

    let x_bound = LEVEL_W / 2. - WIN_W / 2.;
    let y_bound = LEVEL_H / 2. - WIN_H / 2.;
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

/*   CHANGE_GAMEWORLD_STATE FUNCTION   */
/// Changes the state of the gameworld
/// DEBUG: On keypress, the gameworld will switch
/// * I - Island
/// * O - Ocean
/// * U - Dungeon
pub fn change_gameworld_state(
    state: Res<State<GameworldState>>,
    mut next_state: ResMut<NextState<GameworldState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyI) {
        //ISLAND
        //getting background texture
        let bg_texture_handle = asset_server.load("bg_sand_demo.png");

        //spawning background sprite
        commands
            .spawn(SpriteBundle {
                texture: bg_texture_handle.clone(),
                transform: Transform::from_xyz(0., 0., -1.),
                ..default()
            })
            .insert(Background);

        //switching states to island
        next_state.set(GameworldState::Island);
    } else if keyboard_input.just_pressed(KeyCode::KeyO) {
        //OCEAN
        //getting background texture
        let bg_texture_handle = asset_server.load("bg_ocean_demo.png");

        //spawning background sprite
        commands
            .spawn(SpriteBundle {
                texture: bg_texture_handle.clone(),
                transform: Transform::from_xyz(0., 0., -1.),
                ..default()
            })
            .insert(Background);

        //switching state to ocean
        next_state.set(GameworldState::Ocean);
    } else if keyboard_input.just_pressed(KeyCode::KeyU) {
        //DUNGEON
        //switching state to dungeon
        next_state.set(GameworldState::Dungeon)
    }
}

/*   CHANGE_GAME_STATE FUNCTION   */
/// Changes the state of the game. Such as a switch between running and paused
/// DEBUG: On keypress, the game state will switch
/// * E - if Running, to InShop, if InShop, to Running
pub fn change_game_state(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if *state.get() == GameState::Running && keyboard_input.just_pressed(KeyCode::KeyE) {
        next_state.set(GameState::InShop)
    } else if *state.get() == GameState::InShop && keyboard_input.just_pressed(KeyCode::KeyE) {
        next_state.set(GameState::Running)
    }
}
