use bevy::prelude::*;

use crate::data::gameworld_data::*;
use crate::enemies::*;
use crate::hitbox_system::*;
use crate::player::components::*;
use crate::boss::components::*;

pub fn spawn_boss(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let transform = Transform::from_xyz(0., -(WIN_H / 1.5) + ((TILE_SIZE as f32) * 1.5), 900.);

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("s_boss.png"), // You'll need this sprite
            transform,
            ..default()
        },
        Boss {
            current_hp: BOSS_MAX_HP,
        },
        TextureAtlas {
            layout: texture_atlases.add(TextureAtlasLayout::from_grid(
                UVec2::splat(TILE_SIZE * 2),
                2,
                1,
                None,
                None,
            )),
            index: 0,
        },
        AnimationTimer::new(Timer::from_seconds(
            BOSS_ANIMATION_TIME,
            TimerMode::Repeating,
        )),
        AnimationFrameCount::new(2),
        Velocity::new(),
        Hurtbox {
            size: Vec2::splat(80.), // Larger hitbox than rock
            offset: Vec2::splat(0.),
            colliding: false,
            entity: BOSS,
            iframe: Timer::from_seconds(0.75, TimerMode::Once),
            enemy: true,
        },
        Hitbox {
            size: Vec2::splat(70.), // Larger hitbox for damage
            offset: Vec2::splat(0.),
            lifetime: Some(Timer::from_seconds(1000000., TimerMode::Repeating)),
            projectile: false,
            entity: BOSS,
            enemy: true,
        },
    ));
}

pub fn boss_damaged(
    mut commands: Commands,
    mut boss_query: Query<(&mut Boss, Entity, &mut Hurtbox), With<Boss>>,
) {
    for (mut boss, entity, mut hurtbox) in boss_query.iter_mut() {
        if !hurtbox.colliding {
            continue;
        }

        boss.current_hp -= 1.;

        if boss.current_hp <= 0. {
            println!("Boss was defeated!");
            commands.entity(entity).despawn();
        } else {
            println!("Boss was damaged! HP: {}", boss.current_hp);
        }

        hurtbox.colliding = false;
    }
}

pub fn despawn_all_bosses(mut commands: Commands, query: Query<Entity, With<Boss>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn move_boss(
    time: Res<Time>,
    mut boss_query: Query<&mut Transform, With<Boss>>,
    player_query: Query<&Transform, (With<Player>, Without<Boss>)>,
) {
    for mut transform in boss_query.iter_mut() {
        //Gets positions (Vec3) of the entities
        let boss_translation = transform.translation;
        let player_translation = player_query.single().translation;

        //Gets positions (Vec2) of the entities
        let player_position = player_translation.xy();
        let boss_position = boss_translation.xy();

        //Gets distance
        let distance_to_player = boss_position.distance(player_position);

        //Check
        if distance_to_player > BOSS_AGRO_RANGE {
            continue;
        }

        //Gets direction to move
        let direction = (player_translation - boss_translation).normalize();
        let velocity = direction * BOSS_MOVEMENT_SPEED;

        //Moves boss
        transform.translation += velocity * time.delta_seconds();
    }
}