use bracket_lib::prelude::main_loop;

mod ai;
mod engine;
mod entity;
mod game_mechanics;
mod level;
mod map;
mod player_turn;
mod target;
mod targeting;
mod ui;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use specs::{prelude::*, Component};

    pub use crate::{
        engine::RunState,
        entity::{Item, Monster, Player},
        game_mechanics::{
            DealsDamage, Durability, Initiative, Intents, Inventory, ProvidesHealing, Usable,
            Viewshed,
        },
        map::{BlocksTile, Coordinate, Direction, Map},
        target::{Target, TargetingData},
        targeting::TargetingReticule,
        ui::{Appearance, GameLog},
    };
}

use prelude::*;

fn main() -> BError {
    pretty_env_logger::init();

    let bterm = ui::setup()?;
    let gamestate = engine::GameEngine::new();

    main_loop(bterm, gamestate)
}
