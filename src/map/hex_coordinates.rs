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
#[storage(FlaggedStorage)]
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

    pub fn ring(self, radius: i32) -> impl Iterator<Item = Self> {
        Hex2dCoordinate::from(self)
            .ring_iter(radius, hex2d::Spin::CW(hex2d::XY))
            .map(|c| c.into())
    }

    pub fn line_to(self, other: Self) -> impl Iterator<Item = Self> {
        Hex2dCoordinate::from(self)
            .line_to_iter(other.into())
            .map(|c| c.into())
    }

    pub fn fat_line_to(self, other: Self) -> impl Iterator<Item = (Self, Self)> {
        Hex2dCoordinate::from(self)
            .line_to_with_edge_detection_iter(other.into())
            .map(|(c1, c2)| (c1.into(), c2.into()))
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
    use {super::*, test_case::test_case};

    // Points in odd-q layout.
    #[test_case(Coordinate::new( 0, -2), Point::new( 0, -2); "12:00")]
    #[test_case(Coordinate::new( 1, -2), Point::new( 1, -2);  "1:00")]
    #[test_case(Coordinate::new( 2, -2), Point::new( 2, -1);  "2:00")]
    #[test_case(Coordinate::new( 2, -1), Point::new( 2,  0);  "3:00")]
    #[test_case(Coordinate::new( 2,  0), Point::new( 2,  1);  "4:00")]
    #[test_case(Coordinate::new( 1,  1), Point::new( 1,  1);  "5:00")]
    #[test_case(Coordinate::new( 0,  2), Point::new( 0,  2);  "6:00")]
    #[test_case(Coordinate::new(-1,  2), Point::new(-1,  1);  "7:00")]
    #[test_case(Coordinate::new(-2,  2), Point::new(-2,  1);  "8:00")]
    #[test_case(Coordinate::new(-2,  1), Point::new(-2,  0);  "9:00")]
    #[test_case(Coordinate::new(-2,  0), Point::new(-2, -1); "10:00")]
    #[test_case(Coordinate::new(-1, -1), Point::new(-1, -2); "11:00")]
    fn converts_between_coordinates_to_points(coordinate: Coordinate, point: Point) {
        assert_eq!(Coordinate::from(point), coordinate);
        assert_eq!(Point::from(coordinate), point);
    }

    #[test_case(North     => Coordinate::new( 0, -1); "north")]
    #[test_case(NorthEast => Coordinate::new( 1, -1); "north east")]
    #[test_case(SouthEast => Coordinate::new( 1,  0); "south east")]
    #[test_case(South     => Coordinate::new( 0,  1); "south")]
    #[test_case(SouthWest => Coordinate::new(-1,  1); "south west")]
    #[test_case(NorthWest => Coordinate::new(-1,  0); "north west")]
    fn adding_directions(direction: Direction) -> Coordinate {
        Coordinate::new(0, 0) + direction
    }
}
