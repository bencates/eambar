use crate::prelude::*;

pub struct MonsterAI;

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        ReadExpect<'a, Map>,
        ReadExpect<'a, Entity>,
        ReadStorage<'a, Monster>,
        WriteStorage<'a, Coordinate>,
        WriteStorage<'a, Viewshed>,
    );

    fn run(&mut self, (map, player, monsters, mut coordinates, mut viewsheds): Self::SystemData) {
        let player_coord = *coordinates.get(*player).unwrap();

        for (_, coord, vs) in (&monsters, &mut coordinates, &mut viewsheds).join() {
            if !vs.is_visible(player_coord) {
                continue;
            }

            // if data.in_melee_range(monster) {
            //     data.attack_player(monster);
            // } else {

            if let Some(path) = map.path(*coord, player_coord) {
                if let Some(next_coord) = path.skip(1).next() {
                    *coord = next_coord;
                    vs.touch();
                }
            }
        }
    }
}
