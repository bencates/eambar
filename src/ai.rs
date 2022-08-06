use crate::prelude::*;

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
        ReadStorage<'a, Usable>,
    );

    fn run(
        &mut self,
        (map, player, entities, mut intents, monsters, coordinates, viewsheds, usables): Self::SystemData,
    ) {
        let player_coord = *coordinates.get(*player).unwrap();

        for (entity, _, &coord, vs, usable) in (
            &entities,
            &monsters,
            &coordinates,
            &viewsheds,
            usables.maybe(),
        )
            .join()
        {
            if !vs.is_visible(player_coord) {
                continue;
            }

            let can_use_self_on_player = match usable {
                Some(&Usable::OnTarget { range }) => coord.distance(player_coord) <= range,
                _ => false,
            };

            if can_use_self_on_player {
                intents.wants_to_use(entity, *player);
            } else if let Some(dest) = map.path(coord, player_coord).and_then(|mut p| p.nth(1)) {
                intents.wants_to_move(entity, dest)
            }
        }
    }
}
