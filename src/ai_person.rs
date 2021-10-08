use crate::components::Position;

#[derive(Default)]
pub struct AiPerson {
    texture_index: usize,
    position: Position,
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

    fn set_traveling_to(&mut self, position: &Position) {
        self.traveling_to = Some(position.clone());
    }
}

pub trait IsDrawable {
    fn render_info(&self) -> (usize, &Position);
    fn set_texture_index(&mut self, texture_index: usize);
}

pub trait HasPosition {
    fn position(&self) -> &Position;
    fn set_position(&mut self, position: &Position);
    fn move_delta(&mut self, delta: &Position);
}

pub trait HasTravelingTo {
    fn traveling_to(&self) -> Option<&Position>;
    fn done_traveling(&mut self);
    fn set_traveling_to(&mut self, position: &Position);
}
