mod game_log;
mod layout;
mod main_menu;
mod map;
mod map_visualizer;
mod sidebar;

use crate::prelude::*;
use std::fmt::{self, Display};

pub use game_log::GameLog;
pub use main_menu::main_menu;
pub use map_visualizer::*;

pub const TERM_WIDTH: i32 = 80;
pub const TERM_HEIGHT: i32 = 60;

pub const MAP_WIDTH: i32 = 50;
pub const MAP_HEIGHT: i32 = 48;

const SIDEBAR_WIDTH: i32 = TERM_WIDTH - MAP_WIDTH - 2;

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
}

pub fn setup() -> BResult<BTerm> {
    log::debug!("Generating a {TERM_WIDTH}x{TERM_HEIGHT} console");

    BTermBuilder::simple(TERM_WIDTH, TERM_HEIGHT)?
        .with_fancy_console(TERM_WIDTH, TERM_HEIGHT, "terminal8x8.png")
        .with_title("Roguelike Tutorial")
        .with_vsync(false)
        .build()
}

pub fn frame(ctx: &mut BTerm, world: &World) -> RunState {
    ctx.cls();

    layout::draw(ctx, world);
    map::draw(ctx, world);
    sidebar::draw(ctx, world);
    game_log::draw(ctx, world);

    handle_input(ctx)
}

fn handle_input(ctx: &BTerm) -> RunState {
    use {Action::*, Direction::*, RunState::*, VirtualKeyCode::*};

    ctx.key.map_or(AwaitingInput, |key| match key {
        // Movement keys
        Q => PlayerAction(Move(NorthWest)),
        W => PlayerAction(Move(North)),
        E => PlayerAction(Move(NorthEast)),
        A => PlayerAction(Move(SouthWest)),
        S => PlayerAction(Move(South)),
        D => PlayerAction(Move(SouthEast)),

        _ => AwaitingInput,
    })
}
