// use crate::prelude::*;
// use std::ops::Add;

/// The six cardinal directions on our hex grid
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    /// -q axis
    NorthEast,
    /// +q axis
    SouthWest,

    // -r axis
    NorthWest,
    // +r axis
    SouthEast,

    /// -s axis
    North,
    /// +s axis
    South,
}

// impl Add<Direction> for Point {
//     type Output = Point;

//     fn add(self, direction: Direction) -> Self::Output {
//         match direction {
//             Direction::North => self + Point::new(0, -1),
//             Direction::NorthEast => self + Point::new(1, -1),
//             Direction::East => self + Point::new(1, 0),
//             Direction::SouthEast => self + Point::new(1, 1),
//             Direction::South => self + Point::new(0, 1),
//             Direction::SouthWest => self + Point::new(-1, 1),
//             Direction::West => self + Point::new(-1, 0),
//             Direction::NorthWest => self + Point::new(-1, -1),
//         }
//     }
// }
