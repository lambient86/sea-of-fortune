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
    let bg_ocean_texture_handle: Handle<Image> = asset_server.load("bg_ocean_tiles.png");
    let ocean_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 2, 1, None, None);
    let ocean_layout_handle = texture_atlases.add(ocean_layout);

    // load island sand tile sheet
    let bg_sand_texture_handle: Handle<Image> = asset_server.load("bg_sand_tiles.png");
    let sand_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), 2, 1, None, None);
    let sand_layout_handle = texture_atlases.add(sand_layout);

    // store tilesheets and handles
    commands.insert_resource(BGTileSheet(bg_ocean_texture_handle, ocean_layout_handle));
    commands.insert_resource(BGTileSheet(bg_sand_texture_handle, sand_layout_handle));
}

pub fn setup_level(
    mut commands: Commands,
    texture_atlases: Res<Assets<TextureAtlasLayout>>,
    tile_sheet: Res<BGTileSheet>,
    game_world_state: Res<State<GameworldState>>,           // get the current gameworld state
) {

    if *game_world_state.get() == GameworldState::Ocean {          // current state --> ocean

    } else if *game_world_state.get() == GameworldState::Island {
        
    }

//     let brick_layout = texture_atlases.get(&brick_sheet.1);
//     let brick_layout_len = brick_layout.unwrap().len();
//     let mut i = 0;
//     let mut t = Vec3::new(
//         -WIN_W / 2. + TILE_SIZE / 2.,
//         -WIN_H / 2. + TILE_SIZE / 2.,
//         1.,
//     );
//     while (i as f32) * TILE_SIZE < LEVEL_LEN {
//         commands
//             .spawn((
//                 SpriteBundle {
//                     texture: brick_sheet.0.clone(),
//                     transform: Transform {
//                         translation: t,
//                         ..default()
//                     },
//                     ..default()
//                 },
//                 TextureAtlas {
//                     layout: brick_sheet.1.clone(),
//                     index: i % brick_layout_len,
//                 },
//                 Brick,
//             ))
//             .insert(Brick);

//         i += 1;
//         t += Vec3::new(TILE_SIZE, 0., 0.);
//     }
}

pub fn despawn_with<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}