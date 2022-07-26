use super::super::tile::Tile;
use super::MapBuilder;
use crate::prelude::*;

use hex2d::Coordinate as HexCoord;

pub struct SimpleMapBuilder {
    map: Map,
    // rooms: Vec<Rect>,
    // corridors: Vec<VectorLine>,
}

impl SimpleMapBuilder {
    pub fn new(width: i32, height: i32) -> Self {
        let mut map = Map::new(width, height);
        let (rooms, corridors) = build_rooms_and_corridors(width, height);

        for pos in rooms.iter().flat_map(|room| room.point_set()) {
            map.tiles[pos.to_index(width)] = Tile::floor();
        }

        for pos in corridors.into_iter().flatten() {
            map.tiles[pos.to_index(width)] = Tile::floor();
        }

        map.rooms = rooms;

        Self { map }
    }
}

impl MapBuilder for SimpleMapBuilder {
    fn build(self) -> Map {
        self.map
    }
}

const MAX_ROOMS: i32 = 30;
const MIN_ROOM_SIZE: i32 = 6;
const MAX_ROOM_SIZE: i32 = 10;

fn build_rooms_and_corridors(map_width: i32, map_height: i32) -> (Vec<Rect>, Vec<Vec<Point>>) {
    let mut rooms = Vec::new();
    let mut corridors = Vec::new();

    let mut rng = RandomNumberGenerator::new();

    for _ in 0..MAX_ROOMS {
        let w = rng.range(MIN_ROOM_SIZE, MAX_ROOM_SIZE + 1);
        let h = rng.range(MIN_ROOM_SIZE, MAX_ROOM_SIZE + 1);

        // We want to ensure there will be at least one wall tile between
        // each room and the edge of the map.
        let x = rng.range(1, map_width - w - 1);
        let y = rng.range(1, map_height - h - 1);

        let new_room = Rect::with_size(x, y, w, h);
        let new_room_with_walls = Rect::with_size(x - 1, y - 1, w + 2, h + 2);

        if rooms
            .iter()
            .any(|room: &Rect| room.intersect(&new_room_with_walls))
        {
            continue;
        }

        if let Some(prev_room) = rooms.last() {
            let Coordinate { q: q1, r: r1 } = prev_room.center().into();
            let Coordinate { q: q2, r: r2 } = new_room.center().into();

            let c1 = HexCoord::new(q1, r1);
            let c2 = HexCoord::new(q2, r2);

            corridors.push(
                c1.line_to_iter(c2)
                    .map(|c| Coordinate { q: c.x, r: c.y }.into())
                    .collect(),
            );
        }

        rooms.push(new_room);
    }

    (rooms, corridors)
}
