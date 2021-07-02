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

struct TileSelector(HashMap<Dirs, (i32, i32)>);

impl TileSelector {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert(
            Dirs {
                nw: Sea,
                ne: Sea,
                sw: Sea,
                se: Sea,
            },
            (3, 3),
        );
        map.insert(
            Dirs {
                nw: Land,
                ne: Sea,
                sw: Sea,
                se: Sea,
            },
            (2, 3),
        );
        Self(map)
    }
}

impl Default for TileSelector {
    fn default() -> Self {
        Self::new()
    }
}
