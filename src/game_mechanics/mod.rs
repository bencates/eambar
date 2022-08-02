mod character_sheet;
mod field_of_view;
mod inventory;
mod melee_combat;
mod movement;

pub use character_sheet::*;
pub use field_of_view::*;
pub use inventory::*;
pub use melee_combat::*;
pub use movement::*;

use crate::prelude::*;

#[derive(SystemData)]
pub struct Intents<'a> {
    wants_to_move: WriteStorage<'a, WantsToMove>,
    wants_to_melee: WriteStorage<'a, WantsToMelee>,
    wants_to_pick_up: WriteStorage<'a, WantsToPickUp>,
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
}
