use std::{
    collections::{HashMap, HashSet},
    usize,
};

use crate::{TileType, tile_map::TileMap};

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

    // shannon_entropy_for_square =
    //   log(sum(weight)) -
    //   (sum(weight * log(weight)) / sum(weight))
    // TODO: Test for this
    fn shannon_entropy_for_super_position(&self) -> f32 {
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

fn new_super_position_map(map_width: i32, super_position: &SuperPosition) -> Vec<SuperPosition> {
    let map_width = map_width as usize;
    let mut super_positions = Vec::with_capacity(map_width * map_width);
    for y in 0..map_width {
        for x in 0..map_width {
            super_positions.push(super_position.clone());
        }
    }
    super_positions
}

fn collapse(tiles_per_side: i32, super_position: &SuperPosition) -> TileMap {
    let tile_map = TileMap::new(tiles_per_side);
    tile_map
}

fn lowest_entropy_tile(super_position_map: &Vec<SuperPosition>) -> (i32, i32) {
    let mut lowest_x = 0;
    let mut lowest_y = 0;

    // super_position_map.as

    (lowest_x, lowest_y)
}
