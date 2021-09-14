use std::collections::HashMap;

use macroquad::prelude::Texture2D;

use crate::{
    components::Position,
    quadtree::{Quadtree, AABB},
};

pub struct W {
    entities: HashMap<i32, Vec<Component>>,
    house_index: Quadtree,
    position_storage: Vec<Position>,
    sprite_storage: Vec<Texture2D>,
}

impl W {
    pub fn new(aabb: AABB) -> Self {
        Self {
            entities: HashMap::new(),
            house_index: Quadtree::new(aabb),
            position_storage: Vec::new(),
            sprite_storage: Vec::new(),
        }
    }
}

pub enum Component {
    Position,
    Sprite,
    Building,
    Business,
}
