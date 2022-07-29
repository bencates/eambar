use crate::prelude::*;

#[derive(Component)]
pub struct WantsToMelee(pub(super) Entity);

pub fn in_melee_range(from: Coordinate, to: Coordinate) -> bool {
    from.distance(to) <= 1
}

pub struct MeleeCombatSystem;

impl<'a> System<'a> for MeleeCombatSystem {
    type SystemData = (
        ReadStorage<'a, Name>,
        ReadStorage<'a, CharacterSheet>,
        WriteStorage<'a, WantsToMelee>,
        // WriteExpect<'a, GameLog>
    );

    fn run(
        &mut self,
        (
            names,
            character_sheets,
            mut melee_intents,
            // mut game_log,
        ): Self::SystemData,
    ) {
        for (attacker_name, attacker, target_name, target) in
            (&names, &character_sheets, &melee_intents)
                .join()
                .filter_map(|(attacker_name, attacker, &WantsToMelee(target_entity))| {
                    Some((
                        attacker_name,
                        attacker,
                        names.get(target_entity)?,
                        character_sheets.get(target_entity)?,
                    ))
                })
                .filter(|(_, attacker, _, target)| attacker.is_alive() && target.is_alive())
        {
            let raw_damage = attacker.melee_damage();
            let damage = target.block_damage(raw_damage);

            if damage > 0 {
                // TODO: log to game_log
                log::info!("{attacker_name} hits {target_name} for {damage} damage");
            } else {
                // TODO: log to game_log
                log::info!("{attacker_name} is unable to hurt {target_name}.");
            }
        }

        melee_intents.clear();
    }
}
