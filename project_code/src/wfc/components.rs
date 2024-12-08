use bevy::prelude::*;
use rand::Rng;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

use super::systems::find_spawn_points;

#[derive(Component)]
pub struct DebugPathMarker;

#[derive(Resource)]
pub struct DungeonTemplates {
    pub templates: Vec<Handle<Image>>,
    pub loaded: bool,
}

#[derive(Resource)]
pub struct DungeonTileSheet(
    pub Handle<Image>, 
    pub Handle<Image>, 
    pub Handle<Image>, 
    pub Handle<Image>, 
    pub Handle<TextureAtlasLayout>
);

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
pub struct Tile {
    pub tile_type: TileType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]

pub enum TileType {
    Wall,
    Ground,
    Void,
    Hole,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Pattern {
    width: usize,
    height: usize,
    pub data: Vec<TileType>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct EntropyCell {
    x: usize,
    y: usize,
    entropy: usize,
}

impl Ord for EntropyCell {
    fn cmp(&self, other: &Self) -> Ordering {
        other.entropy.cmp(&self.entropy)
    }
}

impl PartialOrd for EntropyCell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct WaveCell {
    possible_patterns: Vec<bool>,
    count: usize,
}

impl WaveCell {
    fn new(pattern_count: usize) -> Self {
        Self {
            possible_patterns: vec![true; pattern_count],
            count: pattern_count,
        }
    }

    fn remove(&mut self, pattern: usize) -> bool {
        if self.possible_patterns[pattern] {
            self.possible_patterns[pattern] = false;
            self.count -= 1;
            true
        } else {
            false
        }
    }
}
impl Clone for WaveCell {
    fn clone(&self) -> Self {
        Self {
            possible_patterns: self.possible_patterns.clone(),
            count: self.count,
        }
    }
}

impl Pattern {
    pub fn new(width: usize, height: usize) -> Self {
        Pattern {
            width,
            height,
            data: vec![TileType::Void; width * height],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<TileType> {
        if x < self.width && y < self.height {
            Some(self.data[y * self.width + x])
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, tile: TileType) {
        if x < self.width && y < self.height {
            self.data[y * self.width + x] = tile;
        }
    }

    pub fn get_rotations(&self) -> Vec<Pattern> {
        let mut rotations = Vec::new();
        rotations.push(self.clone());
        
        let mut current = self.clone();
        for _ in 0..3 {
            current = current.rotate_90();
            rotations.push(current.clone());
        }
        
        rotations
    }

    fn rotate_90(&self) -> Pattern {
        let mut rotated = Pattern::new(self.width, self.height);
        
        for y in 0..self.height {
            for x in 0..self.width {
                let tile = self.data[y * self.width + x];
                let new_x = self.height - 1 - y;
                let new_y = x;
                rotated.data[new_y * rotated.width + new_x] = tile;
            }
        }
        
        rotated
    }

    pub fn overlaps(&self, other: &Pattern, dx: isize, dy: isize) -> bool {
        let (x_range, y_range) = self.overlap_range(other, dx, dy);
        
        for y in y_range {
            for x in x_range.clone() {
                let self_tile = self.get(x as usize, y as usize);
                let other_x = (x - dx) as usize;
                let other_y = (y - dy) as usize;
                let other_tile = other.get(other_x, other_y);
                
                match (self_tile, other_tile) {
                    (Some(a), Some(b)) if a != b => return false,
                    _ => continue,
                }
            }
        }
        true
    }

    fn overlap_range(&self, other: &Pattern, dx: isize, dy: isize) -> (std::ops::Range<isize>, std::ops::Range<isize>) {
        let x_start = dx.max(0);
        let y_start = dy.max(0);
        let x_end = (self.width as isize).min(other.width as isize + dx);
        let y_end = (self.height as isize).min(other.height as isize + dy);
        (x_start..x_end, y_start..y_end)
    }
}

#[derive(Resource)]
pub struct WFCSettings {
    pub pattern_size: usize,
    pub output_width: usize,
    pub output_height: usize,
    pub spawn_area: (usize, usize), // bottom left corner coordinates
    pub door_area: (usize, usize),  // top right corner coordinates
}

impl Default for WFCSettings {
    fn default() -> Self {
        Self {
            pattern_size: 3,
            output_width: 100,
            output_height: 100,
            spawn_area: (3, 3),    // x,y coordinates for spawn area
            door_area: (97, 97),     // x,y coordinates for door area
        }
    }
}


#[derive(Resource, Clone)]
pub struct WFCState {
    pub patterns: Vec<Pattern>,
    pub weights: Vec<f32>,
    pub wave: Vec<Vec<WaveCell>>,
    pub entropy_heap: BinaryHeap<EntropyCell>,
    pub pattern_compatibility: Vec<Vec<Vec<usize>>>,
}

impl WFCState {
    pub fn new(patterns: Vec<Pattern>, weights: Vec<f32>) -> Self {
        let pattern_count = patterns.len();
        let mut compatibility = vec![vec![Vec::new(); 4]; pattern_count];
        
        // Build compatibility rules between patterns
        for i in 0..pattern_count {
            for j in 0..pattern_count {
                for (dir, (dx, dy)) in [(0, -1), (1, 0), (0, 1), (-1, 0)].iter().enumerate() {
                    if patterns[i].overlaps(&patterns[j], *dx, *dy) {
                        compatibility[i][dir].push(j);
                    }
                }
            }
        }

        Self {
            patterns,
            weights,
            wave: Vec::new(),
            entropy_heap: BinaryHeap::new(),
            pattern_compatibility: compatibility,
        }
    }

    pub fn initialize(&mut self, width: usize, height: usize) {
        self.wave = vec![vec![WaveCell::new(self.patterns.len()); width]; height];
        
        self.entropy_heap.clear();
        for y in 0..height {
            for x in 0..width {
                self.entropy_heap.push(EntropyCell {
                    x,
                    y,
                    entropy: self.patterns.len(),
                });
            }
        }
    }

    fn propagate(&mut self, start_x: usize, start_y: usize) {
        let mut stack = vec![(start_x, start_y)];
        let width = self.wave[0].len();
        let height = self.wave.len();
    
        while let Some((x, y)) = stack.pop() {
            // Get current valid patterns before modification
            let current_patterns: Vec<usize> = self.wave[y][x].possible_patterns.iter()
                .enumerate()
                .filter(|(_, &valid)| valid)
                .map(|(i, _)| i)
                .collect();
    
            // Only propagate if we have some valid patterns
            if !current_patterns.is_empty() {
                for (dir, (dx, dy)) in [(0, -1), (1, 0), (0, 1), (-1, 0)].iter().enumerate() {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    
                    if nx >= 0 && ny >= 0 && nx < width as i32 && ny < height as i32 {
                        let nx = nx as usize;
                        let ny = ny as usize;
                        
                        // Keep track of removed patterns
                        let mut removed = false;
                        let original_count = self.wave[ny][nx].count;
                        
                        // Only remove patterns that are incompatible with ALL current patterns
                        let mut compatible = vec![false; self.patterns.len()];
                        for &pattern in &current_patterns {
                            for &comp in &self.pattern_compatibility[pattern][dir] {
                                compatible[comp] = true;
                            }
                        }
    
                        for (i, &can_stay) in compatible.iter().enumerate() {
                            if !can_stay && self.wave[ny][nx].possible_patterns[i] {
                                self.wave[ny][nx].remove(i);
                                removed = true;
                            }
                        }
    
                        // Only propagate if we removed patterns and still have valid options
                        if removed && self.wave[ny][nx].count > 0 && self.wave[ny][nx].count < original_count {
                            stack.push((nx, ny));
                            self.entropy_heap.push(EntropyCell {
                                x: nx,
                                y: ny,
                                entropy: self.wave[ny][nx].count,
                            });
                        }
                    }
                }
            }
        }
    }
    pub fn collapse(&mut self) -> Option<(Vec<Vec<TileType>>, Vec2, Vec2, Vec2)> {
        let mut rng = rand::thread_rng();
        println!("Starting new collapse attempt");
    
        while let Some(EntropyCell { x, y, entropy }) = self.entropy_heap.pop() {
            if entropy == 0 {
                println!("Found cell with zero entropy");
                return None;
            }
            if entropy != self.wave[y][x].count {
                continue;
            }
            
            let valid_patterns: Vec<(usize, f32)> = self.wave[y][x].possible_patterns.iter()
                .enumerate()
                .filter(|(_, &valid)| valid)
                .map(|(i, _)| (i, self.weights[i]))
                .collect();
    
            if valid_patterns.is_empty() {
                return None;
            }
    
            let total_weight: f32 = valid_patterns.iter().map(|(_, w)| w).sum();
            let mut choice = rng.gen::<f32>() * total_weight;
            
            let chosen = valid_patterns.iter()
                .find(|(_, weight)| {
                    choice -= weight;
                    choice <= 0.0
                })
                .map(|(i, _)| *i)
                .unwrap_or_else(|| valid_patterns[0].0);
    
            let cell = &mut self.wave[y][x];
            for i in 0..self.patterns.len() {
                if i != chosen {
                    cell.remove(i);
                }
            }
    
            self.propagate(x, y);
        }
    
        let output = self.build_output();
        println!("Finding spawn points");
        if let Some((player_pos, entrance_pos, exit_pos)) = find_spawn_points(&output) {
            println!("Found valid spawn points");
            return Some((output, player_pos, entrance_pos, exit_pos));
        }
        println!("Failed to find valid spawn points");
        None
    }
    
    fn build_output(&self) -> Vec<Vec<TileType>> {
        let height = self.wave.len();
        let width = self.wave[0].len();
        let mut output = vec![vec![TileType::Void; width]; height];

        for y in 0..height {
            for x in 0..width {
                let cell = &self.wave[y][x];
                if let Some(pattern_idx) = cell.possible_patterns.iter()
                    .enumerate()
                    .find(|(_, &valid)| valid)
                    .map(|(i, _)| i) {
                    output[y][x] = self.patterns[pattern_idx].data[0];
                }
            }
        }

        output
    }
}