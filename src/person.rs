use std::collections::HashMap;

pub struct Person {
    pub hunger: f32,
}

pub struct People(HashMap<i32, Person>);

impl People {
    pub fn get(&self, id: i32) -> &Person {
        self.0.get(&id).expect(&format!("Unknown person id {}", id))
    }

    pub fn get_mut(&mut self, id: i32) -> &mut Person {
        self.0
            .get_mut(&id)
            .expect(&format!("Unknown person id {}", id))
    }

    pub fn new() -> Self {
        People(HashMap::new())
    }

    pub fn add(&mut self, id: i32, person: Person) {
        self.0.insert(id, person);
    }
}

impl Person {
    pub fn buy(&mut self) {
        self.hunger += 1. / 3.;
    }
}
