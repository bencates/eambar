mod movement;

pub use movement::*;

use crate::prelude::*;

/// Actions represent all of the possible this which consume a turn.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Action {
    Move(Direction),
}

pub enum ActionError {
    MovementBlocked,
}

pub fn take_action(world: &mut World, entity: Entity, action: Action) -> Result<(), ActionError> {
    let mut intents = Intents::fetch(&world);

    match action {
        Action::Move(direction) => {
            let map = world.fetch::<Map>();
            let coordinates = world.read_component::<Coordinate>();

            if let Some(coord) = coordinates.get(entity) {
                let dest = *coord + direction;

                if map[(dest).into()].is_blocked() {
                    log::debug!("Movement blocked");
                    return Err(ActionError::MovementBlocked);
                }

                intents.wants_to_move(entity, dest);
            }

            Ok(())
        }
    }
}

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
