use super::{FULL_PAINT, MAP_WIDTH, TERM_WIDTH};
use crate::prelude::*;

const MAP_CONSOLE: usize = 1;
const MAP_ORIGIN: PointF = PointF::new((TERM_WIDTH - MAP_WIDTH - 1) as f32, 1.75);

const NO_ROTATION: Radians = Radians(0.0);
const BASE_SCALE: PointF = PointF::new(1.0, 1.0);

pub struct RenderMapSystem;

impl<'a> System<'a> for RenderMapSystem {
    type SystemData = (
        ReadExpect<'a, Map>,
        ReadExpect<'a, Entity>,
        ReadStorage<'a, Coordinate>,
        ReadStorage<'a, Appearance>,
        ReadStorage<'a, Viewshed>,
    );

    fn run(&mut self, (map, player, coordinates, appearances, viewsheds): Self::SystemData) {
        let player_viewshed = viewsheds.get(*player).unwrap();

        let mut draw_batch = DrawBatch::new();

        draw_batch.target(MAP_CONSOLE);
        draw_batch.cls();

        for coord in map.iter() {
            if let Ok(appearance) = Appearance::try_from(&map[coord]) {
                let mut color = appearance.color;

                if !player_viewshed.is_visible(coord) {
                    color.fg = color.fg.to_greyscale();
                }

                draw_batch.set_fancy(
                    MAP_ORIGIN + coord.into(),
                    appearance.z_order,
                    NO_ROTATION,
                    BASE_SCALE,
                    color,
                    to_cp437(appearance.glyph),
                );
            }
        }

        for (&coord, appearance) in (&coordinates, &appearances).join() {
            if player_viewshed.is_visible(coord) {
                draw_batch.set_fancy(
                    MAP_ORIGIN + coord.into(),
                    appearance.z_order,
                    NO_ROTATION,
                    BASE_SCALE,
                    appearance.color,
                    to_cp437(appearance.glyph),
                );
            }
        }

        draw_batch.submit(FULL_PAINT).unwrap();
    }
}
