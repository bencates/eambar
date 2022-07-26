use crate::prelude::*;
use std::collections::HashSet;

#[derive(Component)]
pub struct Viewshed {
    visible_tiles: HashSet<Coordinate>,
    range: i32,
    // dirty: bool,
}

impl Viewshed {
    pub fn new(range: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            range,
            // dirty: true,
        }
    }

    // pub fn touch(&mut self) {
    //     self.dirty = true;
    // }

    pub fn is_visible(&self, coord: Coordinate) -> bool {
        self.visible_tiles.contains(&coord)
    }
}

pub struct VisibilitySystem;

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        ReadExpect<'a, Map>,
        ReadStorage<'a, Coordinate>,
        WriteStorage<'a, Viewshed>,
    );

    fn run(&mut self, (map, coordinates, mut viewsheds): Self::SystemData) {
        for (&coord, vs) in (&coordinates, &mut viewsheds).join() {
            // if vs.dirty {
            //     vs.dirty = false;

            // TODO: use a hex-aware FOV algorithm. This gives weird results on our map.
            vs.visible_tiles = field_of_view_set(coord.into(), vs.range, &*map)
                .into_iter()
                .map(|point| point.into())
                .collect();
            // }
        }
    }
}
