use bevy::prelude::*;

use crate::bat::components::*;
use crate::data::gameworld_data::*;
use crate::hitbox_system::*;
use crate::player::components::AttackCooldown;
use crate::player::components::Player;

/*   ROTATE_BAT FUNCTION   */
/// This should be changed to a function called "track_player", which will
/// be how the bat knows where to check where the player is for shooting projectiles
///
/// WE DON'T NEED TO ROTATE THE BAT! I WILL MAKE A BACK FACING SPRITE IF NEEDED
pub fn rotate_bat(
    time: Res<Time>,
    mut query: Query<(&Bat, &mut Transform), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
) {
    // getting player position
    let player_transform = player_query.single();
    let player_translation = player_transform.translation.xy();

    for (bat, mut enemy_transform) in &mut query {
        //getting bat's position relative to player position
        let bat_position = enemy_transform.translation.xy();
        let distance_to_player = bat_position.distance(player_translation);

        //ensuring bat is close enough to player to attack
        if distance_to_player > BAT_ATTACK_DIST {
            break;
        }

        //getting enemy forward
        let enemy_forward = (enemy_transform.rotation * Vec3::Y).xy();
        let to_player = (player_translation - enemy_transform.translation.xy()).normalize();
        let forward_dot_player = enemy_forward.dot(to_player);

        if (forward_dot_player - 1.0).abs() < f32::EPSILON {
            continue;
        }

        let enemy_right = (enemy_transform.rotation * Vec3::X).xy();

        let right_dot_player = enemy_right.dot(to_player);

        let rotation_sign = -f32::copysign(1.0, right_dot_player);
        let max_angle = forward_dot_player.clamp(-1.0, 1.0).acos();

        let rotation_angle =
            rotation_sign * (bat.rotation_speed * time.delta_seconds()).min(max_angle);

        enemy_transform.rotate_z(rotation_angle);
    }
}

/*   ANIMATE_BAT FUNCTION   */
/// Animates a bat entity
pub fn animate_bat(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlas, &AnimationFrameCount)>,
) {
    for (mut timer, mut texture_atlas, frame_count) in &mut query {
        timer.tick(time.delta());

        if timer.finished() {
            texture_atlas.index = (texture_atlas.index + 1) % **frame_count;
        }
    }
}

/*  SPAWN_BAT FUNCTION  */
/// Spawns a bat entity in the gameworld
pub fn spawn_bat(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    //getting bat sprite information
    let bat_sheet_handle = asset_server.load("s_bat.png");
    let bat_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 3, 1, None, None);
    let bat_layout_len = 3;
    let bat_layout_handle = texture_atlases.add(bat_layout.clone());

    //spawning bat and setting bat information
    commands.spawn((
        SpriteBundle {
            texture: bat_sheet_handle,
            transform: Transform::from_xyz(0., -(WIN_H / 2.) + ((TILE_SIZE as f32) * 1.5), 900.)
                .with_scale(Vec3::splat(2.0)),
            ..default()
        },
        Bat {
            //Setting default stats
            rotation_speed: f32::to_radians(90.0),
            current_hp: BAT_MAX_HP,
            max_hp: BAT_MAX_HP,
        },
        TextureAtlas {
            layout: bat_layout_handle,
            index: 0,
        },
        AttackCooldown {
            remaining: Timer::from_seconds(1.5, TimerMode::Once),
        },
        AnimationTimer::new(Timer::from_seconds(
            BAT_ANIMATION_TIME,
            TimerMode::Repeating,
        )),
        AnimationFrameCount::new(bat_layout_len),
        Velocity::new(),
    ));
}

/*
    Detects when player is within Bat attack range and attacks.

    Collision handling will be done in move_bat_projectile

    Things not added:
    - Done!...?

    Things currently added:
    - Distance-to-player checking
    - Attack cooldown timer
    - Projectile shooting
*/

pub fn bat_attack(
    mut commands: Commands,
    time: Res<Time>,
    mut bat_query: Query<(&Transform, &mut AttackCooldown), With<Bat>>,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    for (bat_transform, mut cooldown) in bat_query.iter_mut() {
        // Attacks only when cooldown is over
        cooldown.remaining.tick(time.delta());
        if !cooldown.remaining.just_finished() {
            continue;
        }

        cooldown.remaining = Timer::from_seconds(1.5, TimerMode::Once);

        //Gets positions (Vec2) of the entities
        let player_position = player_query.single().translation.xy();
        let bat_position = bat_transform.translation.xy();

        //Gets distance
        let distance_to_player = bat_position.distance(player_position);

        if distance_to_player > BAT_ATTACK_DIST {
            continue;
        }

        /* Debug */
        //println!("Bat can attack player! :O");

        //Gets positions (Vec3) of the entities
        let bat_translation = bat_transform.translation;
        let player_translation = player_query.single().translation;

        //Gets direction projectile will be going
        let direction = (player_translation - bat_translation).normalize();

        //Sets the projectile texture
        let bat_projectile_handle = asset_server.load("s_cutlass.png");

        let projectile_start_position = bat_translation + direction * 100.0; // Adjust the multiplier as needed

        //Creates Projectile
        commands.spawn((
            SpriteBundle {
                texture: bat_projectile_handle,
                transform: Transform::from_translation(projectile_start_position), //Spawns at the bat's location
                ..default()
            },
            BatProjectile,
            Lifetime(BAT_PROJECTILE_LIFETIME),
            Velocity {
                v: direction * BAT_PROJECTILE_SPEED, /* (direction * speed of projectile) */
            },
        ));
    }
}

/*
    Current functionality: Detects when a player is within player attack range (this will later be replaced with
    player weapon/attack collision) and then takes 1 damage (dies)
*/
pub fn bat_damaged(
    mut commands: Commands,
    mut bat_query: Query<(&Transform, &mut Bat, Entity), With<Bat>>,
    player_query: Query<&Transform, With<Player>>,
) {
    for (bat_transform, mut bat, entity) in bat_query.iter_mut() {
        //Gets entity locations
        let player_position = player_query.single().translation.xy();
        let bat_position = bat_transform.translation.xy();

        //Calculates distance
        let distance_to_player = bat_position.distance(player_position);

        //Placeholder value for player attack range
        let player_attack_range = 50.;

        //If the distance is too large (in this case larger than 50) then continue to next bat entity
        if distance_to_player > player_attack_range {
            continue;
        }

        //HP deduction and check
        bat.current_hp -= 1.;

        if bat.current_hp <= 0. {
            println!("Bat was attacked by player, it is dead :(");
            commands.entity(entity).despawn();
        } else {
            println!("Bat was attacked by player");
        }
    }
}

/*
    Updates the locations of bat projectiles

    Things to add:
    - Collision handling, dealing damage on collision
*/
pub fn move_bat_projectile(
    mut proj_query: Query<(&mut Transform, &mut Velocity), With<BatProjectile>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in proj_query.iter_mut() {
        // Calculates/moves the projectile
        transform.translation += velocity.v * time.delta_seconds();

        //Remove this line if you want the projectile to stop at the coordinate player was targetted
        transform.translation.z = 0.;
    }
}

pub fn bat_proj_lifetime_check(
    time: Res<Time>,
    mut commands: Commands,
    mut proj_query: Query<(Entity, &mut Lifetime)>,
) {
    for (entity, mut lifetime) in proj_query.iter_mut() {
        lifetime.0 -= time.delta_seconds();
        if lifetime.0 <= 0.0 {
            commands.entity(entity).despawn();

            /* Debug */
            println!("Projectile despawned");
        }
    }
}
