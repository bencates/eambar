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
///   +s_____-r
///    /     \
/// -q<       > +q
///    \_____/
///   +r     -s
#[derive(Clone, Copy, Component, Debug, Hash, PartialEq, Eq)]
pub struct Coordinate {
    pub q: i32,
    pub r: i32,
}

impl Coordinate {
    pub fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }

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
            self + North,
            self + NorthEast,
            self + SouthEast,
            self + South,
            self + SouthWest,
            self + NorthWest,
        ]
    }

    pub fn distance(self, other: Self) -> i32 {
        Hex2dCoordinate::from(self).distance(other.into())
    }

    pub fn range(self, radius: i32) -> impl Iterator<Item = Self> {
        Hex2dCoordinate::from(self)
            .range_iter(radius)
            .map(|c| c.into())
    }

    pub fn line_to(self, other: Self) -> impl Iterator<Item = Self> {
        Hex2dCoordinate::from(self)
            .line_to_iter(other.into())
            .map(|c| c.into())
    }
}

impl From<Coordinate> for Point {
    fn from(Coordinate { q, r }: Coordinate) -> Self {
        Self {
            x: q,
            y: r + (q - (q & 1)) / 2,
        }
    }
}

impl From<Point> for Coordinate {
    fn from(Point { x, y }: Point) -> Self {
        Self {
            q: x,
            r: y - (x - (x & 1)) / 2,
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

        let y = if x & 1 != 0 { y as f32 + 0.5 } else { y as f32 };

        Self { x: x as f32, y }
    }
}

/// The six cardinal directions on our hex grid
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

impl Add<Direction> for Coordinate {
    type Output = Coordinate;

    fn add(self, direction: Direction) -> Self::Output {
        let Self { q, r } = self;
        match direction {
            North => Coordinate { q, r: r - 1 },
            NorthEast => Coordinate { q: q + 1, r: r - 1 },
            SouthEast => Coordinate { q: q + 1, r },
            South => Coordinate { q, r: r + 1 },
            SouthWest => Coordinate { q: q - 1, r: r + 1 },
            NorthWest => Coordinate { q: q - 1, r },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Points in odd-q layout.
    const CLOCK_DIRECTIONS: [(Coordinate, Point); 12] = [
        (Coordinate { q: 0, r: -2 }, Point { x: 0, y: -2 }), // 12:00
        (Coordinate { q: 1, r: -2 }, Point { x: 1, y: -2 }), // 1:00
        (Coordinate { q: 2, r: -2 }, Point { x: 2, y: -1 }), // 2:00
        (Coordinate { q: 2, r: -1 }, Point { x: 2, y: 0 }),  // 3:00
        (Coordinate { q: 2, r: 0 }, Point { x: 2, y: 1 }),   // 4:00
        (Coordinate { q: 1, r: 1 }, Point { x: 1, y: 1 }),   // 5:00
        (Coordinate { q: 0, r: 2 }, Point { x: 0, y: 2 }),   // 6:00
        (Coordinate { q: -1, r: 2 }, Point { x: -1, y: 1 }), // 7:00
        (Coordinate { q: -2, r: 2 }, Point { x: -2, y: 1 }), // 8:00
        (Coordinate { q: -2, r: 1 }, Point { x: -2, y: 0 }), // 9:00
        (Coordinate { q: -2, r: 0 }, Point { x: -2, y: -1 }), // 10:00
        (Coordinate { q: -1, r: -1 }, Point { x: -1, y: -2 }), // 11:00
    ];

    #[test]
    fn test_converts_coordinates_to_points() {
        for (coordinate, point) in CLOCK_DIRECTIONS {
            assert_eq!(Point::from(coordinate), point);
        }
    }

    #[test]
    fn test_converts_points_to_coordinates() {
        for (coordinate, point) in CLOCK_DIRECTIONS {
            assert_eq!(Coordinate::from(point), coordinate);
        }
    }

    #[test]
    fn test_adding_directions() {
        let origin = Coordinate { q: 0, r: 0 };

        assert_eq!(origin + North, Coordinate { q: 0, r: -1 });
        assert_eq!(origin + NorthEast, Coordinate { q: 1, r: -1 });
        assert_eq!(origin + SouthEast, Coordinate { q: 1, r: 0 });
        assert_eq!(origin + South, Coordinate { q: 0, r: 1 });
        assert_eq!(origin + SouthWest, Coordinate { q: -1, r: 1 });
        assert_eq!(origin + NorthWest, Coordinate { q: -1, r: 0 });
    }
}
