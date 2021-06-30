use std::collections::HashMap;

use crate::tile_map::TileType;

#[derive(Eq, PartialEq, Hash)]
struct TileIntersection {
    nw: TileType,
    ne: TileType,
    sw: TileType,
    se: TileType,
}

struct TileSelector(HashMap<TileIntersection, (i32, i32)>);

impl TileSelector {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert(
            TileIntersection {
                nw: TileType::Sea,
                ne: TileType::Sea,
                sw: TileType::Sea,
                se: TileType::Sea,
            },
            (3, 3),
        );
        Self(map)
    }
}

impl Default for TileSelector {
    fn default() -> Self {
        Self::new()
    }
}
