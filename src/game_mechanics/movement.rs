use crate::prelude::*;

#[derive(Component)]
pub struct WantsToMove(pub(super) Coordinate);

pub fn is_legal_move(map: &Map, coord: Coordinate) -> bool {
    map.in_bounds(coord) && !map[coord].is_blocked()
}

pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        ReadExpect<'a, Map>,
        WriteStorage<'a, WantsToMove>,
        WriteStorage<'a, Coordinate>,
        WriteStorage<'a, Viewshed>,
    );

    fn run(&mut self, (map, mut move_intents, mut coordinates, mut viewsheds): Self::SystemData) {
        for (&WantsToMove(dest), coord, vs) in
            (&move_intents, &mut coordinates, &mut viewsheds).join()
        {
            if is_legal_move(&map, dest) {
                *coord = dest;

                vs.touch();
            }
        }

        move_intents.clear();
    }
}
