use crate::prelude::*;

pub(super) fn draw(ctx: &mut BTerm, world: &World) {
    let map = world.fetch::<Map>();
    let player = world.fetch::<Entity>();

    let coordinates = world.read_component::<Coordinate>();
    let appearances = world.read_component::<Appearance>();
    let viewsheds = world.read_component::<Viewshed>();

    let player_viewshed = viewsheds.get(*player).unwrap();

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

                let mut fg = appearance.color.fg;
                if !player_viewshed.is_visible(map_coord) {
                    fg = fg.to_greyscale();
                }

                ctx.set_fancy(
                    map_coord.into(),
                    0,
                    rotation,
                    scale,
                    fg,
                    appearance.color.bg,
                    to_cp437(appearance.glyph),
                );
            }
        }
    }

    for (coord, appearance) in (&coordinates, &appearances).join() {
        if player_viewshed.is_visible(*coord) {
            ctx.set_fancy(
                PointF::from(*coord),
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
