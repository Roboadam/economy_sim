use rand::prelude::ThreadRng;
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

    fn iter(&self) -> std::collections::hash_map::Iter<TileType, i32> {
        self.0.iter()
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

    fn collapse(&mut self, from: &Self, rules: &HashSet<Rule>) -> bool {
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
    Ok,
    Done,
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

    pub fn get(&self, x: i32, y: i32) -> Option<&SuperPosition> {
        if x < 0 || y < 0 {
            return None;
        }
        let x = x as usize;
        let y = y as usize;
        self.super_positions.get(y * self.width + x)
    }

    fn lowest_entropy_tile(&mut self) -> Option<(&mut SuperPosition, i32, i32)> {
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
                .map(|sp| (sp, lowest_x as i32, lowest_y as i32))
        } else {
            None
        }
    }

    fn apply_rules_once(&mut self, rules: &HashSet<Rule>) -> CollapseResult {
        let up_rules: Vec<Rule> = rules
            .iter()
            .filter(|rule| rule.direction == Direction::Up)
            .map(|rule| rule.clone())
            .collect();
        let down_rules: Vec<&Rule> = rules
            .iter()
            .filter(|rule| rule.direction == Direction::Down)
            .collect();
        let left_rules: Vec<&Rule> = rules
            .iter()
            .filter(|rule| rule.direction == Direction::Left)
            .collect();
        let right_rules: Vec<&Rule> = rules
            .iter()
            .filter(|rule| rule.direction == Direction::Right)
            .collect();
        let mut result = CollapseResult::Done;
        for y in 0..self.width as i32 {
            for x in 0..self.width as i32 {
                let up_tile = self.get(x, y - 1).clone();
                let down_tile = self.get(x, y + 1).clone();
                let left_tile = self.get(x - 1, y).clone();
                let right_tile = self.get(x + 1, y).clone();
                if let Some(tile) = self.get_mut(x, y) {}
            }
        }
        return CollapseResult::Ok;
    }

    fn collapse(&mut self, rng: &mut ThreadRng, rules: &HashSet<Rule>) -> CollapseResult {
        let up_rules: Vec<&Rule> = rules
            .iter()
            .filter(|rule| rule.direction == Direction::Up)
            .collect();
        let down_rules: Vec<&Rule> = rules
            .iter()
            .filter(|rule| rule.direction == Direction::Down)
            .collect();
        let left_rules: Vec<&Rule> = rules
            .iter()
            .filter(|rule| rule.direction == Direction::Left)
            .collect();
        let right_rules: Vec<&Rule> = rules
            .iter()
            .filter(|rule| rule.direction == Direction::Right)
            .collect();
        loop {
            if let Some((tile, x, y)) = self.lowest_entropy_tile() {
                let weight_sum: i32 = tile.iter().map(|value| *value.1).sum();
                let rand_num = rng.gen_range(0..weight_sum);
                tile.select_one(rand_num);
                if let Some(selected_type) = tile.first() {
                    if self.filter_neighbor(&up_rules, selected_type, x, y - 1)
                        == CollapseResult::Contradiction
                    {
                        return CollapseResult::Contradiction;
                    }
                    if self.filter_neighbor(&down_rules, selected_type, x, y + 1)
                        == CollapseResult::Contradiction
                    {
                        return CollapseResult::Contradiction;
                    }
                    if self.filter_neighbor(&left_rules, selected_type, x - 1, y)
                        == CollapseResult::Contradiction
                    {
                        return CollapseResult::Contradiction;
                    }
                    if self.filter_neighbor(&right_rules, selected_type, x + 1, y)
                        == CollapseResult::Contradiction
                    {
                        return CollapseResult::Contradiction;
                    }
                }
            } else {
                return CollapseResult::Done;
            }
        }
    }

    fn filter_neighbor(
        &mut self,
        rules: &Vec<&Rule>,
        selected_type: TileType,
        x: i32,
        y: i32,
    ) -> CollapseResult {
        let allowed_types: HashSet<TileType> = rules
            .iter()
            .filter(|rule| rule.from_tile_type == selected_type)
            .map(|rule| rule.to_tile_type)
            .collect();
        if let Some(tile) = self.get_mut(x, y) {
            let to_remove = tile.set_of_types().sub(&allowed_types);
            to_remove.into_iter().for_each(|type_to_remove| {
                tile.0.remove(&type_to_remove);
            });
            if tile.0.is_empty() {
                return CollapseResult::Contradiction;
            } else {
                return CollapseResult::Ok;
            }
        }
        CollapseResult::Ok
    }
}

pub fn collapse(input_tile_map: &TileMap, output_width: usize) -> TileMap {
    let (rule_set, super_position) = collect_rules_and_super_position(input_tile_map);
    loop {
        let mut super_position_map = SuperPositionMap::new(output_width, &super_position);
        let mut rng = thread_rng();
        let contradiction = super_position_map.collapse(&mut rng, &rule_set);
        if contradiction == CollapseResult::Contradiction {
            continue;
        }
        let mut result = TileMap::new(output_width);
        for y in 0..result.width as i32 {
            for x in 0..result.width as i32 {
                if let Some(super_position) = super_position_map.get(x, y) {
                    if let Some(tile_type) = super_position.first() {
                        result.set_tile(x, y, tile_type);
                    }
                }
            }
        }
        return result;
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
