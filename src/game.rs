use crate::prelude::*;
use crate::ui;

pub enum RunState {
    MainMenu(usize),
}

pub struct State {
    world: World,
    dispatcher: Dispatcher<'static, 'static>,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut bracket_lib::prelude::BTerm) {
        let next_run_state: RunState = match *self.world.fetch::<RunState>() {
            RunState::MainMenu(selection) => ui::main_menu(ctx, selection),
        };

        self.world.insert(next_run_state);
    }
}

impl State {
    pub fn new() -> Self {
        let mut world = World::new();

        let mut dispatcher = DispatcherBuilder::new().build();

        dispatcher.setup(&mut world);

        world.insert(RunState::MainMenu(0));

        Self { world, dispatcher }
    }
}
