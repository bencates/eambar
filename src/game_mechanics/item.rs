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
    );

    fn run(
        &mut self,
        (entities, mut character_sheets, mut item_use_intents, provides_healing): Self::SystemData,
    ) {
        for (character_sheet, &WantsToUse(item)) in
            (&mut character_sheets, &item_use_intents).join()
        {
            if let Some(&ProvidesHealing(amount)) = provides_healing.get(item) {
                character_sheet.heal(amount);
            }

            entities.delete(item).unwrap();
        }

        item_use_intents.clear();
    }
}
