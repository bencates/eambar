mod main_menu;
mod map;
mod map_visualizer;

use crate::prelude::*;

pub use main_menu::main_menu;
pub use map_visualizer::*;

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

pub fn frame(ctx: &mut BTerm, world: &World) -> RunState {
    ctx.cls();

    map::draw(ctx, world);

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
