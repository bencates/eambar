mod game_log;
mod layout;
mod main_menu;
mod map;
mod sidebar;

use crate::prelude::*;
use std::fmt::{self, Display};

pub use game_log::{GameLog, RenderGameLogSystem};
pub use layout::RenderUILayoutSystem;
pub use main_menu::main_menu;
pub use map::RenderMapSystem;
pub use sidebar::{RenderInventorySystem, RenderPlayerStatsSystem};

pub const TERM_WIDTH: i32 = 80;
pub const TERM_HEIGHT: i32 = 60;

pub const MAP_WIDTH: i32 = 49;
pub const MAP_HEIGHT: i32 = 49;

const SIDEBAR_WIDTH: i32 = TERM_WIDTH - MAP_WIDTH - 2;

const FULL_PAINT: usize = (TERM_WIDTH * TERM_HEIGHT) as usize;

#[derive(Component)]
pub struct Name(String);

impl Name {
    pub fn new(name: impl ToString) -> Self {
        Self(name.to_string())
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Component)]
pub struct Appearance {
    pub glyph: char,
    pub color: ColorPair,
    pub z_order: i32,
}

pub fn setup() -> BResult<BTerm> {
    log::debug!("Generating a {TERM_WIDTH}x{TERM_HEIGHT} console");

    BTermBuilder::simple(TERM_WIDTH, TERM_HEIGHT)?
        .with_fancy_console(TERM_WIDTH, TERM_HEIGHT, "terminal8x8.png")
        .with_title("Roguelike Tutorial")
        .with_vsync(false)
        .build()
}
