use crate::game_mechanics::is_legal_move;
use crate::prelude::*;

pub fn handle_input(ctx: &BTerm, world: &mut World) -> RunState {
    use {Direction::*, RunState::*, VirtualKeyCode::*};
    let mut player_turn = PlayerTurn::fetch(world);

    ctx.key.map_or(AwaitingInput, |key| {
        if ctx.control {
            let index = letter_to_option(key);
            if index >= 0 {
                return player_turn.use_item(index as usize);
            }
        }

        match key {
            // Movement keys
            Q => player_turn.attack_or_move(NorthWest),
            W => player_turn.attack_or_move(North),
            E => player_turn.attack_or_move(NorthEast),
            A => player_turn.attack_or_move(SouthWest),
            S => player_turn.attack_or_move(South),
            D => player_turn.attack_or_move(SouthEast),

            Tab => player_turn.cycle_target(ctx.shift),

            G => player_turn.pick_up_item(),

            Escape => Quitting,

            _ => AwaitingInput,
        }
    })
}

#[derive(SystemData)]
pub struct PlayerTurn<'a> {
    map: ReadExpect<'a, Map>,
    player: ReadExpect<'a, Entity>,
    inventory: Read<'a, Inventory>,
    lazy: Read<'a, LazyUpdate>,
    intents: Intents<'a>,
    targeting: Targeting<'a>,
    effect_usage: EffectUsage<'a>,
    initiative_data: InitiativeData<'a>,
    entities: Entities<'a>,
    positions: ReadStorage<'a, Coordinate>,
    monsters: ReadStorage<'a, Monster>,
    items: ReadStorage<'a, Item>,
    usables: ReadStorage<'a, Usable>,
    viewsheds: ReadStorage<'a, Viewshed>,
}

impl<'a> PlayerTurn<'a> {
    pub fn attack_or_move(&mut self, direction: Direction) -> RunState {
        let pos = *self.positions.get(*self.player).unwrap();

        let dest = pos + direction;

        if let Some(target) = self.map[dest].entity(&self.monsters) {
            self.targeting.set_target(*self.player, Some(target));
            self.effect_usage
                .use_on_target(*self.player, *self.player, target)
                .unwrap();
            self.initiative_data.spend_turn(*self.player);
        } else {
            if !is_legal_move(&self.map, dest) {
                log::warn!("Movement blocked");
                return RunState::AwaitingInput;
            }

            log::debug!("Moving to {dest:?}");
            self.intents.wants_to_move(*self.player, dest);
            self.initiative_data.spend_turn(*self.player);
        }

        RunState::Running
    }

    pub fn pick_up_item(&mut self) -> RunState {
        let pos = *self.positions.get(*self.player).unwrap();

        if let Some(item) = self.map[pos].entity(&self.items) {
            self.intents.wants_to_pick_up(*self.player, item);
            self.initiative_data.spend_turn(*self.player);

            RunState::Running
        } else {
            log::warn!("Nothing here to pick up.");

            RunState::AwaitingInput
        }
    }

    pub fn use_item(&mut self, index: usize) -> RunState {
        let item = match self.inventory.0.get(index) {
            Some(&item) => item,
            None => {
                let label = (b'A' + index as u8) as char;
                log::debug!("No item \"{label}\"");
                return RunState::AwaitingInput;
            }
        };

        let usable = match self.usables.get(item) {
            Some(&usable) => usable,
            None => {
                log::debug!("Item {item:?} is not usable");
                return RunState::AwaitingInput;
            }
        };

        match usable {
            Usable::OnSelf => match self.effect_usage.use_on_self(item, *self.player) {
                Ok(()) => {
                    self.initiative_data.spend_turn(*self.player);
                    RunState::Running
                }
                Err(reason) => {
                    log::error!("{reason:?}");
                    RunState::AwaitingInput
                }
            },
            Usable::OnTarget { .. } => {
                let target = match self.targeting.get(*self.player) {
                    Some(&Target(target)) => target,
                    None => {
                        log::error!("no target");
                        return RunState::AwaitingInput;
                    }
                };

                match self.effect_usage.use_on_target(item, *self.player, target) {
                    Ok(()) => {
                        self.initiative_data.spend_turn(*self.player);
                        RunState::Running
                    }
                    Err(reason) => {
                        log::error!("{reason:?}");
                        RunState::AwaitingInput
                    }
                }
            }
            Usable::OnGround { range } => {
                let player_pos = *self.positions.get(*self.player).unwrap();
                let targeting_reticule = TargetingReticule::new(player_pos, range, &self.map);

                self.lazy.exec_mut(|world| world.insert(targeting_reticule));

                RunState::TargetGround(item)
            }
        }
    }

    pub fn cycle_target(&mut self, rev: bool) -> RunState {
        let viewshed = self.viewsheds.get(*self.player).unwrap();

        let potential_targets: Vec<_> = (&self.entities, &self.positions, &self.monsters)
            .join()
            .filter(|&(_, &pos, _)| viewshed.is_visible(pos))
            .map(|(entity, _, _)| entity)
            .collect();

        if rev {
            self.targeting.prev_target(*self.player, &potential_targets)
        } else {
            self.targeting.next_target(*self.player, &potential_targets)
        }

        RunState::AwaitingInput
    }
}
