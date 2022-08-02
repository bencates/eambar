use crate::prelude::*;

#[derive(Component)]
pub struct WantsToPickUp(pub(super) Entity);

#[derive(Component)]
pub struct InInventory(pub(super) Entity);

pub struct ItemPickupSystem;

impl<'a> System<'a> for ItemPickupSystem {
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        Entities<'a>,
        WriteStorage<'a, WantsToPickUp>,
        WriteStorage<'a, Coordinate>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, InInventory>,
    );

    fn run(
        &mut self,
        (player, mut game_log, entities, mut pickup_intents, mut positions, names, mut inventories): Self::SystemData,
    ) {
        for (recipient, &WantsToPickUp(item)) in (&entities, &pickup_intents).join() {
            positions.remove(item);
            inventories.insert(item, InInventory(recipient)).unwrap();

            if recipient == *player {
                if let Some(item_name) = names.get(item) {
                    game_log.player_pickup(item_name);
                }
            }
        }

        pickup_intents.clear();
    }
}
