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

    pub fn iter(&self) -> impl Iterator<Item = &Coordinate> + '_ {
        self.visible_tiles.iter()
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
            if vs.dirty {
                vs.dirty = false;

                vs.visible_tiles.clear();

                let origin = hex2d::Coordinate::new(coord.q, coord.r);

                for edge in origin.ring_iter(vs.range, hex2d::Spin::CW(hex2d::XY)) {
                    for (c1, c2) in origin.line_to_with_edge_detection_iter(edge) {
                        let c1 = Coordinate { q: c1.x, r: c1.y };
                        let c2 = Coordinate { q: c2.x, r: c2.y };

                        vs.visible_tiles.insert(c1);
                        vs.visible_tiles.insert(c2);

                        if map[c1.into()].is_opaque() && map[c2.into()].is_opaque() {
                            break;
                        }
                    }
                }
            }
        }
    }
}
