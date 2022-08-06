use super::{FULL_PAINT, SIDEBAR_WIDTH};
use crate::prelude::*;

const PLAYER_STATS_ORIGIN: Point = Point::constant(2, 2);
const TARGET_STATS_ORIGIN: Point = Point::constant(2, 8);
const INVENTORY_ORIGIN: Point = Point::constant(2, 14);
const WIDTH: i32 = SIDEBAR_WIDTH - 3;

pub struct RenderPlayerStatsSystem;

impl<'a> System<'a> for RenderPlayerStatsSystem {
    type SystemData = (
        ReadExpect<'a, Entity>,
        ReadStorage<'a, Durability>,
        ReadStorage<'a, Target>,
        ReadStorage<'a, Appearance>,
    );

    fn run(&mut self, (player, durabilities, targets, appearances): Self::SystemData) {
        let mut draw_batch = DrawBatch::new();

        let player_durability = durabilities.get(*player).unwrap();

        let (hp, max_hp) = player_durability.hp();

        let health = format!("{} / {}", hp, max_hp);
        let health_x = PLAYER_STATS_ORIGIN.x + WIDTH - health.len() as i32;
        draw_batch.print_color(
            (health_x, PLAYER_STATS_ORIGIN.y).into(),
            &health,
            ColorPair::new(YELLOW, BLACK),
        );

        draw_batch.bar_horizontal(
            (PLAYER_STATS_ORIGIN.x, PLAYER_STATS_ORIGIN.y + 2).into(),
            WIDTH,
            hp,
            max_hp,
            ColorPair::new(RED, BLACK),
        );

        if let Some(&Target(target)) = targets.get(*player) {
            if let Some(appearance) = appearances.get(target) {
                let mut text = TextBuilder::empty();
                full_name(&mut text, appearance);

                let mut text_block =
                    TextBlock::new(TARGET_STATS_ORIGIN.x, TARGET_STATS_ORIGIN.y, WIDTH, 1);
                text_block.print(&text).unwrap();
                text_block.render_to_draw_batch(&mut draw_batch);
            }

            if let Some(target_durability) = durabilities.get(target) {
                let (hp, max_hp) = target_durability.hp();

                draw_batch.bar_horizontal(
                    (TARGET_STATS_ORIGIN.x, TARGET_STATS_ORIGIN.y + 2).into(),
                    WIDTH,
                    hp,
                    max_hp,
                    ColorPair::new(RED, BLACK),
                );
            }
        } else {
            draw_batch.print_centered_at(
                TARGET_STATS_ORIGIN + Point::new(SIDEBAR_WIDTH / 2, 1),
                "No Target",
            );
        }

        draw_batch.submit(2 * FULL_PAINT).unwrap();
    }
}

pub struct RenderInventorySystem;

impl<'a> System<'a> for RenderInventorySystem {
    type SystemData = (Read<'a, Inventory>, ReadStorage<'a, Appearance>);

    fn run(&mut self, (player_inventory, appearances): Self::SystemData) {
        let mut text = TextBuilder::empty();

        text.append("Inventory:").ln().ln();

        let labels = (b'A'..=b'Z').map(|label| label as char);

        for (&item, label) in player_inventory.0.iter().zip(labels) {
            if let Some(appearance) = appearances.get(item) {
                text.fg(WHITE).append(&format!("{label}: "));
                full_name(&mut text, appearance);
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

fn full_name(
    text: &mut TextBuilder,
    Appearance {
        name, color, glyph, ..
    }: &Appearance,
) {
    text.fg(WHITE).append("(");
    text.fg(color.fg).append(&glyph.to_string());
    text.fg(WHITE).append(") ");
    text.fg(color.fg).append(name);
}
