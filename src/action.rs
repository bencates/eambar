use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Action {
    Move(Direction),
}

pub enum ActionError {}

pub fn take_action(world: &mut World, entity: Entity, action: Action) -> Result<(), ActionError> {
    Ok(())
}
