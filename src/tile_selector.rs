use std::collections::HashMap;

use TileType::{Land, Sea};

use crate::tile_map::TileType;

#[derive(Eq, PartialEq, Hash)]
struct Dirs {
    nw: TileType,
    ne: TileType,
    sw: TileType,
    se: TileType,
}

pub struct TileSelector(HashMap<Dirs, (i32, i32)>);

#[rustfmt::skip]
impl TileSelector {
    pub fn select_tile(&self, nw: TileType, ne: TileType, sw: TileType, se: TileType) -> (i32, i32) {
        let result = self.0.get(&Dirs {nw, ne, sw, se});
        let result = result.unwrap_or(&(0, 0));
        *result
    }

    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert( Dirs { nw: Sea,  ne: Sea,  sw: Sea,  se: Sea  },  (3, 3));
        map.insert( Dirs { nw: Sea,  ne: Sea,  sw: Sea,  se: Land },  (3, 2));
        map.insert( Dirs { nw: Sea,  ne: Sea,  sw: Land, se: Sea  },  (3, 1));
        map.insert( Dirs { nw: Sea,  ne: Sea,  sw: Land, se: Land },  (1, 1));
        map.insert( Dirs { nw: Sea,  ne: Land, sw: Sea,  se: Sea  },  (3, 0));
        map.insert( Dirs { nw: Sea,  ne: Land, sw: Sea,  se: Land },  (2, 1));
        map.insert( Dirs { nw: Sea,  ne: Land, sw: Land, se: Sea  },  (2, 0));
        map.insert( Dirs { nw: Sea,  ne: Land, sw: Land, se: Land },  (0, 1));
        map.insert( Dirs { nw: Land, ne: Sea,  sw: Sea,  se: Sea  },  (2, 3));
        map.insert( Dirs { nw: Land, ne: Sea,  sw: Sea,  se: Land },  (1, 2));
        map.insert( Dirs { nw: Land, ne: Sea,  sw: Land, se: Sea  },  (2, 2));
        map.insert( Dirs { nw: Land, ne: Sea,  sw: Land, se: Land },  (0, 2));
        map.insert( Dirs { nw: Land, ne: Land, sw: Sea,  se: Sea  },  (1, 3));
        map.insert( Dirs { nw: Land, ne: Land, sw: Sea,  se: Land },  (0, 3));
        map.insert( Dirs { nw: Land, ne: Land, sw: Land, se: Sea  },  (1, 0));
        map.insert( Dirs { nw: Land, ne: Land, sw: Land, se: Land },  (0, 0));
        Self(map)
    }
}

impl Default for TileSelector {
    fn default() -> Self {
        Self::new()
    }
}
