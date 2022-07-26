use crate::prelude::*;

pub fn visualize_map(ctx: &mut BTerm, map: &Map) {
    ctx.cls();
    ctx.set_active_console(1);
    ctx.cls();

    let map_dimensions = map.dimensions();

    let rotation = Radians::new(0.0);
    let scale = PointF::new(1.0, 1.0);

    for x in 0..map_dimensions.x {
        for y in 0..map_dimensions.y {
            let map_pos = Point::new(x, y);

            if let Ok(appearance) = Appearance::try_from(&map[map_pos]) {
                let mut pos = PointF::new(x as f32, y as f32);

                if x & 1 != 0 {
                    pos.y += 0.5;
                }

                log::trace!(
                    "Printing '{}' to {:?}, color {:?}",
                    appearance.glyph,
                    pos,
                    appearance.color
                );

                ctx.set_fancy(
                    pos,
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

    ctx.set_active_console(0);
}
