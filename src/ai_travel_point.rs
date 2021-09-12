use rand::Rng;
use rand_chacha::ChaCha8Rng;

use crate::components::Position;

pub fn building_positions(
    num_buildings: i32,
    x_max: f32,
    y_max: f32,
    rng: &mut ChaCha8Rng,
) -> Vec<Position> {
    (0..num_buildings)
        .into_iter()
        .map(|_i| {
            let x = rng.gen_range(0f32..x_max);
            let y = rng.gen_range(0f32..y_max);
            Position(x, y)
        })
        .collect()
}
