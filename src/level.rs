use crate::map::{MapBuilder, SimpleMapBuilder};
use crate::ui::{MAP_HEIGHT, MAP_WIDTH};
use crate::{
    entity::{monster, player},
    prelude::*,
};

pub fn build_level(world: &mut World) {
    let map = SimpleMapBuilder::new(MAP_WIDTH, MAP_HEIGHT).build();

    {
        let mut spawn_points = map.spawn_points();

        if let Some(coord) = spawn_points.next() {
            log::debug!("Spawning player at {coord:?}");

            let player = player(world.create_entity()).with(coord).build();

            world.insert(player);
            // world.insert(pos);
            // world.insert(Inventory::default());
        }

        for coord in spawn_points {
            let builder = {
                let rng = world.get_mut::<RandomNumberGenerator>().unwrap();

                spawn_table(rng)
            };

            log::debug!("Spawning a monster at {coord:?}");

            builder(world.create_entity()).with(coord).build();
        }
    };

    world.insert(map);
}

fn spawn_table(rng: &mut RandomNumberGenerator) -> impl FnOnce(EntityBuilder) -> EntityBuilder {
    match rng.range(0, 2) {
        0 => monster::infected_crewmember,
        _ => monster::alien_hatchling,
    }
}
