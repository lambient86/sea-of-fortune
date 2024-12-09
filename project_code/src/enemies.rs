use bevy::prelude::*;

use crate::bat::components::*;
use crate::data::gameworld_data::*;
use crate::ghost_ship::components::*;
use crate::hitbox_system::components::*;
use crate::kraken::components::*;
use crate::player::components::*;
use crate::poison_skeleton::components::*;
use crate::rock::components::*;
use crate::skeleton::components::*;
//use crate::storm::components::*;
use crate::storm::components::*;
use crate::whirlpool::components::*;
use crate::Enemy;

//ENTITIES
pub const PLAYER: i32 = 0;
pub const BOAT: i32 = 1;
pub const BAT: i32 = 2;
pub const KRAKEN: i32 = 3;
pub const GHOSTSHIP: i32 = 4;
pub const ROCK: i32 = 5;
pub const SKELETON: i32 = 6;
pub const SKEL2: i32 = 7;
pub const WHIRLPOOL: i32 = 8;
pub const BOSS: i32 = 9;
pub const STORM: i32 = 10;
pub const PSKELETON: i32 = 10;

#[derive(Component)]
pub struct EnemyTag;

#[derive(Component)]
pub struct Lifetime(pub f32);

pub enum EnemyT {
    Bat,
    Kraken(i32),
    GhostShip(i32),
    Rock,
    RSkeleton,
    Whirlpool(i32),
    Storm(i32),
    PoisonSkeleton,
}

pub fn spawn_enemy(
    commands: &mut Commands,
    enemy: EnemyT,
    transform: Transform,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    match enemy {
        EnemyT::Storm(id) => {
            // Spawn the parent entity
            commands
                .spawn((
                    SpatialBundle {
                        transform,
                        ..default()
                    },
                    Storm {
                        damage_timer: Timer::from_seconds(
                            STORM_DAMAGE_INTERVAL,
                            TimerMode::Repeating,
                        ),
                    },
                    Hurtbox {
                        size: Vec2::new(1200.0, 900.0),
                        offset: Vec2::splat(0.),
                        colliding: Collision::default(),
                        entity: STORM,
                        iframe: Timer::from_seconds(0.75, TimerMode::Once),
                        enemy: true,
                    },
                    Enemy {
                        id,
                        etype: STORM,
                        pos: transform.translation,
                        animation_index: 0,
                        hp: 1.,
                        alive: true,
                        target_id: -1,
                    },
                    EnemyTag,
                ))
                .with_children(|parent| {
                    // Spawn the transparent background
                    parent.spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgba(0.5, 0.5, 0.5, 0.3),
                            custom_size: Some(Vec2::new(1200.0, 900.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(0.0, 0.0, 0.0),
                        ..default()
                    });

                    // Spawn the storm image on top
                    parent.spawn(SpriteBundle {
                        texture: asset_server.load("s_storm.png"), // Make sure to add your storm image to assets
                        transform: Transform::from_xyz(0.0, 0.0, 1.0), // Slightly higher z-index
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(1200.0, 900.0)), // Same size as background
                            ..default()
                        },
                        ..default()
                    });
                });
        }
        EnemyT::Whirlpool(id) => {
            let whirlpool_texture_asset: Handle<Image> = asset_server.load("s_whirlpool.png");

            commands.spawn((
                SpriteBundle {
                    texture: whirlpool_texture_asset,
                    transform,
                    ..default()
                },
                Whirlpool {
                    rotation_speed: f32::to_radians(90.0),
                    current_hp: WHIRLPOOL_HP,
                    max_hp: WHIRLPOOL_HP,
                },
                Lifetime(WHIRLPOOL_LIFETIME),
                Hurtbox {
                    size: Vec2::new(400., 290.),
                    offset: Vec2::splat(0.),
                    colliding: Collision::default(),
                    entity: WHIRLPOOL,
                    iframe: Timer::from_seconds(0.75, TimerMode::Once),
                    enemy: true,
                },
                Enemy {
                    id,
                    etype: WHIRLPOOL,
                    pos: transform.translation,
                    animation_index: 0,
                    hp: WHIRLPOOL_HP,
                    alive: true,
                    target_id: -1,
                },
                EnemyTag,
            ));
        }
        EnemyT::Bat => {
            let bat_layout =
                TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 3, 1, None, None);

            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("s_bat.png"),
                    transform,
                    ..default()
                },
                Bat {
                    //Setting default stats
                    rotation_speed: f32::to_radians(90.0),
                    current_hp: BAT_MAX_HP,
                    max_hp: BAT_MAX_HP,
                },
                TextureAtlas {
                    layout: texture_atlases.add(bat_layout.clone()),
                    index: 0,
                },
                AttackCooldown {
                    remaining: Timer::from_seconds(1.5, TimerMode::Once),
                },
                AnimationTimer::new(Timer::from_seconds(
                    BAT_ANIMATION_TIME,
                    TimerMode::Repeating,
                )),
                AnimationFrameCount::new(3),
                Velocity::new(),
                Hurtbox {
                    size: Vec2::splat(25.),
                    offset: Vec2::splat(0.),
                    colliding: Collision::default(),
                    entity: BAT,
                    iframe: Timer::from_seconds(0.75, TimerMode::Once),
                    enemy: true,
                },
            ));
        }
        EnemyT::Kraken(id) => {
            let kraken_texture_handle = asset_server.load("s_kraken.png");

            commands.spawn((
                SpriteBundle {
                    texture: kraken_texture_handle,
                    transform,
                    ..default()
                },
                Kraken {
                    //Setting default stats
                    rotation_speed: f32::to_radians(90.0),
                    current_hp: KRAKEN_MAX_HP,
                    max_hp: KRAKEN_MAX_HP,
                },
                AttackCooldown {
                    remaining: Timer::from_seconds(1.5, TimerMode::Once),
                },
                Hurtbox {
                    size: Vec2::new(160., 90.),
                    offset: Vec2::splat(0.),
                    colliding: Collision::default(),
                    entity: KRAKEN,
                    iframe: Timer::from_seconds(0.75, TimerMode::Once),
                    enemy: true,
                },
                Enemy {
                    id,
                    etype: KRAKEN,
                    pos: transform.translation,
                    animation_index: 0,
                    hp: KRAKEN_MAX_HP,
                    alive: true,
                    target_id: -1,
                },
                EnemyTag,
            ));
        }
        EnemyT::GhostShip(id) => {
            let ghostship_texture_handle = asset_server.load("s_ghost_ship.png");

            commands.spawn((
                SpriteBundle {
                    texture: ghostship_texture_handle,
                    transform,
                    ..default()
                },
                GhostShip {
                    //Setting default stats
                    rotation_speed: f32::to_radians(90.0),
                    current_hp: GHOSTSHIP_MAX_HP,
                },
                AttackCooldown {
                    remaining: Timer::from_seconds(1.5, TimerMode::Once),
                },
                Hurtbox {
                    size: Vec2::new(160., 90.),
                    offset: Vec2::splat(0.),
                    colliding: Collision::default(),
                    entity: GHOSTSHIP,
                    iframe: Timer::from_seconds(0.75, TimerMode::Once),
                    enemy: true,
                },
                Enemy {
                    id,
                    etype: GHOSTSHIP,
                    pos: transform.translation,
                    animation_index: 0,
                    hp: GHOSTSHIP_MAX_HP,
                    alive: true,
                    target_id: -1,
                },
                EnemyTag,
            ));
        }
        EnemyT::RSkeleton => {
            let skeleton_layout = TextureAtlasLayout::from_grid(
                UVec2::new(31, 32), // SpriteSheet 1 pixel off, maybe fix later? it works like this though
                6,                  // Columns
                1,                  // Rows
                None,               // Padding
                None,               // Spacing
            );

            // Add the texture atlas to the resource

            // Spawn the skeleton entity
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("s_skeleton.png"), // This uses the TextureAtlas handle
                    transform,
                    ..default()
                },
                Skeleton {
                    rotation_speed: 0.0,
                    current_hp: SKELETON_MAX_HP,
                    max_hp: SKELETON_MAX_HP,
                },
                TextureAtlas {
                    layout: texture_atlases.add(skeleton_layout.clone()),
                    index: 0,
                },
                AttackCooldown {
                    remaining: Timer::from_seconds(1.5, TimerMode::Once),
                },
                AnimationTimer::new(Timer::from_seconds(
                    SKELETON_ANIMATION_TIME,
                    TimerMode::Repeating,
                )),
                AnimationFrameCount::new(6),
                Velocity::new(),
                Hurtbox {
                    size: Vec2::new(32., 32.), // Adjust as needed
                    offset: Vec2::ZERO,
                    colliding: Collision::default(),
                    entity: SKELETON,
                    iframe: Timer::from_seconds(0.75, TimerMode::Once),
                    enemy: true,
                },
            ));
        }
        EnemyT::Rock => {
            let rock_layout =
                TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE * 2), 2, 1, None, None);

            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("s_rock.png"),
                    transform,
                    ..default()
                },
                Rock {
                    //Setting default stats
                    current_hp: ROCK_MAX_HP,
                },
                TextureAtlas {
                    layout: texture_atlases.add(rock_layout.clone()),
                    index: 0,
                },
                AnimationTimer::new(Timer::from_seconds(
                    ROCK_ANIMATION_TIME,
                    TimerMode::Repeating,
                )),
                AnimationFrameCount::new(2),
                Velocity::new(),
                Hurtbox {
                    size: Vec2::splat(50.),
                    offset: Vec2::splat(0.),
                    colliding: Collision::default(),
                    entity: ROCK,
                    iframe: Timer::from_seconds(0.75, TimerMode::Once),
                    enemy: true,
                },
                Hitbox {
                    size: Vec2::splat(40.),
                    offset: Vec2::splat(0.),
                    lifetime: Some(Timer::from_seconds(1000000., TimerMode::Repeating)),
                    projectile: false,
                    entity: ROCK,
                    enemy: true,
                    dmg: 1.,
                },
            ));
        }

        EnemyT::PoisonSkeleton => {
            let pskeleton_layout = TextureAtlasLayout::from_grid(
                UVec2::new(31, 32), // SpriteSheet 1 pixel off, maybe fix later? it works like this though
                6,                  // Columns
                1,                  // Rows
                None,               // Padding
                None,               // Spacing
            );

            // Add the texture atlas to the resource

            // Spawn the skeleton entity
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("s_poison_skeleton.png"), // This uses the TextureAtlas handle
                    transform,
                    ..default()
                },
                Skeleton {
                    rotation_speed: 0.0,
                    current_hp: PSKELETON_MAX_HP,
                    max_hp: PSKELETON_MAX_HP,
                },
                TextureAtlas {
                    layout: texture_atlases.add(pskeleton_layout.clone()),
                    index: 0,
                },
                AttackCooldown {
                    remaining: Timer::from_seconds(1.5, TimerMode::Once),
                },
                AnimationTimer::new(Timer::from_seconds(
                    PSKELETON_ANIMATION_TIME,
                    TimerMode::Repeating,
                )),
                AnimationFrameCount::new(6),
                Velocity::new(),
                Hurtbox {
                    size: Vec2::new(32., 32.), // Adjust as needed
                    offset: Vec2::ZERO,
                    colliding: Collision::default(),
                    entity: PSKELETON,
                    iframe: Timer::from_seconds(0.75, TimerMode::Once),
                    enemy: true,
                },
            ));
        }
    }
}
