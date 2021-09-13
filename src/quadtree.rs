use crate::components::Position;

const QT_NODE_CAPACITY: usize = 4;

pub struct Quadtree {
    boundary: AABB,
    points: Vec<Position>,
    children: Option<Children>,
}

struct Children {
    north_west: Box<Quadtree>,
    north_east: Box<Quadtree>,
    south_west: Box<Quadtree>,
    south_east: Box<Quadtree>,
}

impl Quadtree {
    pub fn new(aabb: AABB) -> Self {
        Quadtree {
            boundary: aabb,
            points: Vec::with_capacity(QT_NODE_CAPACITY),
            children: None,
        }
    }

    pub fn insert(&mut self, position: Position) -> Result<(), ()> {
        if !self.boundary.contains_position(position) {
            return Err(());
        }

        if self.points.len() < QT_NODE_CAPACITY && self.children.is_none() {
            self.points.push(position);
            return Ok(());
        }

        if self.children.is_none() {
            self.subdivide();
        }

        let children = self.children.as_mut().unwrap();

        if children.north_west.insert(position).is_ok() {
            return Ok(());
        }
        if children.north_east.insert(position).is_ok() {
            return Ok(());
        }
        if children.south_west.insert(position).is_ok() {
            return Ok(());
        }
        children.south_east.insert(position)
    }

    pub fn query_range(&self, range: &AABB) -> Vec<Position> {
        let mut points_in_range = vec![];

        if !self.boundary.intersects_aabb(&range) {
            return points_in_range;
        }

        for point in &self.points {
            if range.contains_position(*point) {
                points_in_range.push(*point);
            }
        }

        if let Some(children) = &self.children {
            let mut nw = children.north_west.query_range(range);
            let mut ne = children.north_east.query_range(range);
            let mut sw = children.south_west.query_range(range);
            let mut se = children.south_east.query_range(range);
            points_in_range.append(&mut nw);
            points_in_range.append(&mut ne);
            points_in_range.append(&mut sw);
            points_in_range.append(&mut se);
        }

        points_in_range
    }

    pub fn subdivide(&mut self) {
        if self.children.is_some() {
            return;
        }

        let center = self.boundary.center();
        let north_west =
            AABB::new_min_max(self.boundary.x_min, center.x, self.boundary.y_min, center.y);
        let north_east =
            AABB::new_min_max(center.x, self.boundary.x_max, self.boundary.y_min, center.y);
        let south_west =
            AABB::new_min_max(self.boundary.x_min, center.x, center.y, self.boundary.y_max);
        let south_east =
            AABB::new_min_max(center.x, self.boundary.x_max, center.y, self.boundary.y_max);

        self.children = Some(Children {
            north_west: Box::new(Quadtree::new(north_west)),
            north_east: Box::new(Quadtree::new(north_east)),
            south_west: Box::new(Quadtree::new(south_west)),
            south_east: Box::new(Quadtree::new(south_east)),
        });
    }
}

pub struct AABB {
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
    pub fn new_min_max(x_min: f32, x_max: f32, y_min: f32, y_max: f32) -> Self {
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

    pub fn center(&self) -> Position {
        Position {
            x: (self.x_max - self.x_min) / 2. + self.x_min,
            y: (self.y_max - self.y_min) / 2. + self.y_min,
        }
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
