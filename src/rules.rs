use crate::TileType;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Rule(pub TileType, pub TileType, pub Direction);
