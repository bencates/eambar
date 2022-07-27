use crate::game_mechanics::is_legal_move;
use crate::prelude::*;

/// Actions represent all of the possible this which consume a turn.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Action {
    Move(Direction),
}

pub enum ActionError {
    MovementBlocked,
}

pub fn try_action(world: &mut World, action: Action) -> Result<(), ActionError> {
    log::debug!("Player action: {:?}", action);

    let player = *world.fetch::<Entity>();
    let mut intents = Intents::fetch(&world);

    match action {
        Action::Move(direction) => {
            let map = world.fetch::<Map>();
            let coordinates = world.read_component::<Coordinate>();

            if let Some(coord) = coordinates.get(player) {
                let dest = *coord + direction;

                if !is_legal_move(&map, dest) {
                    log::debug!("Movement blocked");
                    return Err(ActionError::MovementBlocked);
                }

                intents.wants_to_move(player, dest);
            }

            Ok(())
        }
    }
}
