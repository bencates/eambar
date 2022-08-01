use crate::entity::SpawnBuilder;
use crate::prelude::*;
use std::collections::HashMap;

/// Randomly subdivides the map into regions.
///
/// Inspired by https://bfnightly.bracketproductions.com/rustbook/chapter_27.html#grouped-placement-in-our-map---enter-the-voronoi
pub fn generate_regions(map: &Map, rng: &mut RandomNumberGenerator) -> Vec<Vec<Coordinate>> {
    let mut noise = FastNoise::seeded(rng.rand());
    noise.set_noise_type(NoiseType::Cellular);
    noise.set_frequency(0.08); // Magic number, tweak for desired results.
    noise.set_cellular_distance_function(CellularDistanceFunction::Manhattan);

    let mut noise_regions: HashMap<u32, Vec<Coordinate>> = HashMap::new();

    for c in map.iter().filter(|&c| !map[c].is_blocked()) {
        let region_id = noise
            .get_noise3d(c.q as f32, c.r as f32, (-c.q - c.r) as f32)
            .to_bits();

        noise_regions.entry(region_id).or_default().push(c);
    }

    noise_regions.into_iter().map(|(_, v)| v).collect()
}

pub fn spawn_region(
    rng: &mut RandomNumberGenerator,
    region: &[Coordinate],
    spawn_table: &[SpawnBuilder],
    spawn_dice: DiceType,
) -> HashMap<Coordinate, SpawnBuilder> {
    let mut spawns = HashMap::new();

    let num_spawns: usize = (rng.roll(spawn_dice))
        .clamp(0, region.len() as i32)
        .try_into()
        .unwrap_or_default();

    while spawns.len() < num_spawns {
        let coord = rng.random_slice_entry(region).unwrap();
        if !spawns.contains_key(coord) {
            let spawn = rng.random_slice_entry(spawn_table).unwrap();
            spawns.insert(*coord, *spawn);
        }
    }

    spawns
}
