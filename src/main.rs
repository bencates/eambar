use bracket_lib::prelude::main_loop;

mod game;
mod ui;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use specs::prelude::*;

    pub use crate::game::RunState;
}

use prelude::*;

fn main() -> BError {
    let bterm = ui::setup()?;
    let gamestate = game::State::new();

    main_loop(bterm, gamestate)
}
