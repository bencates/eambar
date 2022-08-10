use crate::{game_mechanics::HasInitiative, prelude::*};

pub struct MonsterAISystem;

impl<'a> System<'a> for MonsterAISystem {
    type SystemData = (
        ReadExpect<'a, Map>,
        ReadExpect<'a, Entity>,
        Entities<'a>,
        Intents<'a>,
        ReadStorage<'a, Monster>,
        WriteStorage<'a, Initiative>,
        WriteStorage<'a, HasInitiative>,
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
            monsters,
            mut initiatives,
            mut has_initiative,
            coordinates,
            viewsheds,
            usables,
        ): Self::SystemData,
    ) {
        let player_coord = *coordinates.get(*player).unwrap();
        let mut had_initiative = None;

        for (entity, _, _, initiative, &coord, vs, usable) in (
            &entities,
            &monsters,
            &has_initiative,
            &mut initiatives,
            &coordinates,
            &viewsheds,
            usables.maybe(),
        )
            .join()
        {
            had_initiative = Some(entity);
            initiative.current = initiative.speed;

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

        if let Some(entity) = had_initiative {
            has_initiative.remove(entity).unwrap();
        }
    }
}
