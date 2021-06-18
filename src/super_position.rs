use std::{collections::HashSet, usize};

use crate::TileType;

#[derive(Clone)]
struct SuperPosition {
    possible_values: HashSet<TileType>,
}

impl SuperPosition {
    fn new() -> Self {
        let mut possible_values = HashSet::new();
        
        for value in TileType::iter() {
            possible_values.insert(value);
        }

        Self {
            possible_values,
        }
    }

    fn collapse(&mut self) {

    }
}

fn new_super_position_map(map_width: i32) -> Vec<SuperPosition> {
    let map_width = map_width as usize;
    let mut super_positions = Vec::with_capacity(map_width * map_width);
    let all_super_position = SuperPosition::new();
    for y in 0..map_width {
        for x in 0..map_width {
            super_positions.push(all_super_position.clone());
        }
    }
    super_positions
}