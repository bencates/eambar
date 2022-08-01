use crate::{
    entity::SPAWN_TABLE,
    map::DeckBuilder,
    prelude::*,
    ui::{MAP_HEIGHT, MAP_WIDTH},
};

pub fn build_level(world: &mut World) {
    let map = {
        let rng = world.get_mut::<RandomNumberGenerator>().unwrap();

        DeckBuilder::new(MAP_WIDTH, MAP_HEIGHT)
            .with_engines()
            .with_walls(rng)
            .with_spawns(rng, &SPAWN_TABLE)
    }
    .spawn(world);

    // map.reveal();

    let player_entity = {
        let entities = world.entities();
        let players = world.read_component::<Player>();

        (&entities, &players).join().next().unwrap().0
    };

    world.insert(map);
    world.insert(player_entity);
}
