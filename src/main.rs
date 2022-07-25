use bracket_lib::prelude::main_loop;

mod action;
mod entity;
mod game;
mod map;
mod ui;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use specs::{prelude::*, Component};

    pub use crate::{
        action::Action,
        entity::Player,
        game::RunState,
        map::{Coordinates, Direction, Map},
        ui::Appearance,
    };
}

use prelude::*;

fn main() -> BError {
    pretty_env_logger::init();

    let bterm = ui::setup()?;
    let gamestate = game::State::new();

    main_loop(bterm, gamestate)
}
