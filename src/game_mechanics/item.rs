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
        WriteStorage<'a, CharacterSheet>,
        WriteStorage<'a, BeingUsed>,
        ReadStorage<'a, DealsDamage>,
        ReadStorage<'a, ProvidesHealing>,
        WriteStorage<'a, InInventory>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut character_sheets,
            mut item_use_intents,
            deals_damage,
            provides_healing,
            mut in_inventories,
        ): Self::SystemData,
    ) {
        for (item, &BeingUsed(target), damage, healing) in (
            &entities,
            &item_use_intents,
            deals_damage.maybe(),
            provides_healing.maybe(),
        )
            .join()
        {
            if let Some(character_sheet) = character_sheets.get_mut(target) {
                if let Some(&DealsDamage(raw_damage)) = damage {
                    let blocked_damage = character_sheet.block_damage(raw_damage);
                    character_sheet.apply_damage(blocked_damage);
                }

                if let Some(&ProvidesHealing(amount)) = healing {
                    character_sheet.heal(amount);
                }
            }

            // Removing the inventory marker clears the entity off the inventory
            // immediately. All other components will be removed automatically
            // after the turn.
            in_inventories.remove(item).unwrap();
            entities.delete(item).unwrap();
        }

        item_use_intents.clear();
    }
}
