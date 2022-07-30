use super::tile::Tile;
use crate::prelude::*;
use hex2d::Coordinate as Hex2dCoordinate;

pub fn build_level(width: i32, height: i32) -> Map {
    engine_deck(width, height)
}

fn empty_deck(width: i32, height: i32) -> Map {
    let radius = i32::min(width / 2, height / 2);

    let mut map = Map::new(width, height);

    let map_center = Coordinate::from(Point::new(radius, radius));

    for x in 0..width {
        for y in 0..height {
            let pos = Point::new(x, y);
            if Coordinate::from(pos).distance(map_center) < radius {
                map.tiles[pos.to_index(49)] = Tile::floor();
            }
        }
    }

    map.spawn_points.push(map_center);

    map
}

fn engine_deck(width: i32, height: i32) -> Map {
    let mut map = empty_deck(width, height);
    let map_center = map.spawn_points[0];

    let half_radius = i32::min(width / 4, height / 4);

    for coord in [
        Coordinate::new(map_center.q - half_radius, map_center.r),
        Coordinate::new(map_center.q + half_radius, map_center.r - half_radius),
        Coordinate::new(map_center.q, map_center.r + half_radius),
    ] {
        // map.tiles[coord.to_index(width)] = Tile::wall();

        for h2coord in Hex2dCoordinate::from(coord).range_iter(8) {
            let idx = Coordinate::from(h2coord).to_index(width);
            map.tiles[idx] = Tile::wall();
        }
    }

    map
}
