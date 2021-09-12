#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
pub struct Hunger(pub f32);
pub enum TravelingTo {
    Nowhere,
    TravelPosition(Position),
}
pub struct AiPersonTag;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildingType {
    Resturant,
    Home,
}
