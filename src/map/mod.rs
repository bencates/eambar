mod hex_coordinates;
mod level_builder;
#[allow(clippy::module_inception)]
mod map;
mod tile;

pub use hex_coordinates::{Coordinate, Direction};
pub use level_builder::DeckBuilder;
pub use map::Map;

use crate::prelude::*;

#[derive(Component)]
pub struct BlocksTile;

pub struct IndexMapSystem;

impl<'a> System<'a> for IndexMapSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        ReadStorage<'a, Coordinate>,
        ReadStorage<'a, BlocksTile>,
    );

    fn run(&mut self, (mut map, entities, coordinates, blockers): Self::SystemData) {
        map.tiles.iter_mut().for_each(|tile| tile.reset_index());

        for (entity, coord) in (&entities, &coordinates).join() {
            if blockers.contains(entity) {
                map[*coord].block();
            }

            map[*coord].add_entity(entity);
        }
    }
}
