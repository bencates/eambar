use crate::action::Intents;
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
    );

    fn run(
        &mut self,
        (map, player, entities, mut intents, monsters, coordinates, viewsheds): Self::SystemData,
    ) {
        let player_coord = *coordinates.get(*player).unwrap();

        for (entity, _, coord, vs) in (&entities, &monsters, &coordinates, &viewsheds).join() {
            if !vs.is_visible(player_coord) {
                continue;
            }

            // if data.in_melee_range(monster) {
            //     data.attack_player(monster);
            // } else {

            if let Some(path) = map.path(*coord, player_coord) {
                if let Some(dest) = path.skip(1).next() {
                    intents.wants_to_move(entity, dest)
                }
            }
        }
    }
}
