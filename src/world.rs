use std::collections::HashMap;

use macroquad::prelude::Texture2D;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use crate::{
    components::{Position, TravelingTo},
    quadtree::{Quadtree, AABB},
};

pub struct W {
    next_index: i32,
    entities: HashMap<i32, HashMap<Component, usize>>,
    business_index: Quadtree,
    position_storage: Vec<Position>,
    sprite_storage: Vec<Texture2D>,
    traveling_to_storage: Vec<TravelingTo>,
    ai_person_index: Vec<i32>,
    pub rng: ChaCha8Rng,
}

impl W {
    pub fn new(aabb: &AABB, seed: &u64) -> Self {
        Self {
            next_index: 0,
            entities: HashMap::new(),
            business_index: Quadtree::new(aabb),
            position_storage: Vec::new(),
            sprite_storage: Vec::new(),
            traveling_to_storage: Vec::new(),
            ai_person_index: Vec::new(),
            rng: ChaCha8Rng::seed_from_u64(*seed),
        }
    }

    pub fn update_traveling_to(&mut self, entity_id: i32, new_traveling_to: TravelingTo) {
        if let Some(components) = self.entities.get(&entity_id) {
            if let Some(index) = components.get(&Component::TravelingTo) {
                if self.traveling_to_storage.len() > *index {
                    self.traveling_to_storage[*index] = new_traveling_to;
                }
            }
        }
    }

    pub fn position_for_entity_id(&self, entity_id: i32) -> Option<Position> {
        let position_index = self.entities.get(&entity_id)?.get(&Component::Position)?;
        self.position_storage
            .get(*position_index)
            .map(|p| p.clone())
    }

    pub fn traveling_to_for_entity_id(&self, entity_id: i32) -> Option<&TravelingTo> {
        let index = self
            .entities
            .get(&entity_id)?
            .get(&Component::TravelingTo)?;
        self.traveling_to_storage.get(*index)
    }

    pub fn add_sprite_component(&mut self, sprite: Texture2D) -> usize {
        self.sprite_storage.push(sprite);
        self.sprite_storage.len() - 1
    }

    pub fn add_business_entity(&mut self, sprite: usize, position: Position) -> i32 {
        let entity = self.add_position_entity(sprite, position, Component::Business);
        let _ = self.business_index.insert(position, entity);
        entity
    }

    pub fn add_home_entity(&mut self, sprite: usize, position: Position) -> i32 {
        self.add_position_entity(sprite, position, Component::Home)
    }

    pub fn add_ai_person_entity(&mut self, sprite: usize, position: Position) -> i32 {
        let entity = self.add_position_entity(sprite, position, Component::AiPerson);
        let traveling_to_index = self.traveling_to_storage.len();
        self.entities[&entity].insert(Component::TravelingTo, traveling_to_index);
        self.ai_person_index.push(entity);
        entity
    }

    pub fn traveling_to_and_positions(&self) -> Vec<i32> {
        self.entities
            .iter()
            .filter(|(_, components)| {
                components.contains_key(&Component::TravelingTo)
                    && components.contains_key(&Component::Position)
            })
            .map(|(entity_id, _)| *entity_id)
            .collect()
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

    pub fn business_positions(&self, range: &AABB) -> Vec<Position> {
        self.business_index
            .query_range(range)
            .iter()
            .map(|(position, _)| position.clone())
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
    TravelingTo,
}
