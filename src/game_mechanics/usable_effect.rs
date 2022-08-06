use super::InInventory;
use crate::prelude::*;

#[derive(Component)]
pub enum Usable {
    OnSelf,
    OnTarget { range: i32 },
    // OnGround { range: i32, radius: i32 }
}

#[derive(Component)]
pub struct BeingUsed(pub(super) Entity);

#[derive(Component)]
pub struct DealsDamage(pub i32);

#[derive(Component)]
pub struct ProvidesHealing(pub i32);

pub struct ItemUseSystem;

impl<'a> System<'a> for ItemUseSystem {
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
        for (item, item_type, &BeingUsed(target), item_name, damage, healing) in (
            &entities,
            item_types.maybe(),
            &item_use_intents,
            &names,
            deals_damage.maybe(),
            provides_healing.maybe(),
        )
            .join()
        {
            if let Some(durability) = durabilities.get_mut(target) {
                if let Some(&DealsDamage(raw_damage)) = damage {
                    let blocked_damage = durability.block_damage(raw_damage);
                    durability.apply_damage(blocked_damage);
                    if let Some(target_name) = names.get(target) {
                        game_log.damage(item_name, target_name, blocked_damage);
                    }
                }

                if let Some(&ProvidesHealing(amount)) = healing {
                    durability.heal(amount);
                    if let Some(target_name) = names.get(target) {
                        game_log.healing(item_name, target_name, amount);
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
