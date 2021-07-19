#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum TileType {
    LandTile,
    SeaTile,
    BuildingTile(Building),
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Building {
    pub id: i32,
    pub building_type: BuildingType,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum BuildingType {
    Business,
}

pub struct TileMap {
    tiles: Vec<TileType>,
    buildings: Vec<BuildingLocations>,
    pub width: usize,
}

struct BuildingLocations {
    coords: (i32, i32),
    building_id: i32,
}

impl TileMap {
    pub fn new(tiles_per_side: usize) -> Self {
        let mut tiles = Vec::new();
        for _i in 0..tiles_per_side {
            for _j in 0..tiles_per_side {
                tiles.push(TileType::SeaTile);
            }
        }

        TileMap {
            tiles,
            buildings: vec![],
            width: tiles_per_side,
        }
    }

    pub fn set_tile(&mut self, x: i32, y: i32, tile_type: TileType) {
        if let Some(index) = self.xy_as_index(x, y) {
            if let Some(elem) = self.tiles.get_mut(index) {
                *elem = tile_type;
                if let TileType::BuildingTile(Building {
                    id: building_id,
                    building_type: BuildingType::Business,
                }) = tile_type
                {
                    self.buildings.push(BuildingLocations {
                        coords: (x, y),
                        building_id,
                    });
                }
            }
        }
    }

    pub fn close_building(&self, location: (f32, f32)) -> Option<i32> {
        const MAX_DIST: i32 = 2;
        const SQUARED_MAX_DIST: i32 = MAX_DIST * MAX_DIST;
        let mut result = None;
        for building_location in self.buildings {
            let start = building_location.coords;
            let dist_sq = dist_squared(start, location);
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
        let index = y * self.width as i32 + x;
        if index < 0 || index > (self.width * self.width) as i32 - 1 {
            None
        } else {
            Some(index as usize)
        }
    }
}

fn dist_squared(start: (i32, i32), end: (f32, f32)) -> f32 {
    let start = (start.0 as f32, start.1 as f32);
    let diff = (start.0 - end.0, start.1 - end.1);
    diff.0 * diff.0 + diff.1 + diff.1
}
