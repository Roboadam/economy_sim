use crate::components::Position;

struct AABB {
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
}

impl AABB {
    pub fn new(center: Position, half_dimension: f32) -> Self {
        let x_min = center.x - half_dimension;
        let x_max = center.x + half_dimension;
        let y_min = center.y - half_dimension;
        let y_max = center.y + half_dimension;
        AABB {
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }

    pub fn contains_position(&self, position: Position) -> bool {
        position.x >= self.x_min
            && position.x <= self.x_max
            && position.y >= self.y_min
            && position.y <= self.y_max
    }

    pub fn intersects_aabb(&self, other: &Self) -> bool {
        (self.x_min <= other.x_max && self.x_max >= other.x_min)
            && (self.y_min <= other.y_max && self.y_max >= other.y_min)
    }
}

#[cfg(test)]
mod tests {
    use crate::components::Position;

    use super::AABB;

    #[test]
    fn aabb_intesects() {
        let center = Position { x: 0., y: 0. };
        let a = AABB::new(center, 5.);
        let b = AABB::new(center, 6.);
        assert!(a.intersects_aabb(&b));
        assert!(b.intersects_aabb(&a));

        let center = Position { x: 0., y: 0. };
        let a = AABB::new(center, 5.);
        let center = Position { x: 1., y: 0. };
        let b = AABB::new(center, 5.1);
        assert!(a.intersects_aabb(&b));
        assert!(b.intersects_aabb(&a));

        let center = Position { x: 0., y: 0. };
        let a = AABB::new(center, 1.);
        let center = Position { x: 1., y: 1. };
        let b = AABB::new(center, 1.);
        assert!(a.intersects_aabb(&b));
        assert!(b.intersects_aabb(&a));

        let center = Position { x: 0., y: 0. };
        let a = AABB::new(center, 1.);
        let center = Position { x: 10., y: 0. };
        let b = AABB::new(center, 1.);
        assert!(!a.intersects_aabb(&b));
        assert!(!b.intersects_aabb(&a));
    }
}
