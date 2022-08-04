use crate::game_mechanics::is_legal_move;
use crate::prelude::*;

/// Actions represent all of the possible this which consume a turn.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Action {
    Move(Direction),
    GetItem,
    UseItem(usize),
}

pub enum ActionError {
    MovementBlocked,
    NothingToPickUp,
    NotUsable,
}

pub fn try_action(world: &mut World, action: Action) -> Result<(), ActionError> {
    log::debug!("Player action: {:?}", action);

    let map = world.fetch::<Map>();
    let player = *world.fetch::<Entity>();
    let pos = *world.read_component::<Coordinate>().get(player).unwrap();
    let mut intents = Intents::fetch(world);

    match action {
        Action::Move(direction) => {
            let character_sheets = world.read_component::<CharacterSheet>();

            let dest = pos + direction;

            if let Some(target) = map[dest].entity(&character_sheets) {
                intents.wants_to_melee(player, target);
            } else {
                if !is_legal_move(&map, dest) {
                    log::debug!("Movement blocked");
                    return Err(ActionError::MovementBlocked);
                }

                intents.wants_to_move(player, dest);
            }

            Ok(())
        }
        Action::GetItem => {
            let items = world.read_component::<Item>();

            if let Some(item) = map[pos].entity(&items) {
                intents.wants_to_pick_up(player, item);
                Ok(())
            } else {
                log::debug!("Nothing here to pick up.");
                Err(ActionError::NothingToPickUp)
            }
        }
        Action::UseItem(idx) => {
            let inventory = world.fetch::<Inventory>();
            if let Some(&item) = inventory.0.get(idx) {
                intents.wants_to_use(player, item);

                Ok(())
            } else {
                let label = (b'A' + idx as u8) as char;
                log::debug!("No item \"{label}\"");

                Err(ActionError::NotUsable)
            }
        }
    }
}
