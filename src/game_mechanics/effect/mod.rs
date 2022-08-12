mod usage;

pub use usage::EffectUsage;

use super::InInventory;
use crate::prelude::*;

#[derive(Component, Clone, Copy)]
#[allow(clippy::enum_variant_names)]
pub enum Usable {
    OnSelf,
    OnTarget { range: i32 },
    OnGround { range: i32 },
}

#[derive(Component)]
pub struct BeingUsed(pub(super) SmallVec<[Entity; 1]>);

#[derive(Component)]
pub struct DealsDamage(pub i32);

#[derive(Component)]
pub struct ProvidesHealing(pub i32);

pub struct EffectUseSystem;

impl<'a> System<'a> for EffectUseSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Item>,
        WriteStorage<'a, Durability>,
        WriteStorage<'a, BeingUsed>,
        ReadStorage<'a, DealsDamage>,
        ReadStorage<'a, ProvidesHealing>,
        WriteStorage<'a, InInventory>,
        ReadStorage<'a, Appearance>,
        Write<'a, GameLog>,
    );

    fn run(
        &mut self,
        (
            entities,
            item_types,
            mut durabilities,
            mut item_use_intents,
            deals_damage,
            provides_healing,
            mut in_inventories,
            names,
            mut game_log,
        ): Self::SystemData,
    ) {
        for (item, item_type, BeingUsed(targets), item_name, damage, healing) in (
            &entities,
            item_types.maybe(),
            &item_use_intents,
            &names,
            deals_damage.maybe(),
            provides_healing.maybe(),
        )
            .join()
        {
            for &target in targets {
                if let Some(durability) = durabilities.get_mut(target) {
                    if let Some(&DealsDamage(raw_damage)) = damage {
                        let blocked_damage = durability.take_damage(raw_damage);
                        if let Some(target_name) = names.get(target) {
                            game_log.damage(item_name, target_name, blocked_damage);
                        }
                    }

                    if let Some(&ProvidesHealing(amount)) = healing {
                        let amount = durability.heal(amount);
                        if let Some(target_name) = names.get(target) {
                            game_log.healing(item_name, target_name, amount);
                        }
                    }
                }
            }

            if item_type == Some(&Item::Consumable) {
                // Removing the inventory marker clears the entity from the player's
                // inventory immediately. All other components will be removed
                // automatically after the turn.
                in_inventories.remove(item).unwrap();
                entities.delete(item).unwrap();
            }
        }

        item_use_intents.clear();
    }
}
