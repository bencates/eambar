use super::{MAP_HEIGHT, SIDEBAR_WIDTH, TERM_HEIGHT, TERM_WIDTH};
use crate::prelude::*;

pub struct RenderUILayoutSystem;

impl<'a> System<'a> for RenderUILayoutSystem {
    type SystemData = ();

    fn run(&mut self, _data: Self::SystemData) {
        let mut draw_batch = DrawBatch::new();

        draw_batch.cls();

        let outer_frame = Rect::with_size(0, 0, TERM_WIDTH - 1, TERM_HEIGHT - 1);
        let sidebar_frame = Rect::with_size(0, 0, SIDEBAR_WIDTH, TERM_HEIGHT - 1);
        let log_frame = Rect::with_size(
            SIDEBAR_WIDTH,
            MAP_HEIGHT + 1,
            TERM_WIDTH - SIDEBAR_WIDTH - 1,
            TERM_HEIGHT - MAP_HEIGHT - 2,
        );
        let stats_frame = Rect::with_size(0, 0, SIDEBAR_WIDTH, 6);

        let color = ColorPair::new(WHITE, BLACK);

        draw_batch.draw_box(outer_frame, color);
        draw_batch.draw_box(sidebar_frame, color);
        draw_batch.draw_box(log_frame, color);
        draw_batch.draw_box(stats_frame, color);

        // Clean up the intersections
        draw_batch.print((SIDEBAR_WIDTH, 0).into(), "┬");
        draw_batch.print((SIDEBAR_WIDTH, MAP_HEIGHT + 1).into(), "├");
        draw_batch.print((TERM_WIDTH - 1, MAP_HEIGHT + 1).into(), "┤");
        draw_batch.print((SIDEBAR_WIDTH, TERM_HEIGHT - 1).into(), "┴");
        draw_batch.print((0, 6).into(), "├");
        draw_batch.print((SIDEBAR_WIDTH, 6).into(), "┤");

        draw_batch.submit(0).unwrap();
    }
}
