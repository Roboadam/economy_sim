use crate::{
    business::BusinessId,
    tile_map::{Building, BuildingType::Business, TileMap, TileType},
};

pub fn generate_buildings(tile_map: &mut TileMap) -> Vec<Building> {
    let mid = tile_map.width as i32 / 2;

    let building1 = Building {
        id: BusinessId(1),
        building_type: Business,
    };
    let building2 = Building {
        id: BusinessId(2),
        building_type: Business,
    };

    tile_map.set_tile(mid, mid, TileType::BuildingTile(building1));
    tile_map.set_tile(mid + 1, mid, TileType::BuildingTile(building1));

    tile_map.set_tile(mid + 5, mid, TileType::BuildingTile(building2));
    tile_map.set_tile(mid + 5, mid + 1, TileType::BuildingTile(building2));

    vec![building1, building2]
}
