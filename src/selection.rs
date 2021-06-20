use macroquad::prelude::*;
pub struct Selection {
    max_index: usize,
    pub x: i32,
    pub y: i32,
}

impl Selection {
    pub fn new(side_len_in_tiles: usize) -> Self {
        Self {
            max_index: side_len_in_tiles - 1,
            x: 0,
            y: 0,
        }
    }

    pub fn up(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }

    pub fn down(&mut self) {
        if self.y < self.max_index as i32 {
            self.y += 1;
        }
    }

    pub fn left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn right(&mut self) {
        if self.x < self.max_index as i32 {
            self.x += 1;
        }
    }
}
