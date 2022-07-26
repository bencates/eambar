use crate::prelude::*;
use crate::ui;
use crate::{
    action::take_action,
    map::{MapBuilder, SimpleMapBuilder},
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
    GenerateMap,
    Idle,
}

pub struct State {
    world: World,
    _dispatcher: Dispatcher<'static, 'static>,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut bracket_lib::prelude::BTerm) {
        let run_state = *self.world.fetch::<RunState>();

        let next_run_state: RunState = match run_state {
            RunState::MainMenu => ui::main_menu(ctx),
            RunState::NewGame => {
                let map = SimpleMapBuilder::new(ui::TERM_WIDTH, ui::TERM_HEIGHT).build();

                if let Some(room) = map.rooms.first() {
                    crate::entity::spawn_player(&mut self.world, room.center())
                }

                self.world.insert(map);

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
            RunState::Running => RunState::AwaitingInput,

            RunState::GenerateMap => {
                let map = SimpleMapBuilder::new(ui::TERM_WIDTH, ui::TERM_HEIGHT).build();

                ui::visualize_map(ctx, &map);

                RunState::Idle
            }
            RunState::Idle => {
                if ctx.key == Some(VirtualKeyCode::Q) {
                    ctx.quit();
                }

                RunState::Idle
            }
        };

        self.world.insert(next_run_state);
    }
}

impl State {
    pub fn new() -> Self {
        let mut world = World::new();

        let mut dispatcher = DispatcherBuilder::new().build();

        dispatcher.setup(&mut world);
        world.register::<Player>();
        world.register::<Coordinate>();
        world.register::<Appearance>();

        world.insert(RunState::MainMenu);

        Self {
            world,
            _dispatcher: dispatcher,
        }
    }
}
