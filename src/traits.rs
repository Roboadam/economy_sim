use crate::components::Position;

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
