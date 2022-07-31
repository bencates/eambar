use super::tile::Tile;
use crate::prelude::*;
use Direction::*;

pub fn build_level(width: i32, height: i32, rng: &mut RandomNumberGenerator) -> Map {
    // let mut map = empty_deck_template(width, height);
    let mut map = engine_deck_template(width, height);
    map = with_walls(map, rng);

    map
}

fn empty_deck_template(width: i32, height: i32) -> Map {
    let mut map = Map::new(width, height);

    let map_center = Coordinate::from(Point::new(width / 2, height / 2));
    let radius = i32::min(width / 2, height / 2) - 1;

    for c in map_center.range(radius) {
        map[c] = Tile::floor();
    }

    map.spawn_points.push(map_center);

    map
}

fn engine_deck_template(width: i32, height: i32) -> Map {
    let mut map = empty_deck_template(width, height);
    let map_center = map.spawn_points[0];

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

    map
}

const MIN_ROOM_SIZE: i32 = 4;

const Q_AXIS: (Direction, Direction) = (North, South);
const R_AXIS: (Direction, Direction) = (NorthWest, SouthEast);
const S_AXIS: (Direction, Direction) = (NorthEast, SouthWest);

fn with_walls(mut map: Map, rng: &mut RandomNumberGenerator) -> Map {
    let mut doors: Vec<Coordinate> = Vec::new();
    let mut done = (false, false, false);

    while !(done.0 && done.1 && done.2) {
        let paths = q_paths(&map);
        if let Some((c1, c2)) = paths.into_iter().max_by_key(|(c1, c2)| c1.distance(*c2)) {
            match bisect_path(&mut map, c1, c2, &[R_AXIS, S_AXIS], rng) {
                Some(c) => doors.push(c),
                None => done.0 = true,
            };
        }

        let paths = r_paths(&map);
        if let Some((c1, c2)) = paths.into_iter().max_by_key(|(c1, c2)| c1.distance(*c2)) {
            match bisect_path(&mut map, c1, c2, &[Q_AXIS, S_AXIS], rng) {
                Some(c) => doors.push(c),
                None => done.1 = true,
            };
        }

        let paths = s_paths(&map);
        if let Some((c1, c2)) = paths.into_iter().max_by_key(|(c1, c2)| c1.distance(*c2)) {
            match bisect_path(&mut map, c1, c2, &[Q_AXIS, R_AXIS], rng) {
                Some(c) => doors.push(c),
                None => done.2 = true,
            };
        }
    }

    for c in doors {
        map[c] = Tile::floor();
    }

    map
}

fn q_paths(map: &Map) -> Vec<(Coordinate, Coordinate)> {
    let Point { x: width, .. } = map.dimensions();

    let origins = (0..width).map(|x| Coordinate::from(Point::new(x, 0)));

    paths(map, origins, South)
}

fn r_paths(map: &Map) -> Vec<(Coordinate, Coordinate)> {
    let Point {
        x: width,
        y: height,
    } = map.dimensions();

    let origins = (0..width)
        .rev()
        .map(|x| Coordinate::from(Point::new(x, 0)))
        .chain((1..height).map(|y| Coordinate::from(Point::new(0, y))));

    paths(map, origins, SouthEast)
}

fn s_paths(map: &Map) -> Vec<(Coordinate, Coordinate)> {
    let Point {
        x: width,
        y: height,
    } = map.dimensions();

    let origins = (0..width)
        .map(|x| Coordinate::from(Point::new(x, 0)))
        .chain((1..height).map(|y| Coordinate::from(Point::new(width - 1, y))));

    paths(map, origins, SouthWest)
}

fn paths(
    map: &Map,
    origins: impl Iterator<Item = Coordinate>,
    dir: Direction,
) -> Vec<(Coordinate, Coordinate)> {
    let mut paths: Vec<(Coordinate, Coordinate)> = Vec::new();

    for mut coord in origins {
        let mut path_start: Option<Coordinate> = None;

        while map.in_bounds(coord + dir) {
            if path_start.is_none() && !map[coord].is_blocked() {
                path_start = Some(coord)
            }

            if map[coord + dir].is_blocked() {
                if let Some(start) = path_start.take() {
                    paths.push((start, coord));
                }
            }

            coord = coord + dir;
        }
    }

    paths
}

fn bisect_path(
    map: &mut Map,
    c1: Coordinate,
    c2: Coordinate,
    axes: &[(Direction, Direction)],
    rng: &mut RandomNumberGenerator,
) -> Option<Coordinate> {
    let distance = c1.distance(c2);

    if distance > MIN_ROOM_SIZE * 2 {
        let offset = rng.range(MIN_ROOM_SIZE, distance - MIN_ROOM_SIZE);

        if let Some(c) = c1.line_to(c2).skip(offset as usize).next() {
            if let Some((dir1, dir2)) = rng.random_slice_entry(axes) {
                draw_wall(map, c, *dir1, *dir2);

                return Some(c);
            }
        }
    }

    None
}

fn draw_wall(map: &mut Map, mut coord: Coordinate, dir1: Direction, dir2: Direction) {
    while !map[coord + dir1].is_blocked() {
        coord = coord + dir1;
    }

    while !map[coord].is_blocked() {
        map[coord] = Tile::wall();
        coord = coord + dir2;
    }
}
