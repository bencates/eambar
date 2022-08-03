use super::{FULL_PAINT, MAP_WIDTH, TERM_WIDTH};
use crate::prelude::*;

pub struct RenderMapSystem;

impl<'a> System<'a> for RenderMapSystem {
    type SystemData = (
        ReadExpect<'a, Map>,
        ReadStorage<'a, Coordinate>,
        ReadStorage<'a, Appearance>,
    );

    fn run(&mut self, (map, coordinates, appearances): Self::SystemData) {
        let map_origin = PointF::new((TERM_WIDTH - MAP_WIDTH - 1) as f32, 1.75);

        let mut draw_batch = DrawBatch::new();

        draw_batch.target(1);
        draw_batch.cls_color(RGBA::new());

        let rotation = Radians::new(0.0);
        let scale = PointF::new(1.0, 1.0);

        for map_coord in map.iter() {
            if let Ok(appearance) = Appearance::try_from(&map[map_coord]) {
                draw_batch.set_fancy(
                    map_origin + map_coord.into(),
                    appearance.z_order,
                    rotation,
                    scale,
                    appearance.color,
                    to_cp437(appearance.glyph),
                );
            }
        }

        for (&coord, appearance) in (&coordinates, &appearances).join() {
            if map[coord].is_visible() {
                draw_batch.set_fancy(
                    map_origin + coord.into(),
                    appearance.z_order,
                    rotation,
                    scale,
                    appearance.color,
                    to_cp437(appearance.glyph),
                );
            }
        }

        draw_batch.submit(FULL_PAINT).unwrap();
    }
}
