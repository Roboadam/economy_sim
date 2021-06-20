use crate::TileType;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Rule {
    pub from_tile_type: TileType, 
    pub to_tile_type: TileType, 
    pub direction: Direction,
}
