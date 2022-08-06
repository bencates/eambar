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
        ReadStorage<'a, DealsDamage>,
        WriteStorage<'a, Durability>,
        WriteStorage<'a, WantsToMelee>,
        Write<'a, GameLog>,
    );

    fn run(
        &mut self,
        (appearances, damages, mut durabilities, mut melee_intents, mut game_log): Self::SystemData,
    ) {
        let mut damage_taken = ChangeSet::new();

        for (attacker_appearance, attacker_damage, _, target_entity, target_name, target) in
            (&appearances, &damages, &durabilities, &melee_intents)
                .join()
                .filter_map(
                    |(attacker_name, attacker_damage, attacker, &WantsToMelee(target_entity))| {
                        Some((
                            attacker_name,
                            attacker_damage,
                            attacker,
                            target_entity,
                            appearances.get(target_entity)?,
                            durabilities.get(target_entity)?,
                        ))
                    },
                )
                .filter(|(_, _, attacker, _, _, target)| attacker.is_alive() && target.is_alive())
        {
            let damage = target.block_damage(attacker_damage.0);

            game_log.damage(attacker_appearance, target_name, damage);

            if damage > 0 {
                damage_taken.add(target_entity, damage);
            }
        }

        for (&damage, target) in (&damage_taken, &mut durabilities).join() {
            target.apply_damage(damage);
        }

        melee_intents.clear();
    }
}
