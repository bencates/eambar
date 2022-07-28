/// Hexagons are the bestagons!
use crate::prelude::*;
use hex2d::Coordinate as Hex2dCoordinate;
use std::ops::Add;
use Direction::*;

/// Axial hex coordinates
///
/// A coordinate pair is the `q` and `r` coordinates. The `s` coordinate can be
/// derived from those two as `s = -q-r`.
///
/// See https://www.redblobgames.com/grids/hexagons/ for more details.
///
///    -r +q
///     | /
///     *
///   / |
/// -q +r
#[derive(Clone, Copy, Component, Debug, Hash, PartialEq, Eq)]
pub struct Coordinate {
    pub q: i32,
    pub r: i32,
}

impl Coordinate {
    pub fn from_index(idx: usize, width: i32) -> Self {
        let w: usize = width
            .try_into()
            .expect("Not a valid usize. Did something go negative?");

        Point::new(idx % w, idx / w).into()
    }

    pub fn to_index(self, width: impl TryInto<usize>) -> usize {
        Point::from(self).to_index(width)
    }

    pub fn neighbors(self) -> [Self; 6] {
        [
            self + NorthEast,
            self + East,
            self + SouthEast,
            self + SouthWest,
            self + West,
            self + NorthWest,
        ]
    }

    pub fn distance(self, other: Self) -> i32 {
        Hex2dCoordinate::from(self).distance(other.into())
    }
}

impl From<Coordinate> for Point {
    fn from(Coordinate { q, r }: Coordinate) -> Self {
        Self {
            x: q + (r - (r & 1)) / 2,
            y: r,
        }
    }
}

impl From<Point> for Coordinate {
    fn from(Point { x, y }: Point) -> Self {
        Self {
            q: x - (y - (y & 1)) / 2,
            r: y,
        }
    }
}

impl From<Coordinate> for Hex2dCoordinate {
    fn from(c: Coordinate) -> Self {
        Self { x: c.q, y: c.r }
    }
}

impl From<Hex2dCoordinate> for Coordinate {
    fn from(c: Hex2dCoordinate) -> Self {
        Self { q: c.x, r: c.y }
    }
}

/// Tiles rendering is an "odd-q" rectangle. Odd numbered rows are shifted down
/// (+y) by half a tile.
impl From<Coordinate> for PointF {
    fn from(coord: Coordinate) -> Self {
        let Point { x, y } = coord.into();

        let x = if y & 1 != 0 { x as f32 + 0.5 } else { x as f32 };

        Self { x, y: y as f32 }
    }
}

/// The six cardinal directions on our hex grid
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    NorthEast,
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
}

impl Add<Direction> for Coordinate {
    type Output = Coordinate;

    fn add(self, direction: Direction) -> Self::Output {
        match direction {
            NorthEast => Coordinate {
                q: self.q + 1,
                r: self.r - 1,
            },
            East => Coordinate {
                q: self.q + 1,
                r: self.r,
            },
            SouthEast => Coordinate {
                q: self.q,
                r: self.r + 1,
            },
            SouthWest => Coordinate {
                q: self.q - 1,
                r: self.r + 1,
            },
            West => Coordinate {
                q: self.q - 1,
                r: self.r,
            },
            NorthWest => Coordinate {
                q: self.q,
                r: self.r - 1,
            },
        }
    }
}
