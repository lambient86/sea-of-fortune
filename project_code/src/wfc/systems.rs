use bevy::prelude::*;
use rand::prelude::*;
use super::components::*;

impl WFCState {
    pub fn new(width: usize, height: usize, weights: &TileWeights) -> Self {
        let cells = vec![vec![None; width]; height];
        let all_types = vec![
            (TileType::Wall, weights.weights[&TileType::Wall]),
            (TileType::Ground, weights.weights[&TileType::Ground]),
            (TileType::Hole, weights.weights[&TileType::Hole]),
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
            let current_type = self.cells[y][x].unwrap();

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

pub fn setup_wfc(mut commands: Commands, weights: Res<TileWeights>) {
    
}