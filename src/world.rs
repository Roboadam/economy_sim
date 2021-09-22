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
    ai_person_index: Vec<i32>,
}

impl W {
    pub fn new(aabb: &AABB) -> Self {
        Self {
            next_index: 0,
            entities: HashMap::new(),
            business_index: Quadtree::new(aabb),
            position_storage: Vec::new(),
            sprite_storage: Vec::new(),
            ai_person_index: Vec::new(),
        }
    }

    pub fn add_sprite_component(&mut self, sprite: Texture2D) -> usize {
        self.sprite_storage.push(sprite);
        self.sprite_storage.len() - 1
    }

    pub fn add_business_entity(&mut self, sprite: usize, position: Position) -> i32 {
        let entity = self.add_position_entity(sprite, position, Component::Business);
        self.business_index.insert(position, entity);
        entity
    }

    pub fn add_home_entity(&mut self, sprite: usize, position: Position) -> i32 {
        self.add_position_entity(sprite, position, Component::Home)
    }

    pub fn add_ai_person_entity(&mut self, sprite: usize, position: Position) -> i32 {
        let entity = self.add_position_entity(sprite, position, Component::AiPerson);
        self.ai_person_index.push(entity);
        entity
    }

    pub fn ai_positions_and_sprites(&self) -> Vec<(&Position, &Texture2D)> {
        self.ai_person_index
            .iter()
            .map(|entity| self.entities.get(entity))
            .filter(|option| option.is_some())
            .map(|option| {
                let components = option.expect("filtered out nones");
                let position_index = components.get(&Component::Position).unwrap();
                let sprite_index = components.get(&Component::Sprite).unwrap();
                let position = self.position_storage.get(*position_index).unwrap();
                let sprite = self.sprite_storage.get(*sprite_index).unwrap();
                (position, sprite)
            })
            .collect()
    }

    pub fn business_positions_and_sprites(&self, range: &AABB) -> Vec<(&Position, &Texture2D)> {
        self.business_index
            .query_range(range)
            .iter()
            .map(|(_, entity)| self.entities.get(entity))
            .filter(|option| option.is_some())
            .map(|option| {
                let components = option.expect("filtered out nones");
                let position_index = components.get(&Component::Position).unwrap();
                let sprite_index = components.get(&Component::Sprite).unwrap();
                let position = self.position_storage.get(*position_index).unwrap();
                let sprite = self.sprite_storage.get(*sprite_index).unwrap();
                (position, sprite)
            })
            .collect()
    }

    fn add_position_entity(
        &mut self,
        sprite: usize,
        position: Position,
        component: Component,
    ) -> i32 {
        let entity_index = self.next_index;
        self.next_index += 1;

        let position_index = self.position_storage.len();
        self.position_storage.push(position);

        let mut entity = HashMap::new();
        entity.insert(Component::Position, position_index);
        entity.insert(Component::Sprite, sprite);
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
    AiPerson,
}
