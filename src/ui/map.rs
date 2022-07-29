use super::{MAP_WIDTH, TERM_WIDTH};
use crate::prelude::*;

pub(super) fn draw(ctx: &mut BTerm, world: &World) {
    let map = world.fetch::<Map>();
    let map_origin = PointF::new((TERM_WIDTH - MAP_WIDTH - 1) as f32, 1.75);

    let coordinates = world.read_component::<Coordinate>();
    let appearances = world.read_component::<Appearance>();

    ctx.set_active_console(1);
    ctx.cls();

    let map_dimensions = map.dimensions();

    let rotation = Radians::new(0.0);
    let scale = PointF::new(1.0, 1.0);

    for x in 0..map_dimensions.x {
        for y in 0..map_dimensions.y {
            let map_coord: Coordinate = Point::new(x, y).into();

            if let Ok(appearance) = Appearance::try_from(&map[map_coord]) {
                log::trace!(
                    "Printing '{}' to {:?}, color {:?}",
                    appearance.glyph,
                    map_coord,
                    appearance.color
                );

                ctx.set_fancy(
                    map_origin + map_coord.into(),
                    0,
                    rotation,
                    scale,
                    appearance.color.fg,
                    appearance.color.bg,
                    to_cp437(appearance.glyph),
                );
            }
        }
    }

    for (&coord, appearance) in (&coordinates, &appearances).join() {
        if map[coord].is_visible() {
            ctx.set_fancy(
                map_origin + coord.into(),
                1,
                rotation,
                scale,
                appearance.color.fg,
                appearance.color.bg,
                to_cp437(appearance.glyph),
            );
        }
    }
    ctx.set_active_console(0);
}
