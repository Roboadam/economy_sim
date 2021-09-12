use std::collections::HashMap;

use hecs::Entity;

pub struct OneToOne(HashMap<Entity, Entity>);

impl OneToOne {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(&mut self, one: Entity, two: Entity) -> Result<(), ()> {
        if self.0.contains_key(&one) {
            if *self.0.get(&one).unwrap() == two {
                return Ok(());
            } else {
                return Err(());
            }
        }
        if self.0.contains_key(&two) {
            return Err(());
        }

        self.0.insert(one, two);
        self.0.insert(two, one);
        Ok(())
    }

    pub fn get(&self, entity: Entity) -> Option<Entity> {
        self.0.get(&entity).map(|e| *e)
    }

    pub fn contains_key(&self, entity: Entity) -> bool {
        self.0.contains_key(&entity)
    }
}
