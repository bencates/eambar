use crate::prelude::*;
use std::{collections::HashSet, ops::ControlFlow};

pub struct TargetingReticule {
    pub coordinates: HashSet<Coordinate>,
    pub cursor: Coordinate,
}

impl TargetingReticule {
    pub fn new(origin: Coordinate, range: i32, map: &Map) -> Self {
        let mut coordinates = std::collections::HashSet::new();

        for edge in origin.ring(range) {
            for (c1, c2) in origin.fat_line_to(edge) {
                if map[c1].is_opaque() && map[c2].is_opaque() {
                    break;
                }

                if !map[c1].is_opaque() {
                    coordinates.insert(c1);
                }

                if !map[c2].is_opaque() {
                    coordinates.insert(c2);
                }
            }
        }

        Self {
            cursor: origin,
            coordinates,
        }
    }

    pub fn handle_input(&mut self, ctx: &BTerm) -> ControlFlow<Option<Coordinate>, ()> {
        use {Direction::*, VirtualKeyCode::*};

        ctx.key.map_or(ControlFlow::Continue(()), |key| match key {
            // Movement keys
            Q | W | E | A | S | D => {
                let new_cursor = self.cursor
                    + match key {
                        Q => NorthWest,
                        W => North,
                        E => NorthEast,
                        A => SouthWest,
                        S => South,
                        D => SouthEast,
                        _ => unreachable!(),
                    };

                if self.coordinates.contains(&new_cursor) {
                    self.cursor = new_cursor;
                }

                ControlFlow::Continue(())
            }

            Space | Return => ControlFlow::Break(Some(self.cursor)),

            Escape => ControlFlow::Break(None),

            _ => ControlFlow::Continue(()),
        })
    }
}
