#[derive(Debug, Clone, Copy)]
pub struct Position(pub f32, pub f32);
pub struct Hunger(pub f32);
pub enum TravelingTo {
    Nowhere,
    TravelPoint(i32)
}
pub struct AiPersonTag();
