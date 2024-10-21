use bevy::math::vec2;
use bevy::prelude::*;

use crate::bat::components::*;
use crate::data::gameworld_data::*;
use crate::hitbox_system::*;
use crate::player::components::AttackCooldown;
use crate::player::components::Player;

use bevy::math::bounding::Aabb2d;

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
        Hurtbox {
            size: Vec2::splat(25.),
            offset: Vec2::splat(0.),
            colliding: false,
            entity: BAT,
        },
    ));
}

/*   BAT_DAMAGED FUNCTION   */
/// Current functionality: Detects when a player is within player attack range (this will later be replaced with
// player weapon/attack collision) and then takes 1 damage (dies)
pub fn bat_damaged(
    mut commands: Commands,
    mut bat_query: Query<(&mut Bat, Entity, &mut Hurtbox), With<Bat>>,
) {
    for (mut bat, entity, mut hurtbox) in bat_query.iter_mut() {
        if !hurtbox.colliding {
            continue;
        }

        bat.current_hp -= 1.;

        if bat.current_hp <= 0. {
            println!("Bat was attacked by player, it is dead :(");
            commands.entity(entity).despawn();
        } else {
            println!("Bat was attacked by player");
        }

        hurtbox.colliding = false;
    }
}

/*   DESPAWN_ALL_BAT FUNCTION   */
/// Despawns a bat entity
/// DEBUG: Despwans all bat entities
pub fn despawn_all_bats(mut commands: Commands, query: Query<Entity, With<Bat>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

/*   BAT_ATTACK FUNCTION   */
/// Detects when player is within Bat attack range and attacks.
/// Things not added:
/// * Attack cooldown timer
/// * Projectile shooting
/// Things currently added:
/// * Distance-to-player checking
/// * Attack cooldown timer
/// * Projectile shooting
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

        //Gets positions (Vec3) of the entities
        let bat_translation = bat_transform.translation;
        let player_translation = player_query.single().translation;

        //Gets positions (Vec2) of the entities
        let player_position = player_translation.xy();
        let bat_position = bat_translation.xy();

        //Gets distance
        let distance_to_player = bat_position.distance(player_position);

        if distance_to_player > BAT_ATTACK_DIST {
            continue;
        }

        //Gets direction projectile will be going
        let original_direction = (player_translation - bat_translation).normalize();
        let angle = original_direction.x.atan2(original_direction.y);
        let angle_direction = Vec3::new(angle.sin(), angle.cos(), 0.0).normalize();

        let projectile_start_position = bat_translation + angle_direction * 10.0; //bat_pos + direction * offset wanted

        //Sets the projectile texture
        let bat_projectile_handle = asset_server.load("s_sonic_boom.png");

        //Creates Projectile
        commands.spawn((
            SpriteBundle {
                texture: bat_projectile_handle,
                transform: Transform {
                    translation: projectile_start_position,
                    scale: Vec3::splat(2.0),
                    ..default()
                },
                ..default()
            },
            BatProjectile,
            Lifetime(BAT_PROJECTILE_LIFETIME),
            Velocity {
                v: angle_direction * BAT_PROJECTILE_SPEED, /* (direction * speed of projectile) */
            },
            Hitbox {
                size: Vec2::splat(16.),
                offset: Vec2::splat(0.),
                lifetime: Some(Timer::from_seconds(3., TimerMode::Once)),
                entity: BAT,
            },
        ));
    }
}

/*   MOVE_BAT_PROJECTILE FUNCTION   */
/// Updates the locations of bat projectiles
/// Things to add:
/// * Collision handling, dealing damage on collision
pub fn move_bat_projectile(
    mut proj_query: Query<(&mut Transform, &mut Velocity), With<BatProjectile>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in proj_query.iter_mut() {
        // Calculates/moves the projectile
        transform.translation += velocity.v * time.delta_seconds();
    }
}

/*   BAT_PROJ_LIFETIME_CHECK FUNCTION   */
/// Checks the lifetime left on a bat's projectile, and despawns
/// after the lifetime expires
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

/*   MOVE_BAT FUNCTION   */
/// Moves the bat as long as a player is within agro range
pub fn move_bat(
    time: Res<Time>,
    mut bat_query: Query<&mut Transform, With<Bat>>,
    player_query: Query<&Transform, (With<Player>, Without<Bat>)>,
) {
    for mut transform in bat_query.iter_mut() {
        //Gets positions (Vec3) of the entities
        let bat_translation = transform.translation;
        let player_translation = player_query.single().translation;

        //Gets positions (Vec2) of the entities
        let player_position = player_translation.xy();
        let bat_position = bat_translation.xy();

        //Gets distance
        let distance_to_player = bat_position.distance(player_position);

        //Check
        if distance_to_player > BAT_AGRO_RANGE || distance_to_player <= BAT_AGRO_STOP {
            continue;
        }

        //Gets direction projectile will be going
        let direction = (player_translation - bat_translation).normalize();
        let velocity = direction * BAT_MOVEMENT_SPEED;

        //Moves bat
        transform.translation += velocity * time.delta_seconds();
    }
}

/*   DESPAWN_ALL_BAT_PROJ   */
/// Despawns all the bat's projectiles
pub fn despawn_all_bat_proj(
    mut commands: Commands,
    mut proj_query: Query<(Entity, &mut Lifetime)>,
) {
    for (entity, mut lifetime) in proj_query.iter_mut() {
        commands.entity(entity).despawn();
    }
}
