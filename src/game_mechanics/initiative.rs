use crate::prelude::*;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct HasInitiative;

#[derive(Component)]
pub struct Initiative {
    pub current: i32,
    pub speed: i32,
}

pub struct InitiativeSystem;

impl<'a> System<'a> for InitiativeSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Initiative>,
        WriteStorage<'a, HasInitiative>,
    );

    fn run(&mut self, (entities, mut initiatives, mut has_initiative): Self::SystemData) {
        if !has_initiative.is_empty() {
            return;
        }

        match (&entities, &initiatives)
            .join()
            .find(|(_, initiative)| initiative.current <= 0)
        {
            Some((entity, _)) => {
                log::trace!("Giving initiative to Entity #{}", entity.id());
                has_initiative.insert(entity, HasInitiative).unwrap();
            }
            None => {
                log::trace!("Nobody has initiative this round");
                for initiative in (&mut initiatives).join() {
                    initiative.current -= 1;
                }
            }
        }
    }
}
