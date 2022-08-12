mod durability;
mod effect;
mod field_of_view;
mod initiative;
mod inventory;
mod movement;

pub use durability::*;
pub use effect::*;
pub use field_of_view::*;
pub use initiative::*;
pub use inventory::*;
pub use movement::*;

use crate::{ai::MonsterAISystem, map::IndexMapSystem, prelude::*, targeting::ClearTargetSystem};

pub fn dispatcher<'a, 'b>(world: &mut World) -> Dispatcher<'a, 'b> {
    DispatcherBuilder::new()
        .with(InitiativeSystem, "initiative", &[])
        .with(MonsterAISystem, "monster_ai", &["initiative"])
        .with(MovementSystem, "movement", &["monster_ai"])
        .with(ItemPickupSystem, "item_pickup", &[])
        .with(EffectUseSystem, "effect_use", &[])
        .with(VisibilitySystem::new(world), "visibility", &["movement"])
        .with(ShieldRegenSystem, "shield_regen", &["effect_use"])
        .with(DeathSystem, "death", &["effect_use"])
        .with(
            PlayerInventorySystem,
            "player_inventory",
            &["item_pickup", "effect_use"],
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
}
