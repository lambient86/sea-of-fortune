use super::components::*;
use crate::components::*;
use crate::level::components::*;
use bevy::prelude::*;
use rand::Rng;
use crate::components::BoundingBox;

use crate::level::systems::*;
use crate::data::gameworld_data::*;
use crate::enemies::*;

#[derive(Resource)]
pub struct DungeonTemplates {
    pub templates: Vec<Handle<Image>>,
    pub loaded: bool,
}

pub fn init_wfc_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Load multiple template images
    let templates = vec![
        asset_server.load("template.png"),
        // asset_server.load("sewer_template.png"),
    ];
    
    commands.insert_resource(DungeonTemplates {
        templates,
        loaded: false,
    });
}
pub fn create_patterns_from_template(
    mut commands: Commands,
    mut dungeon_templates: ResMut<DungeonTemplates>,
    images: Res<Assets<Image>>,
    settings: Res<WFCSettings>,
) {
    println!("Attempting to create patterns");
    if dungeon_templates.loaded {
        println!("Templates already loaded, skipping");
        return;
    }

    // Check if all images are loaded
    for (i, handle) in dungeon_templates.templates.iter().enumerate() {
        println!("Template {}: loaded = {}", i, images.contains(handle));
    }

    println!("Number of templates: {}", dungeon_templates.templates.len());
    if let Some(template_handle) = dungeon_templates.templates.first() {
        if let Some(template_image) = images.get(template_handle) {
            println!("Template image loaded, size: {}x{}", 
                template_image.texture_descriptor.size.width,
                template_image.texture_descriptor.size.height);
            let patterns = extract_patterns(template_image, settings.pattern_size);
            println!("Extracted {} patterns", patterns.len());
            let weights = calculate_pattern_weights(&patterns);
            
            let wfc_state = WFCState::new(patterns, weights);
            commands.insert_resource(wfc_state);
            dungeon_templates.loaded = true;
        }
    }
}

fn calculate_pattern_weights(patterns: &[Pattern]) -> Vec<f32> {
    let mut pattern_counts: HashMap<&Pattern, f32> = HashMap::new();
    
    // Count occurrences
    for pattern in patterns {
        *pattern_counts.entry(pattern).or_insert(0.0) += 1.0;
    }
    
    // Calculate total for normalization
    let total_count: f32 = pattern_counts.values().sum();
    
    // Convert to vector of weights matching pattern order
    patterns.iter()
        .map(|pattern| pattern_counts.get(pattern).unwrap_or(&0.0) / total_count)
        .collect()
}

use std::collections::HashMap;

fn extract_patterns(image: &Image, pattern_size: usize) -> Vec<Pattern> {
    let mut all_patterns = Vec::new();
    let width = image.texture_descriptor.size.width as usize;
    let height = image.texture_descriptor.size.height as usize;
    
    // Step 1: Extract base patterns
    for y in 0..=height - pattern_size {
        for x in 0..=width - pattern_size {
            let mut pattern = Pattern::new(pattern_size, pattern_size);
            
            for py in 0..pattern_size {
                for px in 0..pattern_size {
                    let idx = ((y + py) * width + (x + px)) * 4;
                    if idx + 2 < image.data.len() {
                        let is_wall = image.data[idx] < 25 && 
                                    image.data[idx + 1] < 25 && 
                                    image.data[idx + 2] < 25;
                        pattern.set(px, py, if is_wall { TileType::Wall } else { TileType::Ground });
                    }
                }
            }
            all_patterns.push(pattern);
        }
    }

    // Step 2: Generate all rotations
    let mut patterns_with_rotations = Vec::new();
    for pattern in &all_patterns {
        patterns_with_rotations.extend(pattern.get_rotations());
    }

    // Step 3: Count pattern occurrences including rotations
    let mut pattern_counts: HashMap<Pattern, f32> = HashMap::new();
    for pattern in &patterns_with_rotations {
        *pattern_counts.entry(pattern.clone()).or_insert(0.0) += 1.0;
    }

    // Step 4: Convert to final vector with weights normalized
    let total_count: f32 = pattern_counts.values().sum();
    let mut final_patterns: Vec<Pattern> = pattern_counts.keys().cloned().collect();
    
    // Sort patterns to ensure consistent ordering
    final_patterns.sort_by(|a, b| {
        a.data.iter()
         .zip(b.data.iter())
         .find(|(a, b)| a != b)
         .map_or(std::cmp::Ordering::Equal, |(a, b)| a.cmp(b))
    });

    println!("Original patterns: {}", all_patterns.len());
    println!("Patterns with rotations: {}", patterns_with_rotations.len());
    println!("Final unique patterns: {}", final_patterns.len());

    final_patterns
}

pub fn load_dungeon(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Load all dungeon tilesets
    let dungeon_1_handle = asset_server.load("ts_dungeon_tiles_1.png");
    let dungeon_2_handle = asset_server.load("ts_dungeon_tiles_2.png");
    let dungeon_3_handle = asset_server.load("ts_dungeon_tiles_3.png");
    let dungeon_boss_handle = asset_server.load("ts_dungeon_tiles_4.png");
    
    let dungeon_layout = TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE * 2), 4, 1, None, None);
    let dungeon_layout_handle = texture_atlases.add(dungeon_layout);

    commands.insert_resource(DungeonTileSheet(
        dungeon_1_handle,
        dungeon_2_handle,
        dungeon_3_handle,
        dungeon_boss_handle,
        dungeon_layout_handle
    ));
}

fn spawn_dungeon_tiles(
    commands: &mut Commands,
    dungeon: &Vec<Vec<TileType>>,
    dungeon_tile_sheet: &Res<DungeonTileSheet>,
    current_island_type: &Res<CurrentIslandType>,
) {
    let height = dungeon.len();
    let width = dungeon[0].len();

    // Select correct tileset based on island type
    let texture_handle = match current_island_type.island_type {
        IslandType::Level1 => dungeon_tile_sheet.0.clone(),
        IslandType::Level2 => dungeon_tile_sheet.1.clone(),
        IslandType::Level3 => dungeon_tile_sheet.2.clone(),
        IslandType::Boss => dungeon_tile_sheet.3.clone(),
        IslandType::Start => dungeon_tile_sheet.0.clone(), // Fallback
    };

    let mut t = Vec3::new(
        -(width as f32) * TILE_SIZE as f32 + (TILE_SIZE * 2) as f32 / 2.,
        -(height as f32) * TILE_SIZE as f32 + (TILE_SIZE * 2) as f32 / 2.,
        -1.0,
    );

    for y in 0..height {
        for x in 0..width {
            let tile_type = dungeon[y][x];
            
            let mut entity = commands.spawn((
                SpriteBundle {
                    texture: texture_handle.clone(),
                    transform: Transform {
                        translation: t,
                        scale: Vec3::splat(1.0),
                        ..default()
                    },
                    ..default()
                },
                TextureAtlas {
                    layout: dungeon_tile_sheet.4.clone(), // Layout handle is now at index 4
                    index: match tile_type {
                        TileType::Wall => 0,
                        TileType::Ground => 1,
                        TileType::Void => 2,
                        TileType::Hole => 3,
                    },
                },
                Tile { tile_type },
            ));

            if tile_type == TileType::Wall {
                entity.insert((
                    Wall,
                    BoundingBox::new(
                        Vec2::new(t.x, t.y),
                        Vec2::new(TILE_SIZE as f32 * 0.8, TILE_SIZE as f32 * 0.8)
                    ),
                ));
            }
            
            t += Vec3::new((TILE_SIZE * 2) as f32, 0., 0.);
        }
        t += Vec3::new(0., (TILE_SIZE * 2) as f32, 0.);
        t.x = -(width as f32) * TILE_SIZE as f32 + (TILE_SIZE * 2) as f32 / 2.;
    }
}

fn add_outer_walls(grid: &mut Vec<TileType>, width: usize, height: usize) {
    // Add top and bottom walls
    for x in 0..width {
        grid[x] = TileType::Wall; // Top wall
        grid[(height-1) * width + x] = TileType::Wall; // Bottom wall
    }
    
    // Add left and right walls
    for y in 0..height {
        grid[y * width] = TileType::Wall; // Left wall
        grid[y * width + (width-1)] = TileType::Wall; // Right wall
    }
}

fn place_landmarks(
    grid: &mut Vec<TileType>, 
    width: usize,
    spawn_pos: (usize, usize),
    door_pos: (usize, usize)
) {
    // Place 5x5 spawn area
    for dy in -2..=2 {
        for dx in -2..=2 {
            let x = (spawn_pos.0 as isize + dx) as usize;
            let y = (spawn_pos.1 as isize + dy) as usize;
            if x > 0 && x < width-1 && y > 0 && y < width-1 {
                grid[y * width + x] = TileType::Ground;
            }
        }
    }
    
    // Place 5x5 door area
    for dy in -2..=2 {
        for dx in -2..=2 {
            let x = (door_pos.0 as isize + dx) as usize;
            let y = (door_pos.1 as isize + dy) as usize;
            if x > 0 && x < width-1 && y > 0 && y < width-1 {
                grid[y * width + x] = TileType::Ground;
            }
        }
    }
}

pub fn generate_dungeon(
    mut commands: Commands,
    mut wfc_state: Option<ResMut<WFCState>>,
    settings: Res<WFCSettings>,
    dungeon_tile_sheet: Res<DungeonTileSheet>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    current_island_type: Res<CurrentIslandType>,
) {
    if let Some(mut wfc_state) = wfc_state {
        for attempt in 0..20 {
            // First create guaranteed path between spawn and door
            let mut grid = vec![TileType::Wall; settings.output_width * settings.output_height];
            create_path(
                &mut grid, 
                settings.output_width, 
                settings.spawn_area, 
                settings.door_area,
                &mut commands,
                &asset_server,
                &mut texture_atlases
            );
            spawn_debug_path_markers(&mut commands, &grid, settings.output_width);

            // Store the path positions for later enemy spawning
            let path_positions: Vec<(usize, usize)> = grid.iter()
                .enumerate()
                .filter(|(_, &tile)| tile == TileType::Ground)
                .map(|(i, _)| (i % settings.output_width, i / settings.output_width))
                .collect();
            
            // Then run WFC on remaining tiles
            wfc_state.initialize(settings.output_width, settings.output_height);
            if let Some((mut dungeon, player_pos, _, door_pos)) = wfc_state.collapse() {
                // Merge the path with WFC generated dungeon
                for (i, tile) in grid.iter().enumerate() {
                    if *tile == TileType::Ground {
                        let y = i / settings.output_width;
                        let x = i % settings.output_width;
                        dungeon[y][x] = TileType::Ground;
                    }
                }
                
                // Convert 2D dungeon to 1D
                let mut final_grid: Vec<TileType> = dungeon.into_iter().flatten().collect();
                
                // Add outer walls and landmarks
                add_outer_walls(&mut final_grid, settings.output_width, settings.output_height);
                place_landmarks(&mut final_grid, settings.output_width, settings.spawn_area, settings.door_area);
                
                if ensure_connectivity(&mut final_grid, settings.output_width, settings.output_height, 
                                     settings.spawn_area, settings.door_area) {
                    // Convert back to 2D for rendering
                    let dungeon: Vec<Vec<TileType>> = final_grid.chunks(settings.output_width)
                        .map(|chunk| chunk.to_vec())
                        .collect();
                    
                    // Spawn the dungeon tiles
                    spawn_dungeon_tiles(&mut commands, &dungeon, &dungeon_tile_sheet);

                    // Now spawn enemies along the stored path positions
                    let mut rng = rand::thread_rng();
                    let mut steps = 0;

                    for (x, y) in path_positions {
                        steps += 1;
                        // Skip first 10 steps to avoid spawn area
                        if steps > 10 {
                            let world_x = -(settings.output_width as f32) * TILE_SIZE as f32 
                                + (x as f32 * TILE_SIZE as f32 * 2.0) + TILE_SIZE as f32;
                            let world_y = -(settings.output_width as f32) * TILE_SIZE as f32 
                                + (y as f32 * TILE_SIZE as f32 * 2.0) + TILE_SIZE as f32;
                            
                            let transform = Transform::from_xyz(world_x, world_y, 900.0)
                                .with_scale(Vec3::splat(2.0));

                            // Roll for each enemy type
                            if rng.gen_bool(0.01) { // 1% chance for skeleton
                                spawn_enemy(
                                    &mut commands,
                                    Enemy::Skeleton,
                                    transform,
                                    &asset_server,
                                    &mut texture_atlases,
                                );
                            } else if rng.gen_bool(0.01) { // 1% chance for bat
                                spawn_enemy(
                                    &mut commands,
                                    Enemy::Bat,
                                    transform,
                                    &asset_server,
                                    &mut texture_atlases,
                                );
                            } else if rng.gen_bool(0.0025) { // 0.25% chance for rock
                                spawn_enemy(
                                    &mut commands,
                                    Enemy::Rock,
                                    transform,
                                    &asset_server,
                                    &mut texture_atlases,
                                );
                            }
                        }
                    }
                    return;
                }
            }
        }
    }
}


fn create_path(
    grid: &mut Vec<TileType>,
    width: usize,
    start: (usize, usize),
    end: (usize, usize),
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut rng = rand::thread_rng();
    let mut current = start;
    
    while current != end {
        grid[current.1 * width + current.0] = TileType::Ground;
        
        // Path creation logic
        if current.0 != end.0 && current.1 != end.1 {
            if rng.gen_bool(0.5) {
                current.0 = if end.0 > current.0 { current.0 + 1 } else { current.0 - 1 };
            } else {
                current.1 = if end.1 > current.1 { current.1 + 1 } else { current.1 - 1 };
            }
        } else if current.0 != end.0 {
            current.0 = if end.0 > current.0 { current.0 + 1 } else { current.0 - 1 };
        } else {
            current.1 = if end.1 > current.1 { current.1 + 1 } else { current.1 - 1 };
        }
    }
    
    grid[end.1 * width + end.0] = TileType::Ground;
}

fn ensure_connectivity(
    grid: &mut Vec<TileType>,
    width: usize,
    height: usize,
    spawn_pos: (usize, usize),
    door_pos: (usize, usize)
) -> bool {
    let mut visited = vec![false; width * height];
    let mut stack = vec![spawn_pos];
    visited[spawn_pos.1 * width + spawn_pos.0] = true;

    // Define only cardinal directions (no diagonals)
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some(pos) = stack.pop() {
        if pos == door_pos {
            return true;
        }

        // Check cardinal neighbors only
        for (dx, dy) in &directions {
            let new_x = (pos.0 as isize + dx) as usize;
            let new_y = (pos.1 as isize + dy) as usize;
            
            if new_x < width && new_y < height {
                let idx = new_y * width + new_x;
                if !visited[idx] && grid[idx] == TileType::Ground {
                    visited[idx] = true;
                    stack.push((new_x, new_y));
                }
            }
        }
    }

    false
}

fn spawn_debug_path_markers(
    commands: &mut Commands,
    grid: &Vec<TileType>,
    width: usize,
) {
    let offset_x = -3200.0;
    let offset_y = -3200.0;

    for (i, tile) in grid.iter().enumerate() {
        if *tile == TileType::Ground {
            let x = offset_x + (i % width) as f32 * (TILE_SIZE * 2) as f32;
            let y = offset_y + (i / width) as f32 * (TILE_SIZE * 2) as f32;
            
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(0.0, 1.0, 0.0, 0.3),
                        custom_size: Some(Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32)),
                        ..default()
                    },
                    transform: Transform::from_xyz(x, y, 100.0),
                    ..default()
                },
                DebugPathMarker,
            ));
        }
    }
}

pub fn find_spawn_points(dungeon: &Vec<Vec<TileType>>) -> Option<(Vec2, Vec2, Vec2)> {
    // Calculate spawn position in bottom left 5x5 area
    let spawn_pos = Vec2::new(
        3.0 * TILE_SIZE as f32 * 2.0, // Center of 5x5 area
        3.0 * TILE_SIZE as f32 * 2.0
    );
    
    // Keep existing door position logic
    let door_pos = Vec2::new(
        95.0 * TILE_SIZE as f32 * 2.0,
        95.0 * TILE_SIZE as f32 * 2.0
    );

    Some((spawn_pos, door_pos, door_pos))
}

pub fn cleanup_debug_markers(
    mut commands: Commands,
    markers: Query<Entity, With<DebugPathMarker>>,
) {
    for entity in markers.iter() {
        commands.entity(entity).despawn();
    }
}
