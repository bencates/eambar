use crate::{
    action::take_action, ai::MonsterAI, field_of_view::VisibilitySystem, level::build_level,
    map::IndexMapSystem, prelude::*, ui,
};

#[derive(Clone, Copy, PartialEq)]
pub enum RunState {
    /// Show the main menu
    MainMenu,
    /// Initialize the world
    NewGame,
    AwaitingInput,
    PlayerAction(Action),
    Running,

    /// Development affordances
    GenerateMap(ui::MapVisualizerState),
}

pub struct State {
    world: World,
    dispatcher: Dispatcher<'static, 'static>,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut bracket_lib::prelude::BTerm) {
        let run_state = *self.world.fetch::<RunState>();

        let next_run_state: RunState = match run_state {
            RunState::MainMenu => ui::main_menu(ctx),
            RunState::NewGame => {
                build_level(&mut self.world);

                self.dispatcher.dispatch(&mut self.world);
                self.world.maintain();

                RunState::AwaitingInput
            }
            RunState::AwaitingInput => ui::frame(ctx, &self.world),
            RunState::PlayerAction(action) => {
                log::debug!("Player action: {:?}", action);

                let player = *self.world.fetch::<Entity>();

                match take_action(&mut self.world, player, action) {
                    Ok(()) => RunState::Running,
                    Err(_) => RunState::AwaitingInput,
                }
            }
            RunState::Running => {
                self.dispatcher.dispatch(&mut self.world);
                self.world.maintain();

                RunState::AwaitingInput
            }

            RunState::GenerateMap(state) => ui::visualize_map(ctx, &mut self.world, state),
        };

        self.world.insert(next_run_state);
    }
}

impl State {
    pub fn new() -> Self {
        let mut world = World::new();

        let mut dispatcher = DispatcherBuilder::new()
            .with(MonsterAI, "monster_ai", &[])
            .with(VisibilitySystem, "visibility", &["monster_ai"])
            .with(IndexMapSystem, "index_map", &["visibility"])
            .build();

        dispatcher.setup(&mut world);
        world.register::<Appearance>();
        world.register::<Monster>();

        world.insert(RandomNumberGenerator::new());
        world.insert(RunState::MainMenu);

        Self { world, dispatcher }
    }
}
