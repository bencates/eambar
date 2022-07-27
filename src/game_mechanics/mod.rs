mod movement;

pub use movement::*;

use crate::prelude::*;
#[derive(SystemData)]
pub struct Intents<'a> {
    wants_to_move: WriteStorage<'a, WantsToMove>,
}

impl<'a> Intents<'a> {
    pub fn wants_to_move(&mut self, entity: Entity, dest: Coordinate) {
        self.wants_to_move
            .insert(entity, WantsToMove(dest))
            .expect("could not queue move intent");
    }
}
