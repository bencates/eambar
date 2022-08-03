use crate::{
    ai::MonsterAI,
    game_mechanics::{
        ItemPickupSystem, MaintainCharacterSheetSystem, MeleeCombatSystem, MovementSystem,
        VisibilitySystem,
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
}

pub struct GameEngine {
    world: World,
    dispatcher: Dispatcher<'static, 'static>,
}

impl GameState for GameEngine {
    fn tick(&mut self, ctx: &mut BTerm) {
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
            .with(ItemPickupSystem, "item_pickup", &[])
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
        world.register::<Item>();

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
