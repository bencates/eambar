use std::ops::ControlFlow;

use crate::{
    game_mechanics::{self, HasInitiative},
    level::build_level,
    player_turn,
    prelude::*,
    ui,
};
use RunState::*;

#[derive(Clone, Copy, PartialEq)]
pub enum RunState {
    /// Show the main menu
    MainMenu,
    /// Initialize the world
    NewGame,
    AwaitingInput,
    TargetGround(Entity),
    Running,
    Quitting,
}

pub struct GameEngine {
    world: World,
    dispatcher: Dispatcher<'static, 'static>,
    ui_dispatcher: Dispatcher<'static, 'static>,
    run_state: RunState,
}

impl GameState for GameEngine {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.run_state = match self.run_state {
            MainMenu => ui::main_menu(ctx),
            NewGame => {
                build_level(&mut self.world);

                self.run()
            }
            AwaitingInput => player_turn::handle_input(ctx, &mut self.world),
            TargetGround(effect) => {
                let res = self
                    .world
                    .fetch_mut::<TargetingReticule>()
                    .handle_input(ctx);

                match res {
                    ControlFlow::Continue(()) => TargetGround(effect),
                    ControlFlow::Break(coord) => {
                        self.world.remove::<TargetingReticule>();

                        match coord {
                            Some(pos) => {
                                let map = self.world.fetch::<Map>();
                                let durabilities = self.world.read_component::<Durability>();

                                if let Some(target) = map[pos].entity(&durabilities) {
                                    let mut intents = Intents::fetch(&self.world);

                                    intents.wants_to_use(effect, target);
                                }

                                let player = *self.world.fetch::<Entity>();
                                let mut has_initiative =
                                    self.world.write_component::<HasInitiative>();
                                let mut initiatives = self.world.write_component::<Initiative>();
                                let initiative = initiatives.get_mut(player).unwrap();

                                initiative.current = initiative.speed;
                                has_initiative.remove(player).unwrap();

                                Running
                            }
                            None => AwaitingInput,
                        }
                    }
                }
            }
            Running => self.run(),
            Quitting => return ctx.quit(),
        };

        self.ui_dispatcher.dispatch(&self.world);
        render_draw_buffer(ctx).unwrap();
    }
}

impl GameEngine {
    pub fn new() -> Self {
        let mut world = World::new();

        let mut dispatcher = game_mechanics::dispatcher(&mut world);
        let mut ui_dispatcher = ui::dispatcher(&mut world);

        dispatcher.setup(&mut world);
        ui_dispatcher.setup(&mut world);
        world.register::<Usable>();

        world.insert(RandomNumberGenerator::new());

        Self {
            world,
            dispatcher,
            ui_dispatcher,
            run_state: NewGame,
        }
    }

    fn run(&mut self) -> RunState {
        let start = std::time::Instant::now();

        while !self.player_has_initiative() {
            self.dispatcher.dispatch(&self.world);
            self.world.maintain();
        }

        log::debug!("Game world update took {:?}", start.elapsed());

        AwaitingInput
    }

    fn player_has_initiative(&self) -> bool {
        let player = *self.world.fetch::<Entity>();
        let has_initiative = self.world.read_component::<HasInitiative>();

        has_initiative.contains(player)
    }
}
