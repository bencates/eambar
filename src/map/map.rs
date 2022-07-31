use super::tile::Tile;
use crate::{game_mechanics::is_legal_move, prelude::*};
use std::ops::{Index, IndexMut};

pub struct Map {
    /// Tiles are stored as an "odd-q" rectangle in row-major order
    pub(super) tiles: Vec<Tile>,
    pub(super) spawn_points: Vec<Coordinate>,
    width: i32,
    height: i32,
}

impl Map {
    pub(super) fn new(width: i32, height: i32) -> Self {
        let tiles: Vec<Tile> = (0..(width * height)).map(|_| Tile::wall()).collect();
        let spawn_points = Vec::new();

        Self {
            tiles,
            spawn_points,
            width,
            height,
        }
    }

    pub fn spawn_points(&self) -> &[Coordinate] {
        &self.spawn_points
    }

    pub fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }

    pub fn iter(&self) -> impl Iterator<Item = Coordinate> + '_ {
        (0..self.tiles.len()).map(|idx| Coordinate::from_index(idx, self.width))
    }

    pub fn reveal(&mut self) {
        self.tiles.iter_mut().for_each(|tile| tile.reveal());
    }

    pub fn path(
        &self,
        start: Coordinate,
        end: Coordinate,
    ) -> Option<impl Iterator<Item = Coordinate> + '_> {
        let path = a_star_search(start.to_index(self.width), end.to_index(self.width), self);

        path.success.then(|| {
            path.steps
                .into_iter()
                .map(|idx| Coordinate::from_index(idx, self.width))
        })
    }

    pub fn in_bounds(&self, coord: Coordinate) -> bool {
        let Point { x, y } = coord.into();

        (0..self.width).contains(&x) && (0..self.height).contains(&y)
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

impl BaseMap for Map {
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        Coordinate::from_index(idx, self.width)
            .neighbors()
            .into_iter()
            .filter_map(|potential_exit| {
                is_legal_move(self, potential_exit)
                    .then(|| (potential_exit.to_index(self.width), 1.0))
            })
            .collect()
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let c1 = Coordinate::from_index(idx1, self.width);
        let c2 = Coordinate::from_index(idx2, self.width);

        c1.distance(c2) as f32
    }
}
