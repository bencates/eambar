mod main_menu;

use crate::prelude::*;

pub use main_menu::main_menu;

pub const TERM_WIDTH: i32 = 92;
pub const TERM_HEIGHT: i32 = 60;

pub fn setup() -> BResult<BTerm> {
    log::debug!("Generating a {TERM_WIDTH}x{TERM_HEIGHT} console");

    BTermBuilder::simple(TERM_WIDTH, TERM_HEIGHT)?
        .with_title("Roguelike Tutorial")
        .build()
}
