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
        Option<Read<'a, TargetingReticule>>,
        Entities<'a>,
        ReadStorage<'a, Coordinate>,
        ReadStorage<'a, Appearance>,
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, Target>,
    );

    fn run(
        &mut self,
        (map, player, targeting_reticule, entities, coordinates, appearances, viewsheds, targets): Self::SystemData,
    ) {
        let player_viewshed = viewsheds.get(*player).unwrap();
        let player_target = targets.get(*player);

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

        if let Some(targeting_reticule) = targeting_reticule {
            for &coord in targeting_reticule.coordinates.iter() {
                draw_batch.set_fancy(
                    MAP_ORIGIN + coord.into(),
                    1,
                    NO_ROTATION,
                    BASE_SCALE,
                    ColorPair::new(BLUE, BLACK),
                    to_cp437('█'),
                );
            }

            for coord in
                map.area_of_effect(targeting_reticule.cursor, targeting_reticule.aoe_radius)
            {
                let color = if coord == targeting_reticule.cursor {
                    ColorPair::new(WHITE, BLACK)
                } else {
                    ColorPair::new(ORANGE, BLACK)
                };

                draw_batch.set_fancy(
                    MAP_ORIGIN + coord.into(),
                    2,
                    NO_ROTATION,
                    BASE_SCALE,
                    color,
                    to_cp437('█'),
                );
            }
        }

        for (entity, &coord, appearance) in (&entities, &coordinates, &appearances).join() {
            let mut color = appearance.color;

            if player_target.map_or(false, |&Target(target)| entity == target) {
                color.bg = RGBA::named(WHITE);
            }

            if player_viewshed.is_visible(coord) {
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

        draw_batch.submit(FULL_PAINT).unwrap();
    }
}
