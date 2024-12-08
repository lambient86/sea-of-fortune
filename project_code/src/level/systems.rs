use crate::components::{BoundingBox, GameworldState};
use crate::level::components::*;
use bevy::prelude::*;

use crate::data::gameworld_data::*;

use rand::Rng;

pub fn load_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    // load ocean tile sheet
    let bg_ocean_texture_handle: Handle<Image> = asset_server.load("ts_ocean_tiles.png");
    let ocean_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE * 2), 2, 1, None, None);
    let ocean_layout_handle = texture_atlases.add(ocean_layout);

    // load island sand tile sheet
    let bg_sand_texture_handle: Handle<Image> = asset_server.load("ts_sand_tiles.png");
    let sand_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE * 2), 3, 1, None, None);
    let sand_layout_handle = texture_atlases.add(sand_layout);

    // island
    let s_island_handle: Handle<Image> = asset_server.load("s_island.png");

    // dungeon
    let s_dungeon_1_handle: Handle<Image> = asset_server.load("s_dungeon1.png");
    let s_dungeon_2_handle: Handle<Image> = asset_server.load("s_dungeon2.png");
    let s_dungeon_3_handle: Handle<Image> = asset_server.load("s_dungeon3.png");
    let s_dungeon_b_handle: Handle<Image> = asset_server.load("s_dungeon4.png");

    // ocean door
    let s_ocean_door_handle: Handle<Image> = asset_server.load("s_ocean_door.png");

    // store tilesheets and handles
    commands.insert_resource(OceanTileSheet(bg_ocean_texture_handle, ocean_layout_handle));
    commands.insert_resource(SandTileSheet(bg_sand_texture_handle, sand_layout_handle));
    commands.insert_resource(IslandTileSheet(s_island_handle));
    commands.insert_resource(DungeonSheet(
        s_dungeon_1_handle,
        s_dungeon_2_handle,
        s_dungeon_3_handle,
        s_dungeon_b_handle,
    ));
    commands.insert_resource(OceanDoorHandle(s_ocean_door_handle));
}

pub fn setup_ocean(
    mut commands: Commands,
    ocean_tile_sheet: Res<OceanTileSheet>,
    game_world_state: Res<State<GameworldState>>,
    island_tile_sheet: Res<IslandTileSheet>,
) {
    let mut rng = rand::thread_rng();
    let mut tile_index;

    if *game_world_state.get() == GameworldState::Ocean {
        // current state --> ocean

        //

        let mut w = 0;
        let mut h = 0;
        let mut t = Vec3::new(
            -OCEAN_W_CENTER + TILE_SIZE as f32 / 2.,
            -OCEAN_H_CENTER + TILE_SIZE as f32 / 2.,
            -1.0,
        );

        // spawn background tiles
        while (h as f32) * (TILE_SIZE as f32) < OCEAN_LEVEL_H {
            while (w as f32) * (TILE_SIZE as f32) < OCEAN_LEVEL_W {
                // weigh it so that its mostly dark blue just for aesthetic reasons
                let rand = rng.gen_range(0..=10);
                if rand < 9 {
                    tile_index = 0
                } else {
                    tile_index = 1
                }

                commands
                    .spawn((
                        SpriteBundle {
                            texture: ocean_tile_sheet.0.clone(),
                            transform: Transform {
                                translation: t,
                                ..default()
                            },
                            ..default()
                        },
                        TextureAtlas {
                            layout: ocean_tile_sheet.1.clone(),
                            index: tile_index,
                        },
                        OceanTile,
                    ))
                    .insert(OceanTile);

                w += 1;
                t += Vec3::new((TILE_SIZE * 2) as f32, 0., 0.);
            }

            w = 0;
            t += Vec3::new(0., (TILE_SIZE * 2) as f32, 0.);
            t.x = -OCEAN_W_CENTER + (TILE_SIZE * 2) as f32 / 2.0;
            h += 1;
        }

        //spawn 4 islands
        /*
           we need 4 islands --> one for each difficulty level
           split ocean into 4 zones --> 1 island in each zone
           we need to have collision detection on each island
            - if the ship collides we transition to the island
        */

        let zone_size = Vec2::new(OCEAN_LEVEL_W / 4.0, OCEAN_LEVEL_H / 4.0);
        let mut zone_count = 0.0;
        let mut island_type = IslandType::Level1;

        // loop through each zone
        while zone_count < 4.0 {
            match zone_count {
                1.0 => island_type = IslandType::Level2,
                2.0 => island_type = IslandType::Level3,
                3.0 => island_type = IslandType::Boss,
                _ => island_type = IslandType::Boss,
            }

            let rand_x = rng.gen_range(-OCEAN_W_CENTER + 64.0..OCEAN_W_CENTER - 64.0);

            // get random y within range
            let rand_y = rng.gen_range(
                -OCEAN_H_CENTER + (zone_count * zone_size.y)
                    ..(-OCEAN_H_CENTER + ((zone_count * zone_size.y) + zone_size.y)) - 128.0,
            );

            let rand_position = Vec2::new(rand_x, rand_y);

            println!("spawning island at {}, {}", rand_x, rand_y);

            commands.spawn((
                SpriteBundle {
                    texture: island_tile_sheet.0.clone(),
                    transform: Transform::from_xyz(rand_x, rand_y, 10.0),
                    ..default()
                },
                Island {
                    aabb: BoundingBox::new(rand_position, Vec2::splat(64.0)),
                    island_type,
                },
            ));

            zone_count += 1.0;
        }
    }
}

pub fn setup_island(
    mut commands: Commands,
    game_world_state: Res<State<GameworldState>>, // get the current gameworld state
    sand_tile_sheet: Res<SandTileSheet>,
    island_query: Query<&Island, With<Island>>,
    dungeon_tile_sheet: Res<DungeonSheet>,
    ocean_door: Res<OceanDoorHandle>,
) {
    if *game_world_state.get() == GameworldState::Island {
        let mut rng = rand::thread_rng();
        let mut tile_index;

        let mut w = 0;
        let mut h = 0;
        let mut t = Vec3::new(
            -SAND_W_CENTER + TILE_SIZE as f32 / 2.,
            -SAND_H_CENTER + TILE_SIZE as f32 / 2.,
            -1.0,
        );

        while (h as f32) * (TILE_SIZE as f32) < SAND_LEVEL_H {
            while (w as f32) * (TILE_SIZE as f32) < SAND_LEVEL_W {
                let rand = rng.gen_range(0..=10);
                if rand < 4 {
                    tile_index = 0
                } else if rand >= 4 && rand <= 7 {
                    tile_index = 1
                } else {
                    tile_index = 2
                }

                commands
                    .spawn((
                        SpriteBundle {
                            texture: sand_tile_sheet.0.clone(),
                            transform: Transform {
                                translation: t,
                                ..default()
                            },
                            ..default()
                        },
                        TextureAtlas {
                            layout: sand_tile_sheet.1.clone(),
                            index: tile_index,
                        },
                        SandTile,
                    ))
                    .insert(SandTile);

                w += 1;
                t += Vec3::new((TILE_SIZE * 2) as f32, 0., 0.);
            }

            w = 0;
            t += Vec3::new(0., (TILE_SIZE * 2) as f32, 0.);
            t.x = -SAND_W_CENTER + (TILE_SIZE * 2) as f32 / 2.0;
            h += 1;
        }

        commands.spawn((
            SpriteBundle {
                texture: ocean_door.0.clone(),
                transform: Transform {
                    translation: Vec3::new(-400., 0., 10.0),
                    ..default()
                },
                ..default()
            },
            OceanDoor {
                aabb: BoundingBox::new(Vec3::new(-400., 0., 10.).truncate(), Vec2::splat(64.0)),
            },
        ));

        // get the current island type
        let mut curr_dungeon: Handle<Image> = dungeon_tile_sheet.0.clone();
        let mut curr_dungeon_type = IslandType::Level1;
        for island in island_query.iter() {
            match island.island_type {
                IslandType::Level1 => {
                    curr_dungeon = dungeon_tile_sheet.0.clone();
                    curr_dungeon_type = IslandType::Level1;
                }
                IslandType::Level2 => {
                    curr_dungeon = dungeon_tile_sheet.1.clone();
                    curr_dungeon_type = IslandType::Level2;
                }
                IslandType::Level3 => {
                    curr_dungeon = dungeon_tile_sheet.2.clone();
                    curr_dungeon_type = IslandType::Level3;
                }
                IslandType::Boss => {
                    curr_dungeon = dungeon_tile_sheet.3.clone();
                    curr_dungeon_type = IslandType::Boss;
                }
                _ => {
                    curr_dungeon = dungeon_tile_sheet.0.clone();
                    curr_dungeon_type = IslandType::Level1;
                }
            }
        }

        // spawn the according dungeon gate
        commands.spawn((
            SpriteBundle {
                texture: curr_dungeon,
                transform: Transform {
                    translation: Vec3::new(0., 256., 10.),
                    ..default()
                },
                ..default()
            },
            Dungeon {
                aabb: BoundingBox::new(Vec3::new(0., 256., 10.).truncate(), Vec2::splat(64.0)),
                dungeon_type: curr_dungeon_type,
                size: Vec2::splat(64.0),
            },
        ));
    }
}

pub fn despawn_with<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}
