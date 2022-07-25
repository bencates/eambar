use crate::prelude::*;

pub fn visualize_map(ctx: &mut BTerm, map: &Map) {
    let map_dimensions = map.dimensions();

    for x in 0..map_dimensions.x {
        for y in 0..map_dimensions.y {
            let pos = Point::new(x, y);

            if let Ok(appearance) = Appearance::try_from(&map[pos]) {
                ctx.set(
                    x,
                    y,
                    appearance.color.fg,
                    appearance.color.bg,
                    to_cp437(appearance.glyph),
                );
            }
        }
    }
}
