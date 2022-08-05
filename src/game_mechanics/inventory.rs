use crate::prelude::*;

#[derive(Component)]
pub struct WantsToPickUp(pub(super) Entity);

#[derive(Default)]
pub struct Inventory(pub Vec<Entity>);

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
        ReadStorage<'a, Appearance>,
        WriteStorage<'a, InInventory>,
    );

    fn run(
        &mut self,
        (
            player,
            mut game_log,
            entities,
            mut pickup_intents,
            mut positions,
            appearances,
            mut inventories,
        ): Self::SystemData,
    ) {
        for (recipient, &WantsToPickUp(item)) in (&entities, &pickup_intents).join() {
            positions.remove(item);
            inventories.insert(item, InInventory(recipient)).unwrap();

            if recipient == *player {
                if let Some(item_appearance) = appearances.get(item) {
                    game_log.player_pickup(item_appearance);
                }
            }
        }

        pickup_intents.clear();
    }
}

pub struct PlayerInventorySystem;

impl<'a> System<'a> for PlayerInventorySystem {
    type SystemData = (
        Write<'a, Inventory>,
        ReadExpect<'a, Entity>,
        Entities<'a>,
        WriteStorage<'a, InInventory>,
    );

    fn run(&mut self, (mut inventory, player, entities, in_inventories): Self::SystemData) {
        let player_inventory: Vec<_> = (&entities, &in_inventories)
            .join()
            .filter_map(|(item, &InInventory(owner))| (owner == *player).then(|| item))
            .collect();

        inventory
            .0
            .retain(|&item| entities.is_alive(item) && player_inventory.contains(&item));

        for item in player_inventory {
            if !inventory.0.contains(&item) {
                inventory.0.push(item);
            }
        }
    }
}
