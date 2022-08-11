use crate::prelude::*;
use std::{collections::HashSet, ops::ControlFlow};

pub fn use_ground_effect(effect: Entity, ctx: &BTerm, world: &mut World) -> RunState {
    let res = world.fetch_mut::<TargetingReticule>().handle_input(ctx);

    match res {
        ControlFlow::Continue(()) => RunState::TargetGround(effect),
        ControlFlow::Break(target) => {
            world.remove::<TargetingReticule>();

            if let Some(target_pos) = target {
                let player = *world.fetch::<Entity>();
                let mut effect_usage = EffectUsage::fetch(world);
                let mut initiative_data = InitiativeData::fetch(world);

                effect_usage
                    .use_on_ground(effect, player, target_pos)
                    .unwrap();
                initiative_data.spend_turn(player);

                RunState::Running
            } else {
                RunState::AwaitingInput
            }
        }
    }
}

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

    pub fn handle_input(&mut self, ctx: &BTerm) -> ControlFlow<Option<Coordinate>> {
        use {ControlFlow::*, Direction::*, VirtualKeyCode::*};

        ctx.key.map_or(Continue(()), |key| match key {
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

                Continue(())
            }

            Space | Return => Break(Some(self.cursor)),

            X | Escape => Break(None),

            _ => Continue(()),
        })
    }
}
