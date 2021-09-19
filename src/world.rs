use std::collections::HashMap;

use macroquad::prelude::Texture2D;

use crate::{
    components::Position,
    quadtree::{Quadtree, AABB},
};

pub struct W {
    next_index: i32,
    entities: HashMap<i32, HashMap<Component, usize>>,
    business_index: Quadtree,
    position_storage: Vec<Position>,
    sprite_storage: Vec<Texture2D>,
}

impl W {
    pub fn new(aabb: AABB) -> Self {
        Self {
            next_index: 0,
            entities: HashMap::new(),
            business_index: Quadtree::new(aabb),
            position_storage: Vec::new(),
            sprite_storage: Vec::new(),
        }
    }

    pub fn add_sprite_component(&mut self, sprite: Texture2D) -> usize {
        self.sprite_storage.push(sprite);
        self.sprite_storage.len() - 1
    }

    pub fn add_business_entity(&self, sprite: usize, position: Position) -> i32 {
        self.add_position_entity(sprite, position, Component::Business)
    }

    pub fn add_home_entity(&self, sprite: usize, position: Position) -> i32 {
        self.add_position_entity(sprite, position, Component::Home)
    }

    fn add_position_entity(&self, sprite: usize, position: Position, component: Component) -> i32 {
        let entity_index = self.next_index;
        self.next_index += 1;

        let position_index = self.position_storage.len();
        self.position_storage.push(position);

        let entity = HashMap::new();
        entity.insert(Component::Position, position_index);
        entity.insert(component, 0);

        self.entities.insert(entity_index, entity);
        entity_index
    }
}

#[derive(PartialEq, Eq, Hash)]
pub enum Component {
    Position,
    Sprite,
    Home,
    Business,
}
