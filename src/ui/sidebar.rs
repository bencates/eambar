use super::{FULL_PAINT, SIDEBAR_WIDTH};
use crate::prelude::*;

const STATS_ORIGIN: Point = Point::constant(2, 2);
const WIDTH: i32 = SIDEBAR_WIDTH - 3;

pub struct RenderPlayerStatsSystem;

impl<'a> System<'a> for RenderPlayerStatsSystem {
    type SystemData = (ReadExpect<'a, Entity>, ReadStorage<'a, CharacterSheet>);

    fn run(&mut self, (player, character_sheets): Self::SystemData) {
        let mut draw_batch = DrawBatch::new();

        // for (_, player_character) in (&players, &character_sheets).join()
        let player_character = character_sheets.get(*player).unwrap();

        let (hp, max_hp) = player_character.hp();

        let health = format!("{} / {}", hp, max_hp);
        let health_x = STATS_ORIGIN.x + WIDTH - health.len() as i32;
        draw_batch.print_color(
            (health_x, STATS_ORIGIN.y).into(),
            &health,
            ColorPair::new(YELLOW, BLACK),
        );

        draw_batch.bar_horizontal(
            (STATS_ORIGIN.x, STATS_ORIGIN.y + 2).into(),
            WIDTH,
            hp,
            max_hp,
            ColorPair::new(RED, BLACK),
        );

        draw_batch.submit(2 * FULL_PAINT).unwrap();
    }
}
