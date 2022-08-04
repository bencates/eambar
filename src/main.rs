use bracket_lib::prelude::main_loop;

mod ai;
mod engine;
mod entity;
mod game_mechanics;
mod level;
mod map;
mod player_turn;
mod ui;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use specs::{prelude::*, Component};

    pub use crate::{
        engine::RunState,
        entity::{Item, Monster, Player},
        game_mechanics::{CharacterSheet, Intents, Inventory, Viewshed},
        map::{BlocksTile, Coordinate, Direction, Map},
        player_turn::Action,
        ui::{Appearance, GameLog, Name},
    };
}

use prelude::*;

fn main() -> BError {
    pretty_env_logger::init();

    let bterm = ui::setup()?;
    let gamestate = engine::GameEngine::new();

    main_loop(bterm, gamestate)
}
