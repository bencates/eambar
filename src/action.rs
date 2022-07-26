use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Action {
    Move(Direction),
}

pub enum ActionError {
    MovementBlocked,
}

pub fn take_action(world: &mut World, entity: Entity, action: Action) -> Result<(), ActionError> {
    match action {
        Action::Move(direction) => {
            let map = world.fetch::<Map>();
            let mut coordinates = world.write_component::<Coordinate>();

            // TODO: move to system
            if let Some(coord) = coordinates.get_mut(entity) {
                let new_coord = *coord + direction;

                if map[(new_coord).into()].is_blocked() {
                    log::debug!("Movement blocked");
                    return Err(ActionError::MovementBlocked);
                }

                *coord = new_coord;
            }

            Ok(())
        }
    }
}
