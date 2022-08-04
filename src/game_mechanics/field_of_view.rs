use crate::prelude::*;
use std::collections::HashSet;

#[derive(Component)]
pub struct Viewshed {
    visible_tiles: HashSet<Coordinate>,
    range: i32,
    dirty: bool,
}

impl Viewshed {
    pub fn new(range: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            range,
            dirty: true,
        }
    }

    pub fn touch(&mut self) {
        self.dirty = true;
    }

    pub fn is_visible(&self, coord: Coordinate) -> bool {
        self.visible_tiles.contains(&coord)
    }
}

pub struct VisibilitySystem;

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        ReadExpect<'a, Entity>,
        WriteExpect<'a, Map>,
        Entities<'a>,
        ReadStorage<'a, Coordinate>,
        WriteStorage<'a, Viewshed>,
    );

    fn run(&mut self, (player, mut map, entities, coordinates, mut viewsheds): Self::SystemData) {
        for (entity, &coord, vs) in (&entities, &coordinates, &mut viewsheds).join() {
            if vs.dirty {
                vs.dirty = false;
                vs.visible_tiles.clear();

                for edge in coord.ring(vs.range) {
                    for (c1, c2) in coord.fat_line_to(edge) {
                        vs.visible_tiles.insert(c1);
                        vs.visible_tiles.insert(c2);

                        if map[c1].is_opaque() && map[c2].is_opaque() {
                            break;
                        }
                    }
                }

                if entity == *player {
                    for &coord in vs.visible_tiles.iter() {
                        map[coord].reveal();
                    }
                }
            }
        }
    }
}
