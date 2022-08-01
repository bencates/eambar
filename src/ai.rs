use crate::{game_mechanics::in_melee_range, prelude::*};

pub struct MonsterAI;

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        ReadExpect<'a, Map>,
        ReadExpect<'a, Entity>,
        Entities<'a>,
        Intents<'a>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Coordinate>,
        ReadStorage<'a, Viewshed>,
    );

    fn run(
        &mut self,
        (map, player, entities, mut intents, monsters, coordinates, viewsheds): Self::SystemData,
    ) {
        let player_coord = *coordinates.get(*player).unwrap();

        for (entity, _, &coord, vs) in (&entities, &monsters, &coordinates, &viewsheds).join() {
            if !vs.is_visible(player_coord) {
                continue;
            }

            if in_melee_range(coord, player_coord) {
                intents.wants_to_melee(entity, *player);
            } else if let Some(dest) = map.path(coord, player_coord).and_then(|mut p| p.nth(1)) {
                intents.wants_to_move(entity, dest)
            }
        }
    }
}
