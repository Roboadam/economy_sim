use crate::tile_map::{BuildingType, TileMap, TileType};

pub fn generate_buildings(tile_map: &mut TileMap) -> Vec<BuildingType> {
    let mid = tile_map.width as i32 / 2;
    tile_map.set_tile(mid, mid, TileType::Building(BuildingType::Business(1)));
    tile_map.set_tile(mid + 1, mid, TileType::Building(BuildingType::Business(1)));
    println!("b1: {},{} {},{}", mid, mid, mid+1, mid);

    tile_map.set_tile(mid + 5, mid, TileType::Building(BuildingType::Business(2)));
    tile_map.set_tile(
        mid + 5,
        mid + 1,
        TileType::Building(BuildingType::Business(2)),
    );
    println!("b2: {},{} {},{}", mid+5, mid, mid+5, mid+1);

    vec![BuildingType::Business(1), BuildingType::Business(2)]
}
