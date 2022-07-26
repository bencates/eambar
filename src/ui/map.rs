use crate::prelude::*;

pub(super) fn draw(ctx: &mut BTerm, world: &World) {
    let map = world.fetch::<Map>();
    let player = world.fetch::<Entity>();
    let coordinates = world.read_component::<Coordinate>();
    let appearances = world.read_component::<Appearance>();
    let player_pos = coordinates.get(*player).unwrap();
    let player_appearance = appearances.get(*player).unwrap();

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

                if map_pos == Point::from(*player_pos) {
                    ctx.set_fancy(
                        pos,
                        1,
                        rotation,
                        scale,
                        player_appearance.color.fg,
                        player_appearance.color.bg,
                        to_cp437(player_appearance.glyph),
                    );
                }
            }
        }
    }

    ctx.set_active_console(0);
}
