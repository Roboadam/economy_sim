use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{ai_travel_point::AiTravelPoint, components::Position};

pub struct Person {
    pub hunger: f32,
    pub position: Position,
}

pub struct AiPerson {
    pub person: Person,
    pub travel_to: Option<i32>,
}

#[derive(PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct PersonId(pub i32);

pub struct People(HashMap<PersonId, Person>);

impl Person {
    pub fn update(&mut self, seconds: f32) {
        self.hunger -= seconds / 10.;
    }
}

impl AiPerson {
    pub fn update(&mut self, seconds: f32, travel_points: &Vec<AiTravelPoint>) {
        self.person.update(seconds);
        if let Some(index) = self.travel_to {
            if let Some(aitp) = travel_points.get(index as usize) {
                let dx = aitp.position.0 - self.person.position.0;
                let dy = aitp.position.1 - self.person.position.1;
                let len = (dx * dx + dy * dy).sqrt();
                self.person.position.0 += dx / (len * 70.);
                self.person.position.1 += dy / (len * 70.);
                if len < 0.1 {
                    self.travel_to = None;
                }
            }
        } else {
            let mut rng = thread_rng();
            let index = rng.gen_range(0..travel_points.len());
            println!("rand {}", index);
            self.travel_to = Some(index as i32);
        }
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
