#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum TileType {
    Land,
    Sea,
    Beach,
}

impl TileType {
    pub fn iter() -> TileTypeIter {
        TileTypeIter(Self::Land)
    }
}

pub struct TileTypeIter(TileType);

impl Iterator for TileTypeIter {
    type Item = TileType;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            TileType::Land => Some(TileType::Sea),
            TileType::Sea => Some(TileType::Beach),
            TileType::Beach => None,
        }
    }
}

pub struct TileMap {
    tiles: Vec<TileType>,
    pub width: i32,
}

impl TileMap {
    pub fn new(tiles_per_side: i32) -> Self {
        let mut tiles = Vec::new();
        for _i in 0..tiles_per_side {
            for _j in 0..tiles_per_side {
                tiles.push(TileType::Sea);
            }
        }

        TileMap {
            tiles,
            width: tiles_per_side,
        }
    }

    pub fn set_tile(&mut self, x: i32, y: i32, tile_type: TileType) {
        if let Some(index) = self.xy_as_index(x, y) {
            if let Some(elem) = self.tiles.get_mut(index) {
                *elem = tile_type;
            }
        }
    }

    pub fn get_tile(&self, x: i32, y: i32) -> Option<&TileType> {
        if let Some(index) = self.xy_as_index(x, y) {
            self.tiles.get(index)
        } else {
            None
        }
    }

    fn xy_as_index(&self, x: i32, y: i32) -> Option<usize> {
        let index = y * self.width + x;
        if index < 0 || index > self.width * self.width - 1 {
            None
        } else {
            Some(index as usize)
        }
    }
}
