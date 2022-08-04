use crate::{
    ai::MonsterAI,
    game_mechanics::{
        ItemPickupSystem, ItemUseSystem, MaintainCharacterSheetSystem, MeleeCombatSystem,
        MovementSystem, PlayerInventorySystem, VisibilitySystem,
    },
    level::build_level,
    map::IndexMapSystem,
    player_turn,
    prelude::*,
    ui::{
        self, RenderGameLogSystem, RenderInventorySystem, RenderMapSystem, RenderPlayerStatsSystem,
        RenderUILayoutSystem,
    },
};
use RunState::*;

#[derive(Clone, Copy, PartialEq)]
pub enum RunState {
    /// Show the main menu
    MainMenu,
    /// Initialize the world
    NewGame,
    AwaitingInput,
    Running,
}

pub struct GameEngine {
    world: World,
    dispatcher: Dispatcher<'static, 'static>,
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
            Running => self.run(),
        };

        render_draw_buffer(ctx).unwrap();
    }
}

impl GameEngine {
    pub fn new() -> Self {
        let mut world = World::new();

        let mut dispatcher = DispatcherBuilder::new()
            .with(MonsterAI, "monster_ai", &[])
            .with(MovementSystem, "movement", &["monster_ai"])
            .with(ItemPickupSystem, "item_pickup", &[])
            .with(ItemUseSystem, "item_use", &[])
            .with(MeleeCombatSystem, "melee_combat", &["monster_ai"])
            .with(
                VisibilitySystem::new(&mut world),
                "visibility",
                &["movement"],
            )
            .with(
                MaintainCharacterSheetSystem,
                "maintain_character_sheet",
                &["melee_combat"],
            )
            .with(
                PlayerInventorySystem::new(&mut world),
                "player_inventory",
                &["item_pickup", "item_use"],
            )
            .with(
                IndexMapSystem,
                "index_map",
                &["movement", "visibility", "maintain_character_sheet"],
            )
            .with(RenderUILayoutSystem, "render_ui_layout", &[])
            .with(RenderMapSystem, "render_map", &["index_map"])
            .with(
                RenderPlayerStatsSystem,
                "render_stats",
                &["maintain_character_sheet"],
            )
            .with(
                RenderInventorySystem,
                "render_inventory",
                &["player_inventory"],
            )
            .with(
                RenderGameLogSystem,
                "render_game_log",
                &["item_pickup", "melee_combat", "maintain_character_sheet"],
            )
            .build();

        dispatcher.setup(&mut world);
        world.register::<Item>();

        world.insert(RandomNumberGenerator::new());

        Self {
            world,
            dispatcher,
            run_state: MainMenu,
        }
    }

    fn run(&mut self) -> RunState {
        self.dispatcher.dispatch(&self.world);
        self.world.maintain();

        AwaitingInput
    }
}
