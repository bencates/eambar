use crate::{
    entity::{monster, player},
    map::DeckBuilder,
    prelude::*,
    ui::{MAP_HEIGHT, MAP_WIDTH},
};
use std::collections::HashMap;

pub fn build_level(world: &mut World) {
    let map = {
        let rng = world.get_mut::<RandomNumberGenerator>().unwrap();

        DeckBuilder::new(MAP_WIDTH, MAP_HEIGHT)
            .with_engines()
            .with_walls(rng)
            .build()
    };

    // map.reveal();

    let mut spawn_points = map.spawn_points().iter().copied();

    if let Some(coord) = spawn_points.next() {
        log::debug!("Spawning player at {coord:?}");

        let player = player(world.create_entity()).with(coord).build();

        world.insert(player);
        // world.insert(pos);
        // world.insert(Inventory::default());
    }

    let regions = {
        let rng = world.get_mut::<RandomNumberGenerator>().unwrap();
        generate_regions(&map, rng)
    };
    let spawn_dice = DiceType::new(1, 6, -3);

    for region in regions {
        spawn_region(world, &region, spawn_dice);
    }

    world.insert(map);
}

fn spawn_table(rng: &mut RandomNumberGenerator) -> impl FnOnce(EntityBuilder) -> EntityBuilder {
    match rng.range(0, 2) {
        0 => monster::infected_crewmember,
        _ => monster::alien_hatchling,
    }
}

/// Randomly subdivides the map into regions.
///
/// Inspired by https://bfnightly.bracketproductions.com/rustbook/chapter_27.html#grouped-placement-in-our-map---enter-the-voronoi
fn generate_regions(map: &Map, rng: &mut RandomNumberGenerator) -> Vec<Vec<Coordinate>> {
    let mut noise = FastNoise::seeded(rng.rand());
    noise.set_noise_type(NoiseType::Cellular);
    noise.set_frequency(0.08); // Magic number, tweak for desired results.
    noise.set_cellular_distance_function(CellularDistanceFunction::Manhattan);

    let mut noise_regions: HashMap<u32, Vec<Coordinate>> = HashMap::new();

    for c in map.iter().filter(|&c| !map[c].is_blocked()) {
        let region_id = noise
            .get_noise3d(c.q as f32, c.r as f32, (-c.q - c.r) as f32)
            .to_bits();

        if noise_regions.contains_key(&region_id) {
            noise_regions.get_mut(&region_id).unwrap().push(c);
        } else {
            noise_regions.insert(region_id, vec![c]);
        }
    }

    noise_regions.into_iter().map(|(_, v)| v).collect()
}

fn spawn_region(world: &mut World, region: &[Coordinate], spawn_dice: DiceType) {
    let mut spawns = HashMap::new();

    {
        let rng = world.get_mut::<RandomNumberGenerator>().unwrap();

        let num_spawns: usize = (rng.roll(spawn_dice))
            .clamp(0, region.len() as i32)
            .try_into()
            .unwrap_or_default();

        while spawns.len() < num_spawns {
            let coord = rng.random_slice_entry(region).unwrap();
            if !spawns.contains_key(coord) {
                spawns.insert(*coord, spawn_table(rng));
            }
        }
    }

    for (coord, builder) in spawns {
        builder(world.create_entity()).with(coord).build();
    }
}
