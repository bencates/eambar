use crate::prelude::*;

#[derive(Component)]
pub struct WantsToMelee(pub(super) Entity);

pub fn in_melee_range(from: Coordinate, to: Coordinate) -> bool {
    from.distance(to) <= 1
}

pub struct MeleeCombatSystem;

impl<'a> System<'a> for MeleeCombatSystem {
    type SystemData = (
        ReadStorage<'a, Appearance>,
        WriteStorage<'a, CharacterSheet>,
        WriteStorage<'a, WantsToMelee>,
        Write<'a, GameLog>,
    );

    fn run(
        &mut self,
        (appearances, mut character_sheets, mut melee_intents, mut game_log): Self::SystemData,
    ) {
        let mut damage_taken = ChangeSet::new();

        for (attacker_appearance, attacker, target_entity, target_name, target) in
            (&appearances, &character_sheets, &melee_intents)
                .join()
                .filter_map(|(attacker_name, attacker, &WantsToMelee(target_entity))| {
                    Some((
                        attacker_name,
                        attacker,
                        target_entity,
                        appearances.get(target_entity)?,
                        character_sheets.get(target_entity)?,
                    ))
                })
                .filter(|(_, attacker, _, _, target)| attacker.is_alive() && target.is_alive())
        {
            let raw_damage = attacker.melee_damage();
            let damage = target.block_damage(raw_damage);

            game_log.damage(attacker_appearance, target_name, damage);

            if damage > 0 {
                damage_taken.add(target_entity, damage);
            }
        }

        for (&damage, target) in (&damage_taken, &mut character_sheets).join() {
            target.apply_damage(damage);
        }

        melee_intents.clear();
    }
}
