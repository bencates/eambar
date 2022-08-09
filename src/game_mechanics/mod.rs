mod durability;
mod field_of_view;
mod inventory;
mod movement;
mod usable_effect;

pub use durability::*;
pub use field_of_view::*;
pub use inventory::*;
pub use movement::*;
pub use usable_effect::*;

use crate::{ai::MonsterAI, map::IndexMapSystem, prelude::*, target::ClearTargetSystem};

pub fn dispatcher<'a, 'b>(world: &mut World) -> Dispatcher<'a, 'b> {
    DispatcherBuilder::new()
        .with(MonsterAI, "monster_ai", &[])
        .with(MovementSystem, "movement", &["monster_ai"])
        .with(ItemPickupSystem, "item_pickup", &[])
        .with(ItemUseSystem, "item_use", &[])
        .with(VisibilitySystem::new(world), "visibility", &["movement"])
        .with(ShieldRegenSystem, "shield_regen", &["item_use"])
        .with(DeathSystem, "death", &["item_use"])
        .with(
            PlayerInventorySystem,
            "player_inventory",
            &["item_pickup", "item_use"],
        )
        .with(ClearTargetSystem, "clear_target", &["visibility", "death"])
        .with(
            IndexMapSystem,
            "index_map",
            &["movement", "visibility", "death"],
        )
        .build()
}

#[derive(SystemData)]
pub struct Intents<'a> {
    wants_to_move: WriteStorage<'a, WantsToMove>,
    wants_to_pick_up: WriteStorage<'a, WantsToPickUp>,
    being_used: WriteStorage<'a, BeingUsed>,
}

impl<'a> Intents<'a> {
    pub fn wants_to_move(&mut self, entity: Entity, dest: Coordinate) {
        self.wants_to_move
            .insert(entity, WantsToMove(dest))
            .expect("could not queue move intent");
    }

    pub fn wants_to_pick_up(&mut self, recipient: Entity, item: Entity) {
        self.wants_to_pick_up
            .insert(recipient, WantsToPickUp(item))
            .expect("could not queue item pickup intent");
    }

    pub fn wants_to_use(&mut self, item: Entity, target: Entity) {
        self.being_used
            .insert(item, BeingUsed(target))
            .expect("could not queue item use intent");
    }
}
