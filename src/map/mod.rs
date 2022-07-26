mod builder;
mod hex_coordinates;
mod map;
mod tile;

pub use builder::*;
pub use hex_coordinates::{Coordinate, Direction};
pub use map::Map;

use crate::prelude::*;

pub struct IndexMapSystem;

impl<'a> System<'a> for IndexMapSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Coordinate>,
        ReadStorage<'a, Viewshed>,
    );

    fn run(&mut self, (mut map, entities, players, coordinates, viewsheds): Self::SystemData) {
        map.tiles.iter_mut().for_each(|tile| tile.reset_index());

        for (entity, coord) in (&entities, &coordinates).join() {
            let pos = Point::from(*coord);

            // if blockers.contains(entity) {
            //     map[pos].block();
            // }

            map[pos].add_entity(entity);
        }

        for (_, vs) in (&players, &viewsheds).join() {
            for &pos in vs.iter() {
                map[pos.into()].reveal();
            }
        }
    }
}
