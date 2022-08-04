use super::InInventory;
use crate::prelude::*;

#[derive(Component)]
pub struct WantsToUse(pub(super) Entity);

#[derive(Component)]
pub struct ProvidesHealing(pub i32);

pub struct ItemUseSystem;

impl<'a> System<'a> for ItemUseSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, CharacterSheet>,
        WriteStorage<'a, WantsToUse>,
        ReadStorage<'a, ProvidesHealing>,
        WriteStorage<'a, InInventory>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut character_sheets,
            mut item_use_intents,
            provides_healing,
            mut in_inventories,
        ): Self::SystemData,
    ) {
        for (character_sheet, &WantsToUse(item)) in
            (&mut character_sheets, &item_use_intents).join()
        {
            if let Some(&ProvidesHealing(amount)) = provides_healing.get(item) {
                character_sheet.heal(amount);
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
