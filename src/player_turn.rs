use crate::game_mechanics::is_legal_move;
use crate::prelude::*;

pub fn handle_input(ctx: &BTerm, world: &mut World) -> RunState {
    use {Direction::*, RunState::*, VirtualKeyCode::*};

    ctx.key.map_or(AwaitingInput, |key| {
        if ctx.control {
            let index = letter_to_option(key);
            if index >= 0 {
                return use_item(world, index as usize);
            }
        }

        match key {
            // Movement keys
            Q => attack_or_move(world, NorthWest),
            W => attack_or_move(world, North),
            E => attack_or_move(world, NorthEast),
            A => attack_or_move(world, SouthWest),
            S => attack_or_move(world, South),
            D => attack_or_move(world, SouthEast),

            Tab => cycle_target(world, ctx.shift),

            G => pick_up_item(world),

            Escape => Quitting,

            _ => AwaitingInput,
        }
    })
}

fn attack_or_move(world: &mut World, direction: Direction) -> RunState {
    let map = world.fetch::<Map>();
    let player = *world.fetch::<Entity>();
    let pos = *world.read_component::<Coordinate>().get(player).unwrap();
    let mut intents = Intents::fetch(world);
    let mut initiative_data = InitiativeData::fetch(world);
    let character_sheets = world.read_component::<Durability>();

    let dest = pos + direction;

    if let Some(target) = map[dest].entity(&character_sheets) {
        TargetingData::fetch(world).set_target(player, Some(target));
        // TODO: maybe check if target in range?
        intents.wants_to_use(player, target);
        initiative_data.spend_turn(player);
    } else {
        if !is_legal_move(&map, dest) {
            log::warn!("Movement blocked");
            return RunState::AwaitingInput;
        }

        log::debug!("Moving to {dest:?}");
        intents.wants_to_move(player, dest);
        initiative_data.spend_turn(player);
    }

    RunState::Running
}

fn pick_up_item(world: &mut World) -> RunState {
    let map = world.fetch::<Map>();
    let player = *world.fetch::<Entity>();
    let pos = *world.read_component::<Coordinate>().get(player).unwrap();
    let mut intents = Intents::fetch(world);
    let mut initiative_data = InitiativeData::fetch(world);
    let items = world.read_component::<Item>();

    if let Some(item) = map[pos].entity(&items) {
        intents.wants_to_pick_up(player, item);
        initiative_data.spend_turn(player);

        RunState::Running
    } else {
        log::warn!("Nothing here to pick up.");

        RunState::AwaitingInput
    }
}

fn use_item(world: &mut World, index: usize) -> RunState {
    let player = *world.fetch::<Entity>();

    let item = match world.fetch::<Inventory>().0.get(index) {
        Some(&item) => item,
        None => {
            let label = (b'A' + index as u8) as char;
            log::debug!("No item \"{label}\"");
            return RunState::AwaitingInput;
        }
    };

    let usable = match world.read_component::<Usable>().get(item) {
        Some(&usable) => usable,
        None => {
            log::debug!("Item {item:?} is not usable");
            return RunState::AwaitingInput;
        }
    };

    match usable {
        Usable::OnSelf => {
            let mut intents = Intents::fetch(world);
            let mut initiative_data = InitiativeData::fetch(world);

            intents.wants_to_use(item, player);
            initiative_data.spend_turn(player);

            RunState::Running
        }
        Usable::OnTarget { range } => {
            let mut intents = Intents::fetch(world);
            let positions = world.read_component::<Coordinate>();
            let targets = world.read_component::<Target>();
            let mut initiative_data = InitiativeData::fetch(world);

            if let Some(&Target(target)) = targets.get(player) {
                if let (Some(player_pos), Some(target_pos)) =
                    (positions.get(player), positions.get(target))
                {
                    if player_pos.distance(*target_pos) <= range {
                        log::debug!("Using {item:?} on {target:?}");

                        intents.wants_to_use(item, target);
                        initiative_data.spend_turn(player);

                        RunState::Running
                    } else {
                        log::debug!("Can't use {item:?}; target out of range");
                        RunState::AwaitingInput
                    }
                } else {
                    log::debug!("Can't use {item:?}; invalid target");
                    RunState::AwaitingInput
                }
            } else {
                log::debug!("Can't use {item:?}; no target");
                RunState::AwaitingInput
            }
        }
        Usable::OnGround { range } => {
            let targeting_reticule = {
                let player_pos = *world.read_component::<Coordinate>().get(player).unwrap();
                let map = world.fetch::<Map>();

                TargetingReticule::new(player_pos, range, &map)
            };

            world.insert(targeting_reticule);

            RunState::TargetGround(item)
        }
    }
}

fn cycle_target(world: &mut World, rev: bool) -> RunState {
    let viewsheds = world.read_component::<Viewshed>();
    let entities = world.entities();
    let monsters = world.read_component::<Monster>();
    let positions = world.read_component::<Coordinate>();
    let mut targeting_data = TargetingData::fetch(world);

    let player = *world.fetch::<Entity>();
    let viewshed = viewsheds.get(player).unwrap();

    let potential_targets: Vec<_> = (&entities, &positions, &monsters)
        .join()
        .filter(|&(_, &pos, _)| viewshed.is_visible(pos))
        .map(|(entity, _, _)| entity)
        .collect();

    let new_target = if rev {
        targeting_data.prev_target(player, &potential_targets)
    } else {
        targeting_data.next_target(player, &potential_targets)
    };

    targeting_data.set_target(player, new_target);

    RunState::AwaitingInput
}
