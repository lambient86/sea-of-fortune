use bevy::prelude::*;
use rand::prelude::*;
use super::components::*;

use crate::level::systems::*;
use crate::data::gameworld_data::*;

impl WFCState {
    pub fn new(width: usize, height: usize, weights: &TileWeights) -> Self {
        let cells = vec![vec![None; width]; height];
        let all_types = vec![
            (TileType::Wall, weights.weights[&TileType::Wall]),
            (TileType::Ground, weights.weights[&TileType::Ground]),
            (TileType::Hole, weights.weights[&TileType::Hole]),
            (TileType::Void, weights.weights[&TileType::Void]),
        ];
        let entropy = vec![vec![all_types.clone(); width]; height];

        Self {
            width,
            height,
            cells,
            entropy,
        }
    }

    pub fn get_min_entropy_pos(&self) -> Option<(usize, usize)> {
        let mut min_entropy = usize::MAX;
        let mut min_pos = None;
        let mut rng = thread_rng();

        for y in 0..self.height {
            for x in 0..self.width {
                if self.cells[y][x].is_none() {
                    let entropy_size = self.entropy[y][x].len();
                    if entropy_size < min_entropy {
                        min_entropy = entropy_size;
                        min_pos = Some((x, y));
                    } else if entropy_size == min_entropy && rng.gen::<bool>() {
                        min_pos = Some((x, y));
                    }
                }
            }
        }

        min_pos
    }

    pub fn collapse_cell(&mut self, x: usize, y: usize) {
        if let Some(possible_types) = self.entropy.get(y).and_then(|row| row.get(x)) {
            if !possible_types.is_empty() {
                let mut rng = thread_rng();
                
                // Calculate total weight
                let total_weight: f32 = possible_types.iter().map(|(_, w)| w).sum();
                
                // Generate a random value between 0 and total weight
                let mut random_val = rng.gen::<f32>() * total_weight;
                
                // Select tile based on weights
                let selected = possible_types.iter().find(|(_, weight)| {
                    random_val -= weight;
                    random_val <= 0.0
                }).unwrap().0;

                self.cells[y][x] = Some(selected);
                self.entropy[y][x] = vec![(selected, 1.0)];
                self.propagate(x, y);
            }
        }
    }

    fn propagate(&mut self, start_x: usize, start_y: usize) {
        let mut stack = vec![(start_x, start_y)];

        while let Some((x, y)) = stack.pop() {
            let current_type = match self.cells[y][x] {
                Some(tile_type) => tile_type,
                None => continue,
            };

            // Check all neighbors (including diagonals)
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    let new_x = x as i32 + dx;
                    let new_y = y as i32 + dy;

                    if new_x >= 0 && new_x < self.width as i32 && 
                       new_y >= 0 && new_y < self.height as i32 {
                        let nx = new_x as usize;
                        let ny = new_y as usize;

                        if self.cells[ny][nx].is_none() {
                            let valid_types: Vec<(TileType, f32)> = self.entropy[ny][nx]
                                .iter()
                                .filter(|&(t, _)| VALID_NEIGHBORS.contains(&(current_type, *t)))
                                .copied()
                                .collect();

                            if valid_types.len() < self.entropy[ny][nx].len() {
                                self.entropy[ny][nx] = valid_types;
                                stack.push((nx, ny));
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn load_dungeon(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {

    // load dungeon tiles
    let bg_dungeon_texture_handle: Handle<Image> = asset_server.load("ts_dungeon_tiles_1.png");
    let dungeon_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE * 2), 4, 1, None, None);
    let dungeon_layout_handle = texture_atlases.add(dungeon_layout);

    // store tilesheets and handles
    commands.insert_resource(DungeonTileSheet(bg_dungeon_texture_handle, dungeon_layout_handle));
}

pub fn setup_wfc(mut commands: Commands, weights: Res<TileWeights>, dungeon_tile_sheet: Res<DungeonTileSheet>,) {

    let mut rng = rand::thread_rng();
    // let width = rng.gen_range(50..=100);
    // let height = rng.gen_range(50..=100);
    let width = 48;
    let height = 48;
    let mut wfc = WFCState::new(width, height, &weights);

    // Set some initial constraints (optional)
    // For example, setting borders as walls
    for y in 0..height {
        wfc.cells[y][0] = Some(TileType::Void);
        wfc.cells[y][width-1] = Some(TileType::Void);
    }

    for x in 0..width {
        wfc.cells[0][x] = Some(TileType::Void);
        wfc.cells[height-1][x] = Some(TileType::Void);
    }

    // Run the WFC algorithm
    while let Some((x, y)) = wfc.get_min_entropy_pos() {
        wfc.collapse_cell(x, y);
    }

    let mut t = Vec3::new(
        -(width as f32) * TILE_SIZE as f32 + (TILE_SIZE * 2) as f32 / 2.,
        -(height as f32) * TILE_SIZE as f32 + (TILE_SIZE * 2) as f32 / 2.,
        -1.0,
        );

    for y in 0..wfc.height {
        for x in 0..wfc.width {
            if let Some(tile_type) = wfc.cells[y][x] {
                commands.spawn((
                    SpriteBundle {
                        texture: dungeon_tile_sheet.0.clone(),
                        transform: Transform {
                            translation: t,
                            ..default()
                        },
                        ..default()
                    },
                    TextureAtlas {
                        layout: dungeon_tile_sheet.1.clone(),
                        index: match tile_type {
                            TileType::Wall => 0,
                            TileType::Ground => 1,
                            TileType::Void => 2,
                            TileType::Hole => 3,
                        },
                    },
                    Tile {tile_type: tile_type},
                ));
                t += Vec3::new((TILE_SIZE * 2) as f32, 0., 0.);
            }
        }
        t += Vec3::new(0., (TILE_SIZE * 2) as f32, 0.);
        t.x = -(width as f32) * TILE_SIZE as f32 + (TILE_SIZE * 2) as f32 / 2.;
    }
}
