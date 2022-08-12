use crate::prelude::*;
use std::collections::HashSet;

#[derive(Component)]
pub struct Viewshed {
    visible_tiles: HashSet<Coordinate>,
    range: i32,
}

impl Viewshed {
    pub fn new(range: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            range,
        }
    }

    pub fn is_visible(&self, coord: Coordinate) -> bool {
        self.visible_tiles.contains(&coord)
    }
}

pub struct VisibilitySystem {
    cursor: ReaderId<ComponentEvent>,
}

impl VisibilitySystem {
    pub fn new(world: &mut World) -> Self {
        world.register::<Coordinate>();
        let mut coordinates = world.write_component::<Coordinate>();

        Self {
            cursor: coordinates.register_reader(),
        }
    }
}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, Map>,
        Entities<'a>,
        ReadStorage<'a, Coordinate>,
        WriteStorage<'a, Viewshed>,
    );

    fn run(&mut self, (player, mut map, entities, coordinates, mut viewsheds): Self::SystemData) {
        use ComponentEvent::*;

        let changed: BitSet = coordinates
            .channel()
            .read(&mut self.cursor)
            .filter_map(|&event| match event {
                Inserted(id) | Modified(id) => Some(id),
                Removed(_) => None,
            })
            .collect();

        for (entity, &coord, vs, _) in (&entities, &coordinates, &mut viewsheds, &changed).join() {
            log::trace!("Updating FOV for {entity:?}");

            vs.visible_tiles = map.field_of_view(coord, vs.range);

            if entity == *player {
                for &coord in vs.visible_tiles.iter() {
                    map[coord].reveal();
                }
            }
        }
    }
}
