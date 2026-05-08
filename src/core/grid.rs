use bevy::prelude::*;

#[derive(Resource, Clone, Debug)]
pub struct Grid2D<T> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<T>,
}

impl<T> Grid2D<T> {
    pub fn at(&self, x: i32, y: i32) -> usize {
        (y as usize) * self.width + (x as usize)
    }

    pub fn at_pos(&self, pos: &IVec2) -> usize {
        self.at(pos.x, pos.y)
    }

    pub fn get(&self, x: i32, y: i32) -> &T {
        &self.data[self.at(x, y)]
    }

    pub fn get_at_pos(&self, pos: &IVec2) -> &T { &self.data[self.at(pos.x, pos.y)] }

    pub fn set(&mut self, x: i32, y: i32, value: T) {
        let idx = self.at(x, y);
        self.data[idx] = value;
    }

    pub fn set_at_pos(&mut self, pos: &IVec2, value: T) { self.set(pos.x, pos.y, value); }
}

impl<T: Clone> Grid2D<T> {
    pub fn new(width: usize, height: usize, default_val: T) -> Self {
        Self {
            width,
            height,
            data: vec![default_val; width * height],
        }
    }
}
