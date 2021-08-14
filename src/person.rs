use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct Person {
    pub hunger: f32,
    pub position: (f32, f32),
}

#[derive(PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct PersonId(pub i32);

pub struct People(HashMap<PersonId, Person>);

impl Person {
    pub fn update(&mut self, seconds: f32) {
        self.hunger -= seconds / 10.;
    }
}

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

    pub fn update(&mut self, seconds: f32) {
        self.0
            .iter_mut()
            .for_each(|(_, person)| person.update(seconds));
    }
}

impl Person {
    pub fn buy(&mut self) {
        self.hunger += 1. / 3.;
    }
}
