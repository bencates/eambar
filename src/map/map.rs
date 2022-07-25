use super::tile::Tile;
use crate::prelude::*;
use std::ops::Index;

pub struct Map {
    pub(super) tiles: Vec<Tile>,
    pub rooms: Vec<Rect>,
    width: i32,
    height: i32,
}

impl Map {
    pub(super) fn new(width: i32, height: i32) -> Self {
        let tiles: Vec<Tile> = (0..(width * height)).map(|_| Tile::default()).collect();
        let rooms = Vec::new();

        Self {
            tiles,
            rooms,
            width,
            height,
        }
    }
}

impl Index<Point> for Map {
    type Output = Tile;

    fn index(&self, pos: Point) -> &Self::Output {
        &self.tiles[self.point2d_to_index(pos)]
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {}
