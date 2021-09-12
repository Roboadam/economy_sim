use crate::components::Position;

struct AABB {
    center: Position,
    half_dimension: f32,
    // function containsPoint(XY point) {...}
    // function intersectsAABB(AABB other) {...}
}

impl AABB {
    pub fn new(center: Position, half_dimension: f32) -> Self {
        AABB {
            center,
            half_dimension,
        }
    }

    pub fn contains_point()
}
