use std::collections::HashMap;

use crate::components::Position;

pub struct Person {
    pub hunger: f32,
    pub position: Position,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct PersonId(pub i32);

pub struct People(HashMap<PersonId, Person>);

impl People {
    pub fn get(&self, id: PersonId) -> &Person {
        self.0
            .get(&id)
            .expect(&format!("Unknown person id {:?}", id))
    }

    pub fn get_mut(&mut self, id: PersonId) -> &mut Person {
        self.0
            .get_mut(&id)
            .expect(&format!("Unknown person id {:?}", id))
    }

    pub fn new() -> Self {
        People(HashMap::new())
    }

    pub fn add(&mut self, id: PersonId, person: Person) {
        self.0.insert(id, person);
    }
}
