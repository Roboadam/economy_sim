#[derive(Debug, Clone, Copy, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub enum TravelingTo {
    Nowhere,
    TravelPosition(Position),
}
