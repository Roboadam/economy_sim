use rand::thread_rng;
use rand::Rng;
use std::collections::HashSet;
use std::ops::Sub;
use std::{collections::HashMap, usize};

use crate::rules::{Direction, Rule};
use crate::tile_map::TileMap;
use crate::TileType;

#[derive(Clone)]
struct SuperPosition(HashMap<TileType, i32>);

impl SuperPosition {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn first(&self) -> Option<TileType> {
        for (tile_type, _weight) in self.0.clone() {
            return Some(tile_type);
        }
        None
    }

    fn set_of_types(&self) -> HashSet<TileType> {
        let mut result: HashSet<TileType> = HashSet::new();
        for (tile_type, _) in self.0.clone() {
            result.insert(tile_type);
        }
        result
    }

    fn get_mut(&mut self, tile_type: &TileType) -> Option<&mut i32> {
        self.0.get_mut(tile_type)
    }

    fn insert(&mut self, tile_type: TileType, value: i32) {
        self.0.insert(tile_type, value);
    }

    fn num_positions(&self) -> usize {
        self.0.len()
    }

    fn select_one(&mut self, selector: i32) {
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

    fn len(&self) -> usize {
        self.0.len()
    }

    fn collapse(&mut self, from: &Self, rules: &Vec<Rule>) -> bool {
        let from_types = from.set_of_types();
        let my_types = self.set_of_types();
        let mut keepers = HashSet::new();
        for rule in rules {
            if from_types.contains(&rule.from_tile_type) && my_types.contains(&rule.to_tile_type) {
                keepers.insert(rule.to_tile_type);
            }
        }
        let to_remove = my_types.sub(&keepers);
        let changed = to_remove.len() > 0;
        for tile_type in to_remove {
            self.0.remove(&tile_type);
        }
        changed
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
}

#[derive(PartialEq, Eq)]
enum CollapseResult {
    Contradiction,
    Done,
    Changed,
    Unchanged,
}

impl SuperPositionMap {
    fn new(width: usize, super_position: &SuperPosition) -> Self {
        let mut super_positions = Vec::with_capacity(width * width);
        for _ in 0..width * width {
            super_positions.push(super_position.clone());
        }
        Self {
            width,
            super_positions,
        }
    }

    fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut SuperPosition> {
        if x < 0 || y < 0 {
            return None;
        }
        let x = x as usize;
        let y = y as usize;
        self.super_positions.get_mut(y * self.width + x)
    }

    pub fn get(&self, x: i32, y: i32) -> Option<SuperPosition> {
        if x < 0 || y < 0 {
            return None;
        }
        let x = x as usize;
        let y = y as usize;
        if let Some(tile) = self.super_positions.get(y * self.width + x) {
            return Some(tile.clone());
        }
        None
    }

    pub fn lowest_entropy_tile(&mut self) -> Option<&mut SuperPosition> {
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

    fn apply_rules_once(&mut self, up_rules: &Vec<Rule>, down_rules: &Vec<Rule>, left_rules: &Vec<Rule>, right_rules: &Vec<Rule>) -> CollapseResult {

        let mut result = CollapseResult::Unchanged;
        let mut is_done = true;
        for y in 0..self.width as i32 {
            for x in 0..self.width as i32 {
                if let Some(up_tile) = self.get(x, y - 1) {
                    let tile = self.get_mut(x, y).unwrap();
                    let changed = tile.collapse(&up_tile, up_rules);
                    if tile.len() == 0 {
                        return CollapseResult::Contradiction;
                    }
                    if changed {
                        result = CollapseResult::Changed;
                    }
                    if tile.len() > 1 {
                        is_done = false;
                    }
                }
                if let Some(down_tile) = self.get(x, y + 1) {
                    let tile = self.get_mut(x, y).unwrap();
                    let changed = tile.collapse(&down_tile, down_rules);
                    if tile.len() == 0 {
                        return CollapseResult::Contradiction;
                    }
                    if changed {
                        result = CollapseResult::Changed;
                    }
                    if tile.len() > 1 {
                        is_done = false;
                    }
                }
                if let Some(left_tile) = self.get(x - 1, y) {
                    let tile = self.get_mut(x, y).unwrap();
                    let changed = tile.collapse(&left_tile, left_rules);
                    if tile.len() == 0 {
                        return CollapseResult::Contradiction;
                    }
                    if changed {
                        result = CollapseResult::Changed;
                    }
                    if tile.len() > 1 {
                    is_done = false;
                    }
                }
                if let Some(right_tile) = self.get(x + 1, y) {
                    let tile = self.get_mut(x, y).unwrap();
                    let changed = tile.collapse(&right_tile, right_rules);
                    if tile.len() == 0 {
                        return CollapseResult::Contradiction;
                    }
                    if changed {
                        result = CollapseResult::Changed;
                    }
                    if tile.len() > 1 {
                        is_done = false;
                    }
                }
            }
        }
        if is_done {
            return CollapseResult::Done;
        }
        result
    }
}

fn collect_rules_and_super_position(tile_map: &TileMap) -> (HashSet<Rule>, SuperPosition) {
    let mut rule_set = HashSet::new();
    let mut super_position = SuperPosition::new();
    for y in 0..tile_map.width as i32 {
        for x in 0..tile_map.width as i32 {
            if let Some(current_tile) = tile_map.get_tile(x, y) {
                if let Some(value) = super_position.get_mut(current_tile) {
                    *value += 1;
                } else {
                    super_position.insert(current_tile.clone(), 1);
                }
                if let Some(up_tile) = tile_map.get_tile(x, y - 1) {
                    rule_set.insert(Rule {
                        from_tile_type: *current_tile,
                        to_tile_type: *up_tile,
                        direction: Direction::Up,
                    });
                }
                if let Some(down_tile) = tile_map.get_tile(x, y + 1) {
                    rule_set.insert(Rule {
                        from_tile_type: *current_tile,
                        to_tile_type: *down_tile,
                        direction: Direction::Down,
                    });
                }
                if let Some(left_tile) = tile_map.get_tile(x - 1, y) {
                    rule_set.insert(Rule {
                        from_tile_type: *current_tile,
                        to_tile_type: *left_tile,
                        direction: Direction::Left,
                    });
                }
                if let Some(right_tile) = tile_map.get_tile(x + 1, y) {
                    rule_set.insert(Rule {
                        from_tile_type: *current_tile,
                        to_tile_type: *right_tile,
                        direction: Direction::Right,
                    });
                }
            }
        }
    }
    (rule_set, super_position)
}

fn super_position_map_to_tile_map(super_position_map: &SuperPositionMap) -> TileMap {
    let mut result = TileMap::new(super_position_map.width);
    for y in 0..result.width as i32 {
        for x in 0..result.width as i32 {
            if let Some(super_position) = super_position_map.get(x, y) {
                if let Some(tile_type) = super_position.first() {
                    result.set_tile(x, y, tile_type);
                }
            }
        }
    }
    result
}

pub fn collapse(input: &TileMap, output_width: usize) -> TileMap {
    let mut rng = thread_rng();

    let (rules, super_position) = collect_rules_and_super_position(input);
    let up_rules: Vec<Rule> = rules
        .iter()
        .filter(|rule| rule.direction == Direction::Up)
        .map(|rule| rule.clone())
        .collect();
    let down_rules: Vec<Rule> = rules
        .iter()
        .filter(|rule| rule.direction == Direction::Down)
        .map(|rule| rule.clone())
        .collect();
    let left_rules: Vec<Rule> = rules
        .iter()
        .filter(|rule| rule.direction == Direction::Left)
        .map(|rule| rule.clone())
        .collect();
    let right_rules: Vec<Rule> = rules
        .iter()
        .filter(|rule| rule.direction == Direction::Right)
        .map(|rule| rule.clone())
        .collect();

    // Loop until there is no contradiction
    let mut collapse_result: CollapseResult;
    let mut num_selected = 0;
    loop {
        println!("Starting");
        let mut super_position_map = SuperPositionMap::new(output_width, &super_position);
        loop {
            if let Some(lowest_entropy) = super_position_map.lowest_entropy_tile() {
                let weight_sum = lowest_entropy.0.iter().map(|(_, weight)| weight).sum();
                let selector = rng.gen_range(0..weight_sum);
                num_selected += 1;
                println!("Selecting one {}", num_selected);
                lowest_entropy.select_one(selector);
                loop {
                    println!("Changed!");
                    collapse_result = super_position_map.apply_rules_once(&up_rules, &down_rules, &left_rules, &right_rules);
                    match collapse_result {
                        CollapseResult::Contradiction => break,
                        CollapseResult::Done => return super_position_map_to_tile_map(&super_position_map),
                        CollapseResult::Changed => {},
                        CollapseResult::Unchanged => break,
                    }
                }
            } else {
                break;
            }
            if collapse_result == CollapseResult::Contradiction {
                println!("Contradictdion!");
                break;
            }
        }
    }   
}
