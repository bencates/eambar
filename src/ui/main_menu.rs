use super::{TERM_HEIGHT, TERM_WIDTH};
use crate::prelude::*;

/// Render the main menu
///
/// Lists the available options
pub fn main_menu(ctx: &mut BTerm) -> RunState {
    ctx.cls();
    ctx.draw_box(0, 0, TERM_WIDTH - 1, TERM_HEIGHT - 1, WHITE, BLACK);

    ctx.print_centered(TERM_HEIGHT / 2 - 1, "(N)ew Game");
    ctx.print_centered(TERM_HEIGHT / 2, "Generate (M)ap");
    ctx.print_centered(TERM_HEIGHT / 2 + 1, "(Q)uit");

    ctx.key.map_or(RunState::MainMenu, |key| match key {
        VirtualKeyCode::N => {
            log::info!("Starting a new game");

            RunState::NewGame
        }
        VirtualKeyCode::M => {
            log::info!("Generating a map");

            RunState::GenerateMap
        }
        VirtualKeyCode::Q => {
            log::info!("Quitting");
            ctx.quit();
            RunState::MainMenu
        }

        _ => RunState::MainMenu,
    })
}
