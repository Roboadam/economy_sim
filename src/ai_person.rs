use crate::{components::Position, traits::*};

#[derive(Default)]
pub struct AiPerson {
    texture_index: usize,
    position: Position,
    hunger: f32,
    traveling_to: Option<Position>,
}

impl IsDrawable for AiPerson {
    fn render_info(&self) -> (usize, &Position) {
        (self.texture_index, &self.position)
    }

    fn set_texture_index(&mut self, texture_index: usize) {
        self.texture_index = texture_index;
    }
}

impl NeedsFood for AiPerson {
    fn eat(&mut self, food: f32) -> f32 {
        if self.hunger <= 0. {
            return food;
        }
        self.hunger -= food;
        if self.hunger < 0. {
            let leftovers = -self.hunger;
            self.hunger = 0.;
            return leftovers;
        }
        0.
    }

    fn burn_calories(&mut self, burn: f32) {
        self.hunger += burn;
    }

    fn hunger_index(&self) -> f32 {
        self.hunger
    }
}

impl HasPosition for AiPerson {
    fn position(&self) -> &Position {
        &self.position
    }

    fn set_position(&mut self, position: &Position) {
        self.position = *position;
    }

    fn move_delta(&mut self, delta: &Position) {
        self.position.x += delta.x;
        self.position.y += delta.y;
    }
}

impl HasTravelingTo for AiPerson {
    fn traveling_to(&self) -> Option<&Position> {
        (&self.traveling_to).as_ref()
    }

    fn done_traveling(&mut self) {
        self.traveling_to = None;
    }

    fn set_traveling_to(&mut self, position: &Position, seconds: f32) {
        self.traveling_to = Some(position.clone());
        self.burn_calories(seconds);
    }
}
