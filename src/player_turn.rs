use crate::game_mechanics::is_legal_move;
use crate::prelude::*;
use anyhow::{anyhow, ensure, Context, Result};

pub fn handle_input(ctx: &BTerm, world: &mut World) -> RunState {
    use {Direction::*, RunState::*, VirtualKeyCode::*};

    let run_state = ctx.key.map_or(Ok(AwaitingInput), |key| {
        let mut player_turn = PlayerTurn::fetch(world);

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

            Escape => Ok(Quitting),

            _ => Ok(AwaitingInput),
        }
    });

    world.maintain();

    run_state.unwrap_or_else(|reason| {
        log::warn!("{reason}");

        AwaitingInput
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
    pub fn attack_or_move(&mut self, direction: Direction) -> Result<RunState> {
        let pos = *self.positions.get(*self.player).unwrap();

        let dest = pos + direction;

        if let Some(target) = self.map[dest].entity(&self.monsters) {
            self.targeting.set_target(*self.player, Some(target));
            self.effect_usage
                .use_on_target(*self.player, *self.player, target)?;
            self.initiative_data.spend_turn(*self.player);
        } else {
            ensure!(is_legal_move(&self.map, dest), "Movement blocked");

            log::debug!("Moving to {dest:?}");
            self.intents.wants_to_move(*self.player, dest);
            self.initiative_data.spend_turn(*self.player);
        }

        Ok(RunState::Running)
    }

    pub fn pick_up_item(&mut self) -> Result<RunState> {
        let pos = *self.positions.get(*self.player).unwrap();
        let item = self.map[pos]
            .entity(&self.items)
            .context("nothing to pick up")?;

        self.intents.wants_to_pick_up(*self.player, item);
        self.initiative_data.spend_turn(*self.player);

        Ok(RunState::Running)
    }

    pub fn use_item(&mut self, index: usize) -> Result<RunState> {
        let item = *self.inventory.0.get(index).with_context(|| {
            let label = (b'A' + index as u8) as char;
            anyhow!("no item \"{label}\"")
        })?;

        match *self.usables.get(item).context("not usable")? {
            Usable::OnSelf => {
                self.effect_usage.use_on_self(item, *self.player)?;
                self.initiative_data.spend_turn(*self.player);

                Ok(RunState::Running)
            }
            Usable::OnTarget { .. } => {
                let Target(target) = *self.targeting.get(*self.player).context("no target")?;

                self.effect_usage
                    .use_on_target(item, *self.player, target)?;
                self.initiative_data.spend_turn(*self.player);

                Ok(RunState::Running)
            }
            Usable::OnGround { range } => {
                let player_pos = *self.positions.get(*self.player).unwrap();
                let targeting_reticule = TargetingReticule::new(player_pos, range, &self.map);

                self.lazy.exec_mut(|world| world.insert(targeting_reticule));

                Ok(RunState::TargetGround(item))
            }
        }
    }

    pub fn cycle_target(&mut self, rev: bool) -> Result<RunState> {
        let viewshed = self.viewsheds.get(*self.player).unwrap();

        let potential_targets: Vec<_> = (&self.entities, &self.positions, &self.monsters)
            .join()
            .filter(|&(_, &pos, _)| viewshed.is_visible(pos))
            .map(|(entity, _, _)| entity)
            .collect();

        if rev {
            self.targeting.prev_target(*self.player, &potential_targets);
        } else {
            self.targeting.next_target(*self.player, &potential_targets);
        }

        Ok(RunState::AwaitingInput)
    }
}
