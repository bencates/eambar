use crate::prelude::*;

#[derive(Component)]
pub struct WantsToMove(pub(super) Coordinate);

pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        WriteStorage<'a, WantsToMove>,
        WriteStorage<'a, Coordinate>,
        WriteStorage<'a, Viewshed>,
    );

    fn run(&mut self, (mut move_intents, mut coordinates, mut viewsheds): Self::SystemData) {
        for (&WantsToMove(dest), coord, vs) in
            (&move_intents, &mut coordinates, &mut viewsheds).join()
        {
            *coord = dest;

            vs.touch();
        }

        move_intents.clear();
    }
}
