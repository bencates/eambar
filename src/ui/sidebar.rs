use super::SIDEBAR_WIDTH;
use crate::prelude::*;

const STATS_ORIGIN: Point = Point::constant(2, 2);
const WIDTH: i32 = SIDEBAR_WIDTH - 3;

pub(super) fn draw(ctx: &mut BTerm, world: &World) {
    let player = *world.fetch::<Entity>();
    let character_sheets = world.read_component::<CharacterSheet>();
    let player_character = character_sheets.get(player).unwrap();

    let (hp, max_hp) = player_character.hp();

    let health = format!("{} / {}", hp, max_hp);
    let health_x = STATS_ORIGIN.x + WIDTH - health.len() as i32;
    ctx.print_color(health_x, STATS_ORIGIN.y, YELLOW, BLACK, &health);

    ctx.draw_bar_horizontal(
        STATS_ORIGIN.x,
        STATS_ORIGIN.y + 2,
        WIDTH,
        hp,
        max_hp,
        RED,
        BLACK,
    );
}
