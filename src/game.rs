use crate::map::{MapBuilder, SimpleMapBuilder};
use crate::prelude::*;
use crate::ui;

#[derive(Clone, Copy, PartialEq)]
pub enum RunState {
    MainMenu,
    NewGame,
    AwaitingInput,

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

                ui::frame(ctx, &self.world)
            }
            RunState::AwaitingInput => RunState::AwaitingInput,
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
        world.register::<Coordinates>();
        world.register::<Appearance>();

        world.insert(RunState::MainMenu);

        Self {
            world,
            _dispatcher: dispatcher,
        }
    }
}
