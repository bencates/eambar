use crate::prelude::*;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct HasInitiative;

#[derive(Component)]
pub struct Initiative {
    pub current: i32,
    pub speed: i32,
}

#[derive(SystemData)]
pub struct InitiativeData<'a> {
    // entities: Entities<'a>,
    initiatives: WriteStorage<'a, Initiative>,
    has_initiative: WriteStorage<'a, HasInitiative>,
}

impl<'a> InitiativeData<'a> {
    pub fn has_initiative(&self) -> &WriteStorage<'a, HasInitiative> {
        &self.has_initiative
    }

    pub fn spend_turn(&mut self, entity: Entity) {
        if self.has_initiative.remove(entity).is_some() {
            if let Some(initiative) = self.initiatives.get_mut(entity) {
                initiative.current = initiative.speed;
            }
        }
    }
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
                log::trace!("Giving initiative to {entity:?}");
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
