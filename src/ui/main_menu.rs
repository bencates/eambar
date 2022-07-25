use super::{TERM_HEIGHT, TERM_WIDTH};
use crate::prelude::*;

pub fn main_menu(ctx: &mut BTerm, selection: usize) -> RunState {
    use RunState::MainMenu;

    ctx.cls();
    ctx.draw_box(0, 0, TERM_WIDTH - 1, TERM_HEIGHT - 1, WHITE, BLACK);

    ctx.print_centered(TERM_HEIGHT / 2, "(Q)uit");

    ctx.key.map_or(MainMenu(selection), |key| match key {
        VirtualKeyCode::Q => {
            ctx.quit();
            MainMenu(selection)
        }
        _ => MainMenu(selection),
    })
}
