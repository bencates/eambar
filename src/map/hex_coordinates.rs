/// Hexagons are the bestagons!
use crate::prelude::*;
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
#[derive(Clone, Copy, Component, Debug, PartialEq, Eq)]
pub struct Coordinate {
    q: i32,
    r: i32,
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
        match direction {
            North => Coordinate {
                q: self.q,
                r: self.r - 1,
            },
            NorthEast => Coordinate {
                q: self.q + 1,
                r: self.r - 1,
            },
            SouthEast => Coordinate {
                q: self.q + 1,
                r: self.r,
            },
            South => Coordinate {
                q: self.q,
                r: self.r + 1,
            },
            SouthWest => Coordinate {
                q: self.q - 1,
                r: self.r + 1,
            },
            NorthWest => Coordinate {
                q: self.q - 1,
                r: self.r,
            },
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
