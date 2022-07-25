mod main_menu;
mod map_visualizer;

use crate::prelude::*;

pub use main_menu::main_menu;
pub use map_visualizer::visualize_map;

pub const TERM_WIDTH: i32 = 80;
pub const TERM_HEIGHT: i32 = 50;

#[derive(Component)]
pub struct Appearance {
    pub glyph: char,
    pub color: ColorPair,
}

pub fn setup() -> BResult<BTerm> {
    log::debug!("Generating a {TERM_WIDTH}x{TERM_HEIGHT} console");

    BTermBuilder::simple80x50()
        .with_fancy_console(80, 50, "terminal8x8.png")
        .with_title("Roguelike Tutorial")
        .with_vsync(false)
        .build()
}
