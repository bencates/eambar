mod bisection_generator;
mod spawner;
mod template;

use crate::{
    entity::{player, SpawnBuilder},
    prelude::*,
};
use std::collections::HashMap;

pub struct DeckBuilder {
    map: Map,
    spawns: HashMap<Coordinate, SpawnBuilder>,
}

impl DeckBuilder {
    pub fn new(width: i32, height: i32) -> Self {
        let map = template::empty_deck(width, height);

        let mut spawns: HashMap<Coordinate, SpawnBuilder> = HashMap::new();
        spawns.insert(Coordinate::from(Point::new(width / 2, height / 2)), player);

        Self { map, spawns }
    }

    pub fn with_engines(mut self) -> Self {
        template::add_engines(&mut self.map);
        self
    }

    pub fn with_walls(mut self, rng: &mut RandomNumberGenerator) -> Self {
        bisection_generator::add_walls(&mut self.map, rng);
        self
    }

    pub fn with_spawns(
        mut self,
        rng: &mut RandomNumberGenerator,
        spawn_table: &[SpawnBuilder],
    ) -> Self {
        let regions = spawner::generate_regions(&self.map, rng);
        let spawn_dice = DiceType::new(1, 6, -3);

        for region in regions {
            self.spawns
                .extend(spawner::spawn_region(rng, &region, spawn_table, spawn_dice))
        }

        self
    }

    pub fn spawn(self, world: &mut World) -> Map {
        for (coord, builder) in self.spawns {
            builder(world.create_entity()).with(coord).build();
        }

        self.map
    }
}
