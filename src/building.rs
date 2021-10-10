use crate::{components::Position, traits::*};

#[derive(Default)]
pub struct Building {
    texture_index: usize,
    position: Position,
}

impl IsDrawable for Building {
    fn render_info(&self) -> (usize, &Position) {
        (self.texture_index, &self.position)
    }

    fn set_texture_index(&mut self, texture_index: usize) {
        self.texture_index = texture_index;
    }
}

impl HasPosition for Building {
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
