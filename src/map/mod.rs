mod builder;
mod hex_coordinates;
mod map;
mod tile;

pub use builder::*;
pub use hex_coordinates::{Coordinate, Direction};
pub use map::Map;

use crate::prelude::*;

#[derive(Component)]
pub struct BlocksTile;

pub struct IndexMapSystem;

impl<'a> System<'a> for IndexMapSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Coordinate>,
        ReadStorage<'a, BlocksTile>,
        ReadStorage<'a, Viewshed>,
    );

    fn run(
        &mut self,
        (mut map, entities, players, coordinates, blockers, viewsheds): Self::SystemData,
    ) {
        map.tiles.iter_mut().for_each(|tile| tile.reset_index());

        for (entity, coord) in (&entities, &coordinates).join() {
            if blockers.contains(entity) {
                map[*coord].block();
            }

            map[*coord].add_entity(entity);
        }

        for (_, vs) in (&players, &viewsheds).join() {
            for &pos in vs.iter() {
                map[pos.into()].reveal();
            }
        }
    }
}
