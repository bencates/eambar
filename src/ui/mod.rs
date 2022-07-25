mod main_menu;
mod map_visualizer;

use crate::prelude::*;

pub use main_menu::main_menu;
pub use map_visualizer::visualize_map;

pub const TERM_WIDTH: i32 = 92;
pub const TERM_HEIGHT: i32 = 60;

#[derive(Component)]
pub struct Appearance {
    pub glyph: char,
    pub color: ColorPair,
}

pub fn setup() -> BResult<BTerm> {
    log::debug!("Generating a {TERM_WIDTH}x{TERM_HEIGHT} console");

    BTermBuilder::simple(TERM_WIDTH, TERM_HEIGHT)?
        .with_title("Roguelike Tutorial")
        .build()
}
