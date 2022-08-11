use crate::prelude::*;

pub struct MonsterAISystem;

impl<'a> System<'a> for MonsterAISystem {
    type SystemData = (
        ReadExpect<'a, Map>,
        ReadExpect<'a, Entity>,
        Entities<'a>,
        Intents<'a>,
        EffectUsage<'a>,
        InitiativeData<'a>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Coordinate>,
        ReadStorage<'a, Viewshed>,
    );

    fn run(
        &mut self,
        (
            map,
            player,
            entities,
            mut intents,
            mut effect_usage,
            mut initiative_data,
            monsters,
            coordinates,
            viewsheds,
        ): Self::SystemData,
    ) {
        let player_coord = *coordinates.get(*player).unwrap();
        let mut had_initiative: SmallVec<[Entity; 1]> = SmallVec::new();

        for (entity, _, _, &coord, vs) in (
            &entities,
            &monsters,
            initiative_data.has_initiative(),
            &coordinates,
            &viewsheds,
        )
            .join()
        {
            had_initiative.push(entity);

            if !vs.is_visible(player_coord) {
                continue;
            }

            match effect_usage.use_on_target(entity, entity, *player) {
                Ok(()) => {}
                Err(_) => {
                    if let Some(dest) = map.path(coord, player_coord).and_then(|mut p| p.nth(1)) {
                        intents.wants_to_move(entity, dest)
                    }
                }
            }
        }

        for entity in had_initiative {
            initiative_data.spend_turn(entity);
        }
    }
}
