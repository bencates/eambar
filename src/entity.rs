use crate::prelude::*;

/// Marker trait for the player entity.
#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Player;

pub fn spawn_player(world: &mut World, pos: Point) {
    let player = world
        .create_entity()
        .with(Player)
        // .with(Name::new(Player))
        .with(Appearance {
            color: ColorPair::new(YELLOW, BLACK),
            glyph: '@',
        })
        .with(Coordinates::from(pos))
        // .with(Viewshed::new(8))
        // .with(CombatStats::new(30, 5, 2))
        .build();

    world.insert(player);
    // world.insert(pos);
    // world.insert(Inventory::default());
}
