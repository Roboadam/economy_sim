use std::{
    collections::HashMap,
    usize,
};
use rand::prelude::ThreadRng;
use rand::thread_rng;

use crate::TileType;

#[derive(Clone)]
pub struct SuperPosition(HashMap<TileType, i32>);

impl SuperPosition {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get_mut(&mut self, tile_type: &TileType) -> Option<&mut i32> {
        self.0.get_mut(tile_type)
    }

    pub fn insert(&mut self, tile_type: TileType, value: i32) {
        self.0.insert(tile_type, value);
    }

    pub fn num_positions(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<TileType, i32> {
        self.0.iter()
    }

    pub fn select_one(&mut self, selector: i32) {
        let mut selected = TileType::Beach;
        let mut current_weight = 0;
        for (tile_type, weight) in self.0.iter() {
            current_weight += weight;
            if selector < current_weight {
                selected = tile_type.clone();
                break;
            }
        }
        self.0.clear();
        self.0.insert(selected, 0);
    }

    // shannon_entropy_for_square =
    //   log(sum(weight)) -
    //   (sum(weight * log(weight)) / sum(weight))
    // TODO: Test for this
    fn shannon_entropy(&self) -> f32 {
        let sum_weight: i32 = self.0.iter().map(|entry| entry.1).sum();
        let sum_weight = sum_weight as f32;
        let sum_weight_log_weight: f32 = self
            .0
            .iter()
            .map(|entry| (*entry.1 as f32) * (*entry.1 as f32).log2())
            .sum();
        sum_weight.log2() - (sum_weight_log_weight / sum_weight)
    }
}

struct SuperPositionMap {
    width: usize,
    super_positions: Vec<SuperPosition>,
    rng: ThreadRng,
}

impl SuperPositionMap {
    fn new(width: usize, super_position: &SuperPosition) -> Self {
        let rng = thread_rng();
        let mut super_positions = Vec::with_capacity(width * width);
        for _ in 0..width*width {
            super_positions.push(super_position.clone());
        }
        Self {
            width,
            super_positions,
            rng,
        }
    }
    
    fn get_mut(&mut self, x: i32, y:i32) -> Option<&mut SuperPosition> {
        if x < 0 || y < 0 {
            return None;
        }
        let x = x as usize;
        let y = y as usize;
        self.super_positions.get_mut(y * self.width + x)
    }

    fn lowest_entropy_tile(&mut self) -> Option<&mut SuperPosition> {
        let mut lowest_x: usize = 0;
        let mut lowest_y: usize = 0;
        let mut lowest_entropy = f32::INFINITY;
        let mut found_one = false;
    
        for y in 0..self.width {
            for x in 0..self.width {
                if let Some(super_position) = self.get_mut(x as i32, y as i32) {
                    if super_position.num_positions() > 1 {
                        let entropy = super_position.shannon_entropy();
                        if entropy < lowest_entropy {
                            lowest_entropy = entropy;
                            lowest_x = x;
                            lowest_y = y;
                            found_one = true;
                        }
                    }
                }
            }
        }
    
        if found_one {
            self.get_mut(lowest_x as i32, lowest_y as i32)
        } else {
            None
        }
    }

    fn collapse(&mut self) {
        loop {
            if let Some(tile) = self.lowest_entropy_tile() {
                // let weight_sum: i32 = tile.iter().map(|value| *value.1).sum();
                // let rand_num = self.rng.gen_range(0..weight_sum);
                tile.select_one(1);
            } else {
                return;
            }
        }
    }
}

// fn collapse(tiles_per_side: usize, super_position: &SuperPosition) -> TileMap {
//     let mut super_position_map = SuperPositionMap::new(tiles_per_side, super_position);

//     let tile_map = TileMap::new(tiles_per_side);
//     tile_map
// }
