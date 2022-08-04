use super::{FULL_PAINT, SIDEBAR_WIDTH};
use crate::prelude::*;

const STATS_ORIGIN: Point = Point::constant(2, 2);
const INVENTORY_ORIGIN: Point = Point::constant(2, 8);
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

pub struct RenderInventorySystem;

impl<'a> System<'a> for RenderInventorySystem {
    type SystemData = (
        Read<'a, Inventory>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, Appearance>,
    );

    fn run(&mut self, (player_inventory, names, appearances): Self::SystemData) {
        let mut text = TextBuilder::empty();

        text.append("Inventory:").ln().ln();

        let labels = (b'A'..=b'Z').map(|label| label as char);

        for (&item, label) in player_inventory.0.iter().zip(labels) {
            if let (Some(name), Some(Appearance { color, glyph, .. })) =
                (names.get(item), appearances.get(item))
            {
                text.fg(WHITE).append(&format!("{label}: ("));
                text.fg(color.fg).append(&glyph.to_string());
                text.fg(WHITE).append(") ");
                text.fg(color.fg).append(&name.to_string());
                text.ln();
            }
        }

        let mut draw_batch = DrawBatch::new();

        let mut text_block = TextBlock::new(INVENTORY_ORIGIN.x, INVENTORY_ORIGIN.y, WIDTH, 28); // FIXME: real height
        text_block.print(&text).unwrap();
        text_block.render_to_draw_batch(&mut draw_batch);

        draw_batch.submit(2 * FULL_PAINT).unwrap();
    }
}
