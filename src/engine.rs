use crate::{
    ai::MonsterAI,
    game_mechanics::{
        MaintainCharacterSheetSystem, MeleeCombatSystem, MovementSystem, VisibilitySystem,
    },
    level::build_level,
    map::IndexMapSystem,
    player_turn::try_action,
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
    PlayerAction(Action),
    Running,
    // /// Development affordances
    // GenerateMap(ui::MapVisualizerState),
}

pub struct GameEngine {
    world: World,
    dispatcher: Dispatcher<'static, 'static>,
}

impl GameState for GameEngine {
    fn tick(&mut self, ctx: &mut bracket_lib::prelude::BTerm) {
        let run_state = *self.world.fetch::<RunState>();

        let next_run_state: RunState = match run_state {
            MainMenu => ui::main_menu(ctx),
            NewGame => {
                build_level(&mut self.world);

                self.run()
            }
            AwaitingInput => ui::frame(ctx, &self.world),
            PlayerAction(action) => match try_action(&mut self.world, action) {
                Ok(()) => Running,
                Err(_) => AwaitingInput,
            },
            Running => self.run(),
            // GenerateMap(state) => ui::visualize_map(ctx, &mut self.world, state),
        };

        self.world.insert(next_run_state);
    }
}

impl GameEngine {
    pub fn new() -> Self {
        let mut world = World::new();

        let mut dispatcher = DispatcherBuilder::new()
            .with(MonsterAI, "monster_ai", &[])
            .with(MovementSystem, "movement", &["monster_ai"])
            .with(MeleeCombatSystem, "melee_combat", &["monster_ai"])
            .with(VisibilitySystem, "visibility", &["movement"])
            .with(
                MaintainCharacterSheetSystem,
                "maintain_character_sheet",
                &[],
            )
            .with(IndexMapSystem, "index_map", &["movement", "visibility"])
            .build();

        dispatcher.setup(&mut world);
        world.register::<Appearance>();

        world.insert(RandomNumberGenerator::new());
        world.insert(MainMenu);

        Self { world, dispatcher }
    }

    fn run(&mut self) -> RunState {
        self.dispatcher.dispatch(&self.world);
        self.world.maintain();

        AwaitingInput
    }
}
