use crate::components::*;
use crate::data::gameworld_data::*;
use crate::hitbox_system::{Hitbox, Hurtbox};
use crate::level::components::{Dungeon, IslandType, OceanDoor};
use crate::player::components::*;
use crate::{boat::components::*, level::components::Island};
use bevy::math::bounding::IntersectsVolume;
use bevy::{prelude::*, window::PresentMode};
use bevy::math::bounding::BoundingVolume;

use crate::wfc::components::Wall;
use crate::bat::components::Bat;
use crate::skeleton::components::Skeleton;
use crate::player::components::Player; 
use crate::rock::components::Rock;

use crate::components::*;

/*   MOVE_CAMERA FUNCTIONS  */
/// Updates the cameras position to center the current player
/// and tracks the player wherever they go
pub fn move_player_camera(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (Without<Player>, With<Camera>)>,
    gameworld_state: Res<State<GameworldState>>,
) {
    let pt = player.single();
    let mut ct = camera.single_mut();

    match gameworld_state.get() {
        GameworldState::Island => {
            let x_bound = SAND_LEVEL_W / 2. - WIN_W / 2.;
            let y_bound = SAND_LEVEL_H / 2. - WIN_H / 2.;
            ct.translation.x = pt.translation.x.clamp(-x_bound, x_bound);
            ct.translation.y = pt.translation.y.clamp(-y_bound, y_bound);
        }
        GameworldState::Dungeon => {
            let x_bound = DUNGEON_LEVEL_W / 2. - WIN_W / 2.;
            let y_bound = DUNGEON_LEVEL_H / 2. - WIN_H / 2.;
            ct.translation.x = pt.translation.x.clamp(-x_bound, x_bound);
            ct.translation.y = pt.translation.y.clamp(-y_bound, y_bound);
        }
        _ => {}
    }
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

pub fn handle_transition_immunity(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut TransitionImmunity), With<Player>>,
) {
    for (entity, mut immunity) in query.iter_mut() {
        immunity.timer.tick(time.delta());
        if immunity.timer.finished() {
            commands.entity(entity).remove::<TransitionImmunity>();
        }
    }
}

pub fn change_gameworld_state(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameworldState>>,
    mut current_island_type: ResMut<CurrentIslandType>,
    islands_query: Query<&mut Island, With<Island>>,
    dungeon_query: Query<&mut Dungeon, With<Dungeon>>,
    gameworld_state: Res<State<GameworldState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Player, With<Player>>,
    mut boat_query: Query<&mut Boat, With<Boat>>,
    door_query: Query<&mut OceanDoor, With<OceanDoor>>,
    query: Query<(Entity, &Transform), (With<Player>, Without<TransitionImmunity>)>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter)
            && *gameworld_state.get() != GameworldState::Ocean
            && *gameworld_state.get() != GameworldState::Island
            && *gameworld_state.get() != GameworldState::Dungeon
        {
            current_island_type.island_type = IslandType::Start;
            next_state.set(GameworldState::Island);
            return;
        }
    for (entity, transform) in query.iter() {
        
        //  CASE: OCEAN --> ISLAND
        if *gameworld_state.get() == GameworldState::Ocean {
            let boat = boat_query.single_mut();
            for island in islands_query.iter() {
                if island.aabb.aabb.intersects(&boat.aabb.aabb) {
                    current_island_type.island_type = island.island_type;
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
        commands.entity(entity).insert(TransitionImmunity {
            timer: Timer::from_seconds(0.5, TimerMode::Once),
        });
    }
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

pub fn check_wall_collisions(
    mut entities_query: Query<(&mut Transform, &mut Velocity, &Hurtbox), Or<(With<Player>, With<Bat>, With<Skeleton>, With<Rock>)>>,
    walls_query: Query<&BoundingBox, With<Wall>>,
) {
    for (mut transform, mut velocity, hurtbox) in entities_query.iter_mut() {
        let entity_aabb = BoundingBox::new(
            transform.translation.truncate(),
            hurtbox.size
        ).aabb;

        for wall_box in walls_query.iter() {
            if entity_aabb.intersects(&wall_box.aabb) {
                // Stop movement in collision direction
                let overlap = entity_aabb.center() - wall_box.aabb.center();
                let push_direction = overlap.normalize();
                
                transform.translation += Vec3::new(push_direction.x, push_direction.y, 0.0) * 2.0;
                
                // Zero out velocity in collision direction
                if overlap.x.abs() > overlap.y.abs() {
                    velocity.v.x = 0.0;
                } else {
                    velocity.v.y = 0.0;
                }
            }
        }
    }
}

pub fn handle_dungeon_entry(
    mut player_query: Query<&mut Transform, With<Player>>,
    gameworld_state: Res<State<GameworldState>>,
) {
    if *gameworld_state.get() == GameworldState::Dungeon {
        if let Ok(mut transform) = player_query.get_single_mut() {
            transform.translation = Vec3::new(-2976.0, -2976.0, 0.0);
            println!("Translated player to dungeon spawn: {:?}", transform.translation);
        }
    }
}

pub fn handle_door_translation(
    mut door_query: Query<&mut Transform, Or<(With<Dungeon>, With<OceanDoor>)>>,
    gameworld_state: Res<State<GameworldState>>,
) {
    if let Ok(mut transform) = door_query.get_single_mut() {
        match *gameworld_state.get() {
            GameworldState::Dungeon => {
                transform.translation = Vec3::new(2976.0, 2976.0, 10.0);
                println!("Translated door to dungeon position");
            },
            GameworldState::Island => {
                transform.translation = Vec3::new(0.0, 256.0, 10.0);
                println!("Translated door to island position");
            },
            _ => {}
        }
    }
}

pub fn update_dungeon_collision(
    mut dungeon_query: Query<(&Transform, &mut Dungeon)>,
) {
    for (transform, mut dungeon) in dungeon_query.iter_mut() {
        dungeon.aabb = BoundingBox::new(
            transform.translation.truncate(),
            dungeon.size
        );
    }
}
