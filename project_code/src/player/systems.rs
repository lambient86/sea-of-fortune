use std::ops::Bound;

use crate::components::*;
use crate::components::{BoundingBox, GameworldState, SpawnLocations};
use crate::controls::*;
use crate::data::gameworld_data::*;
use crate::hitbox_system::*;
use crate::network::components::HostPlayer;
use crate::player::components::*;

use crate::shop::components::{Inventory, Item, ItemType};
use crate::shop::systems::generate_loot_item;

use bevy::input::mouse::{self, MouseButtonInput};
use bevy::prelude::*;

/*   MOVE_PLAYER FUNCTION */
/// Moves the player, updating its position depending on
/// button pressed and players current velocity
pub fn move_player(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut player: Query<(&mut Transform, &mut Velocity, &mut Player), With<Player>>,
    gameworld_state: Res<State<GameworldState>>,
) {
    let (mut player_transform, mut player_velocity, mut player) = player.single_mut();

    let mut deltav = Vec2::splat(0.);

    //checking left/right input and deciding movement
    //if left pressed and right not : 0 - 1 = -1
    deltav.x = get_player_input(PlayerControl::Right, &keyboard_input, &mouse_input)
        - get_player_input(PlayerControl::Left, &keyboard_input, &mouse_input);

    //checking up/down input and deciding movement
    //if up pressed and down not; 0 - 1 = -1
    deltav.y = get_player_input(PlayerControl::Up, &keyboard_input, &mouse_input)
        - get_player_input(PlayerControl::Down, &keyboard_input, &mouse_input);

    //getting acceleration
    let delta_t = time.delta_seconds();
    let acc = PLAYER_ACCELERATION * delta_t;

    //deciding player velocity
    player_velocity.v = if deltav.length() > 0. {
        (player_velocity.v + (deltav.normalize_or_zero() * acc)).clamp_length_max(PLAYER_SPEED)
    } else if player_velocity.v.length() > acc {
        player_velocity.v + (player_velocity.v.normalize_or_zero() * -acc)
    } else {
        Vec2::splat(0.)
    };

    //getting change in location
    let change = player_velocity.v * delta_t;
    let mut change_vec = Vec2::splat(0.);

    // Get bounds based on gameworld state
    let (level_w, level_h) = match gameworld_state.get() {
        GameworldState::Island => (SAND_LEVEL_W, SAND_LEVEL_H),
        GameworldState::Dungeon => (DUNGEON_LEVEL_W, DUNGEON_LEVEL_H),
        _ => return,
    };

    //setting new player x position if within bounds
    let new_position = player_transform.translation + Vec3::new(change.x, 0., 0.);
    if new_position.x >= -(level_w / 2.) + (TILE_SIZE as f32) / 2.
        && new_position.x <= level_w / 2. - (TILE_SIZE as f32) / 2.
    {
        player_transform.translation = new_position;
        change_vec.x = player_transform.translation.x;
    }

    //setting new player y position if within bounds
    let new_pos = player_transform.translation + Vec3::new(0., change.y, 0.);
    if new_pos.y >= -(level_h / 2.) + (TILE_SIZE as f32) / 2.
        && new_pos.y <= level_h / 2. - (TILE_SIZE as f32) / 2.
    {
        player_transform.translation = new_pos;
        change_vec.y = player_transform.translation.y;
    }

    if change_vec != Vec2::splat(0.) {
        // println!("updating aabb position");
        player.aabb.update_position(change_vec);
    }
}

/*   PLAYER_ANIMATION FUNCTION   */
/// Animates the player sprite depending on the movement of
/// the player
pub fn player_animation(
    time: Res<Time>,
    mut player_query: Query<(&Velocity, &mut TextureAtlas, &mut Player), With<Player>>,
) {
    let (velocity, mut texture_atlas, mut player) = player_query.single_mut();
    //deciding what animation to use
    let new_state = if velocity.v.cmpeq(Vec2::ZERO).all() {
        SpriteState::Idle
    } else if velocity.v.x < 0. {
        SpriteState::LeftRun
    } else if velocity.v.x > 0. {
        SpriteState::RightRun
    } else if velocity.v.y < 0. {
        SpriteState::ForwardRun
    } else if velocity.v.y > 0. {
        SpriteState::BackwardRun
    } else {
        SpriteState::Idle
    };

    //changing player animation state if needed
    if new_state != player.animation_state {
        player.animation_state = new_state;
        player.timer = Timer::from_seconds(
            player.animation_state.animation_speed(),
            TimerMode::Repeating,
        );

        //setting animation at start
        let start = player.animation_state.animation_indices();
        texture_atlas.index = start.start;
    }

    //passing time
    player.timer.tick(time.delta());

    //going to next frame
    if player.timer.just_finished() {
        let indices = player.animation_state.animation_indices();
        texture_atlas.index = if texture_atlas.index + 1 >= indices.end {
            indices.start
        } else {
            texture_atlas.index + 1
        };
    }
}

/*   SPAWN_PLAYER FUNCTION */
/// Spawns the player in the gameworld
pub fn initial_spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    gameworld_state: Res<State<GameworldState>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    host: Res<HostPlayer>,
) {
    // Check if player already exists
    if let Ok(mut transform) = player_query.get_single_mut() {
        println!(
            "Current position before transition: {:?}",
            transform.translation
        );
        // Just update position for existing player
        transform.translation.z = 0.0; // Bring back into view

        // Set position based on gameworld state
        transform.translation = match gameworld_state.get() {
            GameworldState::Island => Vec3::ZERO,
            GameworldState::Dungeon => Vec3::new(-2976.0, -2976.0, 0.0),
            _ => transform.translation,
        };
        println!("New position after transition: {:?}", transform.translation);
        return;
    }

    // Only spawn new player if player count < 4

    let player_id = host.player.id;
    if player_id >= 4 {
        return;
    }

    //getting sprite info
    let master_handle: Handle<Image> = asset_server.load("s_pirate.png");
    let master_layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 8, 5, None, None);
    let master_layout_length = master_layout.textures.len();
    let master_layout_handle = texture_atlases.add(master_layout);

    let spawn_position = match gameworld_state.get() {
        GameworldState::Dungeon => Vec3::new(-2976.0, -2976.0, 0.0),
        _ => Vec3::ZERO,
    };

    //creating hurtbox for player
    let size = Vec2::new(32., 32.);
    let offset = Vec2::new(0., 0.);

    let inventory = Inventory::new(100);

    //setting up player for spawning
    commands.spawn((
        SpriteBundle {
            texture: master_handle,
            transform: Transform {
                translation: spawn_position,
                ..default()
            },
            ..default()
        },
        TextureAtlas {
            layout: master_layout_handle,
            index: 0,
        },
        AnimationTimer::new(Timer::from_seconds(
            PLAYER_ANIMATION_TIME,
            TimerMode::Repeating,
        )),
        AnimationFrameCount::new(master_layout_length),
        Velocity::new(),
        AttackCooldown {
            remaining: Timer::from_seconds(0.75, TimerMode::Once),
        },
        Player {
            host_id: player_id,
            animation_state: SpriteState::Idle,
            timer: Timer::from_seconds(SpriteState::Idle.animation_speed(), TimerMode::Repeating),
            health: PLAYER_MAX_HP,
            max_health: PLAYER_MAX_HP,
            inventory,
            spawn_position,
            weapon: 0,
            aabb: BoundingBox::new(
                Vec2::new(spawn_position.x, spawn_position.y),
                Vec2::splat(PLAYER_SIZE / 2.0),
            ),
        },
        Hurtbox {
            size,
            offset,
            colliding: false,
            entity: PLAYER,
            iframe: Timer::from_seconds(0.75, TimerMode::Once),
            enemy: false,
        },
    ));
}

/*   SPAWN_WEAPON FUNCTION   */
/// Spawns the weapon on the player
pub fn spawn_weapon(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    player_query: Query<(Entity, &Player)>,
) {
    if let Ok((player_entity, player)) = player_query.get_single() {
        // Get initial weapon (sword)
        let sword_level = player
            .inventory
            .items
            .iter()
            .find(|item| item.item_type == ItemType::Sword)
            .map(|item| item.level)
            .unwrap_or(0);

        let master_handle: Handle<Image> = asset_server.load("s_cutlass.png");
        let master_layout =
            TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 8, 5, None, None);
        let master_layout_handle = texture_atlases.add(master_layout);

        commands.entity(player_entity).with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    texture: master_handle,
                    transform: Transform {
                        scale: Vec3::splat(2.),
                        translation: Vec3::new(32., 0.0, 0.0),
                        ..default()
                    },
                    ..default()
                },
                TextureAtlas {
                    layout: master_layout_handle,
                    index: 0,
                },
                Sword {
                    damage: Sword::default().calculate_damage(sword_level),
                    upgraded: sword_level > 0,
                },
            ));
        });
    }
}

/*   SWAP_WEAPON FUNCTION   */
/// Switches between the players weapons
pub fn swap_weapon(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Player>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    player_entity_query: Query<Entity, With<Player>>,
    player_children_query: Query<&Children, With<Player>>,
    weapon_query: Query<Entity>,
) {
    let mut player = player_query.single_mut();
    let player_entity = player_entity_query.single();

    if keyboard_input.just_pressed(KeyCode::Digit1) {
        // Switch to sword
        if player.weapon != 0 {
            if let Ok(children) = player_children_query.get_single() {
                despawn_weapon_internal(&mut commands, children, &weapon_query);
            }

            player.weapon = 0;
            println!("Switched to sword!");

            let sword_level = player
                .inventory
                .items
                .iter()
                .find(|item| item.item_type == ItemType::Sword)
                .map(|item| item.level)
                .unwrap_or(0);

            let master_handle: Handle<Image> = asset_server.load("s_cutlass.png");
            let master_layout =
                TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 8, 5, None, None);
            let master_layout_handle = texture_atlases.add(master_layout);

            let player = player_entity_query.single();
            commands.entity(player).with_children(|parent| {
                parent.spawn((
                    SpriteBundle {
                        texture: master_handle,
                        transform: Transform {
                            scale: Vec3::splat(2.),
                            translation: Vec3::new(32., 0.0, 0.0),
                            ..default()
                        },
                        ..default()
                    },
                    TextureAtlas {
                        layout: master_layout_handle,
                        index: 0,
                    },
                    Sword {
                        damage: Sword::default().calculate_damage(sword_level),
                        upgraded: sword_level > 0,
                    },
                ));
            });
        }
    } else if keyboard_input.just_pressed(KeyCode::Digit2) {
        // Switch to musket
        if player.weapon != 1 {
            if let Ok(children) = player_children_query.get_single() {
                despawn_weapon_internal(&mut commands, children, &weapon_query);
            }

            player.weapon = 1;
            println!("Switched to dagger!");

            let dagger_level = player
                .inventory
                .items
                .iter()
                .find(|item| item.item_type == ItemType::Dagger)
                .map(|item| item.level)
                .unwrap_or(0);

            let master_handle: Handle<Image> = asset_server.load("s_dagger.png");
            let master_layout =
                TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 8, 5, None, None);
            let master_layout_handle = texture_atlases.add(master_layout);

            let player = player_entity_query.single();
            commands.entity(player).with_children(|parent| {
                parent.spawn((
                    SpriteBundle {
                        texture: master_handle,
                        transform: Transform {
                            scale: Vec3::splat(2.),
                            translation: Vec3::new(32., 0.0, 0.0),
                            ..default()
                        },
                        ..default()
                    },
                    TextureAtlas {
                        layout: master_layout_handle,
                        index: 0,
                    },
                    Dagger {
                        damage: Dagger::default().calculate_damage(dagger_level),
                        upgraded: dagger_level > 0,
                    },
                ));
            });
        }
    } else if keyboard_input.just_pressed(KeyCode::Digit3) {
        // Switch to dagger
        if player.weapon != 2 {
            if let Ok(children) = player_children_query.get_single() {
                despawn_weapon_internal(&mut commands, children, &weapon_query);
            }

            player.weapon = 2;
            println!("Switched to musket!");

            let musket_level = player
                .inventory
                .items
                .iter()
                .find(|item| item.item_type == ItemType::Musket)
                .map(|item| item.level)
                .unwrap_or(0);
            let master_handle: Handle<Image> = asset_server.load("s_musket.png");
            let master_layout =
                TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 8, 5, None, None);
            let master_layout_handle = texture_atlases.add(master_layout);

            let player = player_entity_query.single();
            commands.entity(player).with_children(|parent| {
                parent.spawn((
                    SpriteBundle {
                        texture: master_handle,
                        transform: Transform {
                            scale: Vec3::splat(1.),
                            translation: Vec3::new(40., 0., 0.),
                            ..default()
                        },
                        ..default()
                    },
                    Musket {
                        damage: Musket::default().calculate_damage(musket_level),
                        upgraded: musket_level > 0,
                    },
                ));
            });
        }
    } else if keyboard_input.just_pressed(KeyCode::Digit4) {
        // Switch to pistol
        if player.weapon != 3 {
            if let Ok(children) = player_children_query.get_single() {
                despawn_weapon_internal(&mut commands, children, &weapon_query);
            }

            player.weapon = 3;
            println!("Switched to pistol!");

            let pistol_level = player
                .inventory
                .items
                .iter()
                .find(|item| item.item_type == ItemType::Pistol)
                .map(|item| item.level)
                .unwrap_or(0);

            let master_handle: Handle<Image> = asset_server.load("s_pistol.png");
            let master_layout =
                TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 8, 5, None, None);
            let master_layout_handle = texture_atlases.add(master_layout);

            let player = player_entity_query.single();
            commands.entity(player).with_children(|parent| {
                parent.spawn((
                    SpriteBundle {
                        texture: master_handle,
                        transform: Transform {
                            scale: Vec3::splat(1.),
                            translation: Vec3::new(40., 0., 0.),
                            ..default()
                        },
                        ..default()
                    },
                    Pistol {
                        damage: Pistol::default().calculate_damage(pistol_level),
                        upgraded: pistol_level > 0,
                    },
                ));
            });
        }
    }
}

/*   MOVE_WEAPON FUNCITON   */
/// Move the weapon with the player (reflects where player is facing)
pub fn move_weapon(
    player_query: Query<(&Player, &Children)>,
    mut transform_query: Query<(&mut Transform, &mut Sprite)>,
) {
    for (player, children) in player_query.iter() {
        // For each child (active weapon)
        for &child in children.iter() {
            if let Ok((mut transform, mut sprite)) = transform_query.get_mut(child) {
                let player_direction = player.animation_state;
                let is_gun = player.weapon >= 2; // weapons 2 and 3 are guns

                if player_direction == SpriteState::LeftRun
                    || player_direction == SpriteState::BackwardRun
                {
                    transform.translation = if is_gun {
                        Vec3::new(-40., 0., 0.)
                    } else {
                        Vec3::new(-32., 0., 0.)
                    };
                    sprite.flip_x = true;
                } else if player_direction == SpriteState::RightRun
                    || player_direction == SpriteState::ForwardRun
                {
                    transform.translation = if is_gun {
                        Vec3::new(40., 0., 0.)
                    } else {
                        Vec3::new(32., 0., 0.)
                    };
                    sprite.flip_x = false;
                }
            }
        }
    }
}

/*   PLAYER_ATTACK FUNCTION   */
/// Creates an attacking hitbox that will deal damage to enemy entities
pub fn sword_attack(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    curr_mouse_pos: ResMut<CurrMousePos>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_query: Query<
        (
            Entity,
            &Transform,
            &Velocity,
            &mut AttackCooldown,
            &mut Player,
        ),
        With<Player>,
    >,
) {
    for (entity, transform, velocity, mut cooldown, player) in player_query.iter_mut() {
        //If the cooldown is not finished, tick and break because you can't attack anyway
        if !cooldown.remaining.finished() {
            cooldown.remaining.tick(time.delta());
            break;
        }

        //checking if weapon is sword
        if player.weapon == 0 {
            // Checks if the left mouse button is pressed
            if get_player_input(PlayerControl::Attack, &keyboard_input, &mouse_input) == 1. {
                println!("Player attacked!");
                let mouse_pos = curr_mouse_pos.0;
                println!("Mouse world coords {} {}", mouse_pos.x, mouse_pos.y);
                cooldown.remaining = Timer::from_seconds(0.75, TimerMode::Once);

                // Player position
                let player_position = transform.translation.truncate();

                // Calculate direction from player position to mouse position
                let direction = (mouse_pos - player_position).normalize();
                // Calculate hitbox offset based on direction
                let hitbox_offset = direction * 50.0; // Distance from the player to the hitbox

                // Define the size of the hitbox
                let hitbox_size = Vec2::new(40.0, 60.0);

                // Create the hitbox
                create_hitbox(
                    &mut commands,
                    entity,
                    hitbox_size,
                    hitbox_offset,
                    Some(0.1),
                    PLAYER,
                    false,
                    false,
                );

                commands.spawn((
                    SpriteBundle {
                        texture: asset_server.load("s_sword_swipe_bigger.png"), // Replace with your swoosh sprite path
                        transform: Transform::from_translation(
                            transform.translation + direction.extend(0.1) * 50.0,
                        )
                        .with_rotation(Quat::from_rotation_z(direction.angle_between(Vec2::X))),
                        ..default()
                    },
                    SwordSwooshAnimation {
                        timer: Timer::from_seconds(0.05, TimerMode::Once), // Adjust duration as needed
                        active: true,
                    },
                ));
            }
        }
    }
}

pub fn sword_swoosh_animation(
    time: Res<Time>,
    mut commands: Commands,
    mut swoosh_query: Query<(Entity, &mut SwordSwooshAnimation)>,
) {
    for (entity, mut swoosh) in swoosh_query.iter_mut() {
        swoosh.timer.tick(time.delta());

        if swoosh.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn dagger_attack(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    curr_mouse_pos: ResMut<CurrMousePos>,
    mut commands: Commands,
    mut player_query: Query<
        (Entity, &Transform, &Velocity, &mut AttackCooldown, &Player),
        With<Player>,
    >,
) {
    for (entity, transform, velocity, mut cooldown, player) in player_query.iter_mut() {
        if !cooldown.remaining.finished() {
            cooldown.remaining.tick(time.delta());
            break;
        }

        if player.weapon == 1 {
            if get_player_input(PlayerControl::Attack, &keyboard_input, &mouse_input) == 1. {
                println!("Player attacked with dagger!");
                cooldown.remaining = Timer::from_seconds(DAGGER_COOLDOWN, TimerMode::Once);

                let player_position = transform.translation.truncate();
                let direction = (curr_mouse_pos.0 - player_position).normalize();
                let hitbox_offset = direction * 25.0; // Shorter range than sword
                let hitbox_size = Vec2::new(30.0, 45.0); // Smaller hitbox than sword

                create_hitbox(
                    &mut commands,
                    entity,
                    hitbox_size,
                    hitbox_offset,
                    Some(0.05), // Faster hitbox duration
                    PLAYER,
                    false,
                    false,
                );
            }
        }
    }
}
/*   MUSKET_ATTACK FUNCTION */
/// Function that fires the musket if players weapon is set to musket
pub fn musket_attack(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    curr_mouse_pos: ResMut<CurrMousePos>,
    mut commands: Commands,
    mut player_query: Query<
        (
            Entity,
            &Transform,
            &Velocity,
            &mut AttackCooldown,
            &mut Player,
        ),
        With<Player>,
    >,
    asset_server: Res<AssetServer>,
) {
    for (entity, transform, velocity, mut cooldown, player) in player_query.iter_mut() {
        //If the cooldown is not finished, tick and break because you can't attack anyway
        if !cooldown.remaining.finished() {
            cooldown.remaining.tick(time.delta());
            break;
        }

        if player.weapon == 2 {
            // Checks if the left mouse button is pressed
            if get_player_input(PlayerControl::Attack, &keyboard_input, &mouse_input) == 1. {
                println!("Player attacked!");
                let mouse_pos = curr_mouse_pos.0;
                println!("Mouse world coords {} {}", mouse_pos.x, mouse_pos.y);
                cooldown.remaining = Timer::from_seconds(0.75, TimerMode::Once);

                // Player position
                let player_position = transform.translation.truncate();

                //getting angle to fire at
                let pos2 = curr_mouse_pos.0;
                let original_direction =
                    (Vec3::new(pos2.x, pos2.y, 0.) - transform.translation).normalize();
                let angle = original_direction.x.atan2(original_direction.y);
                let firing_angle = Vec3::new(angle.sin(), angle.cos(), 0.0).normalize();

                //getting musketball sprite
                let musketball_handler = asset_server.load("s_cannonball.png");

                //getting start position to fire from
                let projectile_start_position = transform.translation.xyz();

                //getting hitbox info
                let hitbox_size = Vec2::new(15., 15.);
                let offset = Vec2::new(0., 0.);

                //spawning cannonball
                commands.spawn((
                    SpriteBundle {
                        texture: musketball_handler,
                        transform: Transform {
                            translation: projectile_start_position,
                            scale: Vec3::splat(0.8),
                            ..default()
                        },
                        ..default()
                    },
                    Musketball,
                    MusketballLifetime(MUSKETBALL_LIFETIME),
                    MusketballVelocity {
                        v: firing_angle * MUSKETBALL_SPEED, /* (direction * speed of projectile) */
                    },
                    Hitbox {
                        size: hitbox_size,
                        offset: offset,
                        lifetime: Some(Timer::from_seconds(MUSKETBALL_LIFETIME, TimerMode::Once)),
                        entity: PLAYER,
                        projectile: true,
                        enemy: false,
                    },
                ));
            }
        }
    }
}

pub fn pistol_attack(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    curr_mouse_pos: ResMut<CurrMousePos>,
    mut commands: Commands,
    mut player_query: Query<
        (Entity, &Transform, &Velocity, &mut AttackCooldown, &Player),
        With<Player>,
    >,
    asset_server: Res<AssetServer>,
) {
    for (entity, transform, velocity, mut cooldown, player) in player_query.iter_mut() {
        if !cooldown.remaining.finished() {
            cooldown.remaining.tick(time.delta());
            break;
        }

        if player.weapon == 3 {
            if get_player_input(PlayerControl::Attack, &keyboard_input, &mouse_input) == 1. {
                println!("Player attacked with pistol!");
                cooldown.remaining = Timer::from_seconds(PISTOL_COOLDOWN, TimerMode::Once);

                let player_position = transform.translation;
                let original_direction = (Vec3::new(curr_mouse_pos.0.x, curr_mouse_pos.0.y, 0.)
                    - transform.translation)
                    .normalize();
                let angle = original_direction.x.atan2(original_direction.y);
                let firing_angle = Vec3::new(angle.sin(), angle.cos(), 0.0);
                let projectile_start_position = player_position + firing_angle * 10.0;

                let hitbox_size = Vec2::new(12., 12.);
                let offset = Vec2::splat(0.);

                commands.spawn((
                    SpriteBundle {
                        texture: asset_server.load("s_cannonball.png"),
                        transform: Transform {
                            translation: projectile_start_position,
                            scale: Vec3::splat(0.6),
                            ..default()
                        },
                        ..default()
                    },
                    Musketball,
                    MusketballLifetime(MUSKETBALL_LIFETIME),
                    MusketballVelocity {
                        v: firing_angle * (MUSKETBALL_SPEED), // Slightly slower than musket
                    },
                    Hitbox {
                        size: hitbox_size,
                        offset: offset,
                        lifetime: Some(Timer::from_seconds(MUSKETBALL_LIFETIME, TimerMode::Once)),
                        entity: PLAYER,
                        projectile: true,
                        enemy: false,
                    },
                ));
            }
        }
    }
}

/*   MOVE_MUSKETBALL FUNCTION   */
/// Updates the locations of musket projectiles
pub fn move_musketball(
    mut proj_query: Query<(&mut Transform, &mut MusketballVelocity), With<Musketball>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in proj_query.iter_mut() {
        // Calculates/moves the projectile
        transform.translation += velocity.v * time.delta_seconds();
    }
}

/*   CHECK_PLAYER_HEALTH FUNCTION   */
/// Function checks the current state of the player's health
/// if current health == 0 --> respawn player
pub fn check_player_health(
    mut player_query: Query<(&mut Player, Entity, &mut Hurtbox, &mut Transform), With<Player>>,
    gameworld_state: Res<State<GameworldState>>,
) {
    for (mut player, entity, mut hurtbox, mut transform) in player_query.iter_mut() {
        if !hurtbox.colliding {
            continue;
        }

        player.health -= 1.;

        if player.health <= 0. {
            println!("Player died... yikes!");
            player.health = player.max_health;

            let respawn_pos = match gameworld_state.get() {
                GameworldState::Dungeon => Vec3::new(-2976.0, -2976.0, 0.0),
                _ => Vec3::ZERO,
            };

            transform.translation = respawn_pos;
            println!("Player respawned!");
        } else {
            println!("Ouch! Player was hit.");
        }

        hurtbox.colliding = false;
    }
}

/*   DESPAWN_PLAYER FUNCTION   */
/// Despawns the player entity
/// DEBUG: Will despawn any and all players
pub fn despawn_player(mut player_query: Query<&mut Transform, With<Player>>) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        transform.translation.z = 10000.0;
    }
}

/*   MUSKETBALL_LIFETIME_CHECK FUNCTION   */
/// Checks the lifetime of a musketball
pub fn musketball_lifetime_check(
    time: Res<Time>,
    mut commands: Commands,
    mut proj_query: Query<(Entity, &mut MusketballLifetime)>,
) {
    for (entity, mut lifetime) in proj_query.iter_mut() {
        lifetime.0 -= time.delta_seconds();
        if lifetime.0 <= 0.0 {
            commands.entity(entity).despawn();

            /* Debug */
            println!("Musketball despawned");
        }
    }
}

/*   DESPAWN_MUSKETBALLS FUNCTION   */
/// Despawns musketballs
/// DEBUG: Will despawn any and all musketballs
pub fn despawn_musketballs(mut commands: Commands, query: Query<Entity, With<Musketball>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn despawn_weapon_internal(
    commands: &mut Commands,
    children: &Children,
    weapon_query: &Query<Entity>,
) {
    for &child in children.iter() {
        if let Ok(weapon_entity) = weapon_query.get(child) {
            commands.entity(weapon_entity).despawn();
        }
    }
}
