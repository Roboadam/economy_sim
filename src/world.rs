use std::collections::HashMap;

use macroquad::prelude::Texture2D;

use crate::{
    components::Position,
    quadtree::{Quadtree, AABB},
};

pub struct W {
    next_index: i32,
    entities: HashMap<i32, HashMap<Component, usize>>,
    resturant_index: Quadtree,
    position_storage: Vec<Position>,
    sprite_storage: Vec<Texture2D>,
}

impl W {
    pub fn new(aabb: AABB) -> Self {
        Self {
            next_index: 0,
            entities: HashMap::new(),
            resturant_index: Quadtree::new(aabb),
            position_storage: Vec::new(),
            sprite_storage: Vec::new(),
        }
    }

    pub fn add_sprite_component(&mut self, sprite: Texture2D) -> usize {
        self.sprite_storage.push(sprite);
        self.sprite_storage.len() - 1
    }

    pub fn add_resturant_entity(&self, sprite: usize, position: Position) -> i32 {
        let entity_index = self.next_index;
        self.next_index += 1;

        let position_index = self.position_storage.len();
        self.position_storage.push(position);

        let entity = HashMap::new();
        entity.insert(Component::Position, position_index);

        self.entities.insert(entity_index, entity);
        entity_index
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum Component {
    Position,
    Sprite,
    Building,
    Business,
}
