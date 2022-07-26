use super::tile::Tile;
use crate::prelude::*;
use std::ops::{Index, IndexMut};

pub struct Map {
    /// Tiles are stored as an "odd-q" rectangle in row-major order
    pub(super) tiles: Vec<Tile>,
    pub rooms: Vec<Rect>,
    width: i32,
    height: i32,
}

impl Map {
    pub(super) fn new(width: i32, height: i32) -> Self {
        let tiles: Vec<Tile> = (0..(width * height)).map(|_| Tile::wall()).collect();
        let rooms = Vec::new();

        Self {
            tiles,
            rooms,
            width,
            height,
        }
    }

    pub fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl Index<Point> for Map {
    type Output = Tile;

    fn index(&self, pos: Point) -> &Self::Output {
        &self.tiles[pos.to_index(self.width)]
    }
}

impl IndexMut<Point> for Map {
    fn index_mut(&mut self, pos: Point) -> &mut Self::Output {
        let idx = pos.to_index(self.width);
        &mut self.tiles[idx]
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx].is_opaque()
    }
}
