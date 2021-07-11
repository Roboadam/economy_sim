use crate::tile_map::{TileMap, TileType};

pub fn generate_buildings(tile_map: &mut TileMap) -> Vec<i32> {
    let mid = tile_map.width as i32 / 2;
    tile_map.set_tile(mid, mid, TileType::Building(1));
    tile_map.set_tile(mid + 1, mid, TileType::Building(1));

    tile_map.set_tile(mid + 5, mid, TileType::Building(2));
    tile_map.set_tile(mid + 5, mid + 1, TileType::Building(2));

    vec![1, 2]
}
