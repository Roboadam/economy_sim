use crate::tile_map::TileType;
use crate::TileMap;
use rand::thread_rng;
use rand::Rng;

pub fn create_land_mass(tile_map: &mut TileMap) {
    let mut rng = thread_rng();
    for y in 0..tile_map.width {
        for x in 0..tile_map.width {
            let is_land = rng.gen_bool(odds_of_land(x, y, tile_map.width));
            if is_land {
                tile_map.set_tile(x as i32, y as i32, TileType::Land)
            }
        }
    }
    for y in 0..tile_map.width as i32 {
        for x in 0..tile_map.width as i32 {
            if is_lonely_tile(x, y, &TileType::Sea, &tile_map) {
                tile_map.set_tile(x, y, TileType::Land);
            }
        }
    }
    for y in 0..tile_map.width as i32 {
        for x in 0..tile_map.width as i32 {
            if is_lonely_tile(x, y, &TileType::Land, &tile_map) {
                tile_map.set_tile(x, y, TileType::Sea);
            }
        }
    }
}

fn is_lonely_tile(x: i32, y: i32, tile_type: &TileType, tile_map: &TileMap) -> bool {
    if tile_map.get_tile(x, y).unwrap() != tile_type {
        return false;
    }
    if let Some(up_tile_type) = tile_map.get_tile(x, y - 1) {
        if up_tile_type == tile_type {
            return false;
        }
    }
    if let Some(down_tile_type) = tile_map.get_tile(x, y + 1) {
        if down_tile_type == tile_type {
            return false;
        }
    }
    if let Some(left_tile_type) = tile_map.get_tile(x - 1, y) {
        if left_tile_type == tile_type {
            return false;
        }
    }
    if let Some(right_tile_type) = tile_map.get_tile(x + 1, y) {
        if right_tile_type == tile_type {
            return false;
        }
    }
    true
}

fn odds_of_land(x: usize, y: usize, width: usize) -> f64 {
    let middle = width / 2;
    if x == middle && y == middle {
        return 1.;
    }

    if x == 0 || x == width - 1 || y == 0 || y == width - 1 {
        return 0.;
    }

    let middle = middle as f64;
    let x = x as f64;
    let y = y as f64;
    let x_dist = (x - middle).abs();
    let y_dist = (y - middle).abs();
    let distance = (x_dist * x_dist + y_dist * y_dist).sqrt();

    return (1. - distance / middle).clamp(0., 1.);
}
