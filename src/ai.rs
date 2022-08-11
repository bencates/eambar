use crate::prelude::*;

pub struct MonsterAISystem;

impl<'a> System<'a> for MonsterAISystem {
    type SystemData = (
        ReadExpect<'a, Map>,
        ReadExpect<'a, Entity>,
        Entities<'a>,
        Intents<'a>,
        InitiativeData<'a>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Coordinate>,
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, Usable>,
    );

    fn run(
        &mut self,
        (
            map,
            player,
            entities,
            mut intents,
            mut initiative_data,
            monsters,
            coordinates,
            viewsheds,
            usables,
        ): Self::SystemData,
    ) {
        let player_coord = *coordinates.get(*player).unwrap();
        let mut had_initiative: SmallVec<[Entity; 1]> = SmallVec::new();

        for (entity, _, _, &coord, vs, usable) in (
            &entities,
            &monsters,
            initiative_data.has_initiative(),
            &coordinates,
            &viewsheds,
            usables.maybe(),
        )
            .join()
        {
            had_initiative.push(entity);

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

        for entity in had_initiative {
            initiative_data.spend_turn(entity);
        }
    }
}
