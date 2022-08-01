use super::super::tile::Tile;
use crate::prelude::*;

pub fn empty_deck(width: i32, height: i32) -> Map {
    let mut map = Map::new(width, height);

    let map_center = Coordinate::from(Point::new(width / 2, height / 2));
    let radius = i32::min(width / 2, height / 2) - 1;

    for c in map_center.range(radius) {
        map[c] = Tile::floor();
    }

    map
}

pub fn add_engines(map: &mut Map) {
    let Point {
        x: width,
        y: height,
    } = map.dimensions();

    let map_center = Coordinate::from(Point::new(width / 2, height / 2));
    let half_radius = i32::min(width / 4, height / 4);

    for coord in [
        Coordinate::new(map_center.q - half_radius, map_center.r),
        Coordinate::new(map_center.q + half_radius, map_center.r - half_radius),
        Coordinate::new(map_center.q, map_center.r + half_radius),
    ] {
        for c in coord.range(8) {
            map[c] = Tile::wall();
        }
    }
}
