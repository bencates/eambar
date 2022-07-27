use bracket_lib::prelude::main_loop;

mod action;
mod ai;
mod engine;
mod entity;
mod field_of_view;
mod level;
mod map;
mod ui;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use specs::{prelude::*, Component};

    pub use crate::{
        action::Action,
        engine::RunState,
        entity::{Monster, Player},
        field_of_view::Viewshed,
        map::{Coordinate, Direction, Map},
        ui::Appearance,
    };
}

use prelude::*;

fn main() -> BError {
    pretty_env_logger::init();

    let bterm = ui::setup()?;
    let gamestate = engine::GameEngine::new();

    main_loop(bterm, gamestate)
}
