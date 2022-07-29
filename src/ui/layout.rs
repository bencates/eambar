use super::{MAP_HEIGHT, MAP_WIDTH, TERM_HEIGHT, TERM_WIDTH};
use crate::prelude::*;

const SIDEBAR_WIDTH: i32 = TERM_WIDTH - MAP_WIDTH - 2;

pub(super) fn draw(ctx: &mut BTerm, _world: &World) {
    ctx.draw_box(0, 0, TERM_WIDTH - 1, TERM_HEIGHT - 1, WHITE, BLACK);
    ctx.draw_box(0, 0, SIDEBAR_WIDTH, TERM_HEIGHT - 1, WHITE, BLACK);
    ctx.draw_box(
        SIDEBAR_WIDTH,
        MAP_HEIGHT + 1,
        TERM_WIDTH - SIDEBAR_WIDTH - 1,
        TERM_HEIGHT - MAP_HEIGHT - 2,
        WHITE,
        BLACK,
    );
    ctx.draw_box(0, 0, SIDEBAR_WIDTH, 6, WHITE, BLACK);
    // ctx.print(0, 6, "├──────────────────┤");

    // Clean up the intersections
    ctx.print(SIDEBAR_WIDTH, 0, "┬");
    ctx.print(SIDEBAR_WIDTH, MAP_HEIGHT + 1, "├");
    ctx.print(TERM_WIDTH - 1, MAP_HEIGHT + 1, "┤");
    ctx.print(SIDEBAR_WIDTH, TERM_HEIGHT - 1, "┴");
    ctx.print(0, 6, "├");
    ctx.print(SIDEBAR_WIDTH, 6, "┤");
}
