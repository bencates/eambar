use crate::prelude::*;

#[derive(Component)]
pub struct WantsToPickUp(pub(super) Entity);

#[derive(Default)]
pub struct Inventory(pub Vec<Entity>);

#[derive(Component)]
#[storage(FlaggedStorage)]
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

pub struct PlayerInventorySystem {
    cursor: ReaderId<ComponentEvent>,
}

impl PlayerInventorySystem {
    pub fn new(world: &mut World) -> Self {
        world.register::<InInventory>();
        let mut in_inventories = world.write_component::<InInventory>();

        Self {
            cursor: in_inventories.register_reader(),
        }
    }
}

impl<'a> System<'a> for PlayerInventorySystem {
    type SystemData = (
        Write<'a, Inventory>,
        ReadExpect<'a, Entity>,
        Entities<'a>,
        WriteStorage<'a, InInventory>,
    );

    fn run(&mut self, (mut inventory, player, entities, in_inventories): Self::SystemData) {
        let (mut inserted, mut removed) = (BitSet::new(), BitSet::new());

        for event in in_inventories.channel().read(&mut self.cursor) {
            match event {
                ComponentEvent::Inserted(id) => inserted.add(*id),
                ComponentEvent::Modified(_) => unreachable!(),
                ComponentEvent::Removed(id) => removed.add(*id),
            };
        }

        for (_item, &InInventory(owner), _) in (&entities, &in_inventories, &removed).join() {
            if owner == *player {
                todo!();
                // inventory.remove(item);
            }
        }

        for (item, &InInventory(owner), _) in (&entities, &in_inventories, &inserted).join() {
            if owner == *player {
                inventory.0.push(item);
            }
        }
    }
}
