mod character_sheet;
mod field_of_view;
mod inventory;
mod item;
mod melee_combat;
mod movement;

pub use character_sheet::*;
pub use field_of_view::*;
pub use inventory::*;
pub use item::*;
pub use melee_combat::*;
pub use movement::*;

use crate::{ai::MonsterAI, map::IndexMapSystem, prelude::*, target::ClearTargetSystem};

pub fn dispatcher<'a, 'b>(world: &mut World) -> Dispatcher<'a, 'b> {
    DispatcherBuilder::new()
        .with(MonsterAI, "monster_ai", &[])
        .with(MovementSystem, "movement", &["monster_ai"])
        .with(ItemPickupSystem, "item_pickup", &[])
        .with(ItemUseSystem, "item_use", &[])
        .with(MeleeCombatSystem, "melee_combat", &["monster_ai"])
        .with(VisibilitySystem::new(world), "visibility", &["movement"])
        .with(DeathSystem, "death", &["melee_combat"])
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
    wants_to_melee: WriteStorage<'a, WantsToMelee>,
    wants_to_pick_up: WriteStorage<'a, WantsToPickUp>,
    wants_to_use: WriteStorage<'a, WantsToUse>,
}

impl<'a> Intents<'a> {
    pub fn wants_to_move(&mut self, entity: Entity, dest: Coordinate) {
        self.wants_to_move
            .insert(entity, WantsToMove(dest))
            .expect("could not queue move intent");
    }

    pub fn wants_to_melee(&mut self, attacker: Entity, target: Entity) {
        self.wants_to_melee
            .insert(attacker, WantsToMelee(target))
            .expect("could not queue melee attack intent");
    }

    pub fn wants_to_pick_up(&mut self, recipient: Entity, item: Entity) {
        self.wants_to_pick_up
            .insert(recipient, WantsToPickUp(item))
            .expect("could not queue item pickup intent");
    }

    pub fn wants_to_use(&mut self, user: Entity, item: Entity) {
        self.wants_to_use
            .insert(user, WantsToUse(item))
            .expect("could not queue item use intent");
    }
}
