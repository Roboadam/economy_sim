use std::collections::HashMap;

use TileType::{LandTile, SeaTile};

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
        if let TileType::BuildingTile(_) = nw {
            return (4, 0);
        }
        let result = self.0.get(&Dirs {nw, ne, sw, se});
        let result = result.unwrap_or(&(0, 0));
        *result
    }

    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert( Dirs { nw: SeaTile,  ne: SeaTile,  sw: SeaTile,  se: SeaTile  },  (3, 3));
        map.insert( Dirs { nw: SeaTile,  ne: SeaTile,  sw: SeaTile,  se: LandTile },  (3, 2));
        map.insert( Dirs { nw: SeaTile,  ne: SeaTile,  sw: LandTile, se: SeaTile  },  (3, 1));
        map.insert( Dirs { nw: SeaTile,  ne: SeaTile,  sw: LandTile, se: LandTile },  (1, 1));
        map.insert( Dirs { nw: SeaTile,  ne: LandTile, sw: SeaTile,  se: SeaTile  },  (3, 0));
        map.insert( Dirs { nw: SeaTile,  ne: LandTile, sw: SeaTile,  se: LandTile },  (2, 1));
        map.insert( Dirs { nw: SeaTile,  ne: LandTile, sw: LandTile, se: SeaTile  },  (2, 0));
        map.insert( Dirs { nw: SeaTile,  ne: LandTile, sw: LandTile, se: LandTile },  (0, 1));
        map.insert( Dirs { nw: LandTile, ne: SeaTile,  sw: SeaTile,  se: SeaTile  },  (2, 3));
        map.insert( Dirs { nw: LandTile, ne: SeaTile,  sw: SeaTile,  se: LandTile },  (1, 2));
        map.insert( Dirs { nw: LandTile, ne: SeaTile,  sw: LandTile, se: SeaTile  },  (2, 2));
        map.insert( Dirs { nw: LandTile, ne: SeaTile,  sw: LandTile, se: LandTile },  (0, 2));
        map.insert( Dirs { nw: LandTile, ne: LandTile, sw: SeaTile,  se: SeaTile  },  (1, 3));
        map.insert( Dirs { nw: LandTile, ne: LandTile, sw: SeaTile,  se: LandTile },  (0, 3));
        map.insert( Dirs { nw: LandTile, ne: LandTile, sw: LandTile, se: SeaTile  },  (1, 0));
        map.insert( Dirs { nw: LandTile, ne: LandTile, sw: LandTile, se: LandTile },  (0, 0));
        Self(map)
    }
}

impl Default for TileSelector {
    fn default() -> Self {
        Self::new()
    }
}
