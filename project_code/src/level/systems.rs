
use bevy::prelude::*;
use crate::components::GameworldState;
use crate::level::components::*;

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

    // load dungeon tile sheet
    let bg_dungeon_texture_handle: Handle<Image> = asset_server.load("ts_dungeon_tiles.png");
    let dungeon_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 2, 1, None, None);
    let dungeon_layout_handle = texture_atlases.add(dungeon_layout);

    // store tilesheets and handles
    commands.insert_resource(OceanTileSheet(bg_ocean_texture_handle, ocean_layout_handle));
    commands.insert_resource(SandTileSheet(bg_sand_texture_handle, sand_layout_handle));
    commands.insert_resource(DungeonTileSheet(bg_dungeon_texture_handle, dungeon_layout_handle));
}

pub fn setup_level(
    mut commands: Commands,
    ocean_tile_sheet: Res<OceanTileSheet>,
    sand_tile_sheet: Res<SandTileSheet>,
    dungeon_tile_sheet: Res<DungeonTileSheet>,
    game_world_state: Res<State<GameworldState>>,           // get the current gameworld state
) {

    let mut rng = rand::thread_rng();
    let mut tile_index;

    if *game_world_state.get() == GameworldState::Ocean {          // current state --> ocean

        let mut w = 0;
        let mut h = 0;
        let mut t = Vec3::new(
        -OCEAN_W_CENTER + TILE_SIZE as f32 / 2.,
        -OCEAN_H_CENTER + TILE_SIZE as f32 / 2.,
        -1.0,
        );

        while (h as f32) * (TILE_SIZE as f32) < OCEAN_LEVEL_H {
            while (w as f32) * (TILE_SIZE as f32) < OCEAN_LEVEL_W {

                // weigh it so that its mostly dark blue just for aesthetic reasons
                let rand = rng.gen_range(0..=10);
                if rand < 9 {tile_index = 0}
                else {tile_index = 1}
                
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
            t.x = -OCEAN_W_CENTER + (TILE_SIZE * 2) as f32/2.0;
            h+=1;
        }

    } else if *game_world_state.get() == GameworldState::Island {   // current state --> island

        let mut w = 0;
        let mut h = 0;
        let mut t = Vec3::new(
        -SAND_W_CENTER + TILE_SIZE as f32 / 2.,
        -SAND_H_CENTER + TILE_SIZE as f32 / 2.,
        -1.0,
        );

        while (h as f32) * (TILE_SIZE as f32) < SAND_LEVEL_H {
            while (w as f32) * (TILE_SIZE as f32) < SAND_LEVEL_W {

                // weigh it so that its mostly dark blue just for aesthetic reasons
                let rand = rng.gen_range(0..=10);
                if rand < 4 {tile_index = 0}
                else if rand >= 4 && rand <=7 {tile_index = 1}
                else {tile_index = 2}
                
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
            t.x = -SAND_W_CENTER + (TILE_SIZE * 2) as f32/2.0;
            h+=1;
        }
    }  else if *game_world_state.get() == GameworldState::Dungeon {     // current state --> dungeon

        let mut w = 0;
        let mut h = 0;
        let mut t = Vec3::new(
        -DUNGEON_W_CENTER + TILE_SIZE as f32 / 2.,
        -DUNGEON_H_CENTER + TILE_SIZE as f32 / 2.,
        -1.0,
        );

        while (h as f32) * (TILE_SIZE as f32) < DUNGEON_LEVEL_H {
            while (w as f32) * (TILE_SIZE as f32) < DUNGEON_LEVEL_W {

                // weigh it so that its mostly dark blue just for aesthetic reasons
                let rand = rng.gen_range(0..=10);
                if rand < 8 {tile_index = 0}
                else {tile_index = 1}
                
                commands
                    .spawn((
                        SpriteBundle {
                            texture: dungeon_tile_sheet.0.clone(),
                            transform: Transform {
                                translation: t,
                                scale: Vec3::splat(2.),
                                ..default()
                            },
                            ..default()
                        },
                        TextureAtlas {
                            layout: dungeon_tile_sheet.1.clone(),
                            index: tile_index,
                        },
                        DungeonTile,
                    ))
                    .insert(DungeonTile);
    
                w += 1;
                t += Vec3::new((TILE_SIZE) as f32, 0., 0.);
            }

            w = 0;
            t += Vec3::new(0., (TILE_SIZE) as f32, 0.);
            t.x = -DUNGEON_W_CENTER + (TILE_SIZE) as f32/2.0;
            h+=1;
        }
    }

}

pub fn despawn_with<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}