mod builder;
mod map;
mod tile;

pub use builder::*;
pub use map::Map;

use crate::prelude::*;

// TODO: convert to hex coordinates
// https://www.redblobgames.com/grids/hexagons/
#[derive(Component, Clone, Copy)]
pub struct Coordinates {
    x: i32,
    y: i32,
}

impl From<Point> for Coordinates {
    fn from(Point { x, y }: Point) -> Self {
        Self { x, y }
    }
}

impl From<Coordinates> for Point {
    fn from(Coordinates { x, y }: Coordinates) -> Self {
        Self { x, y }
    }
}
