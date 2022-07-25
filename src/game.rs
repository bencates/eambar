use crate::map::{MapBuilder, SimpleMapBuilder};
use crate::prelude::*;
use crate::ui;

pub enum RunState {
    MainMenu,
    GenerateMap,
    // TODO: remove
    Idle,
}

pub struct State {
    world: World,
    _dispatcher: Dispatcher<'static, 'static>,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut bracket_lib::prelude::BTerm) {
        let next_run_state: RunState = match *self.world.fetch::<RunState>() {
            RunState::MainMenu => ui::main_menu(ctx),
            RunState::GenerateMap => {
                let map = SimpleMapBuilder::new(ui::TERM_WIDTH, ui::TERM_HEIGHT).build();

                ui::visualize_map(ctx, &map);

                RunState::Idle
            }
            // TODO: remove
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

        world.insert(RunState::MainMenu);

        Self {
            world,
            _dispatcher: dispatcher,
        }
    }
}
