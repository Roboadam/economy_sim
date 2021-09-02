use crate::business::BusinessId;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum TileType {
    LandTile,
    SeaTile,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Building {
    pub id: BusinessId,
}
