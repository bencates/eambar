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

    pub fn spawn_points(&self) -> impl Iterator<Item = Coordinate> + '_ {
        self.rooms.iter().map(|room| room.center().into())
    }

    pub fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }

    pub fn reveal(&mut self) {
        self.tiles.iter_mut().for_each(|tile| tile.reveal());
    }
}

impl Index<Coordinate> for Map {
    type Output = Tile;

    fn index(&self, coord: Coordinate) -> &Self::Output {
        &self.tiles[coord.to_index(self.width)]
    }
}

impl IndexMut<Coordinate> for Map {
    fn index_mut(&mut self, coord: Coordinate) -> &mut Self::Output {
        let idx = coord.to_index(self.width);
        &mut self.tiles[idx]
    }
}

impl BaseMap for Map {}
