use crate::prelude::*;

#[derive(Component)]
pub struct Target(pub Entity);

#[derive(SystemData)]
pub struct Targeting<'a> {
    targets: WriteStorage<'a, Target>,
}

impl<'a> Targeting<'a> {
    pub fn prev_target(&self, entity: Entity, potential_targets: &[Entity]) -> Option<Entity> {
        if let Some(&Target(curr)) = self.targets.get(entity) {
            potential_targets
                .iter()
                .position(|&e| curr == e)
                .and_then(|idx| idx.checked_sub(1))
                .and_then(|idx| potential_targets.get(idx))
        } else {
            potential_targets.last()
        }
        .copied()
    }

    pub fn next_target(&self, entity: Entity, potential_targets: &[Entity]) -> Option<Entity> {
        if let Some(&Target(curr)) = self.targets.get(entity) {
            potential_targets
                .iter()
                .position(|&e| curr == e)
                .and_then(|idx| potential_targets.get(idx + 1))
        } else {
            potential_targets.first()
        }
        .copied()
    }

    pub fn set_target(&mut self, entity: Entity, target_entity: Option<Entity>) {
        match (self.targets.get_mut(entity), target_entity) {
            (Some(current_target), Some(target_entity)) => {
                current_target.0 = target_entity;
            }
            (None, Some(target_entity)) => {
                self.targets.insert(entity, Target(target_entity)).unwrap();
            }
            (Some(_), None) => {
                self.targets.remove(entity);
            }
            (None, None) => {}
        };
    }
}

pub struct ClearTargetSystem;

impl<'a> System<'a> for ClearTargetSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Target>,
        ReadStorage<'a, Coordinate>,
        ReadStorage<'a, Viewshed>,
    );

    fn run(&mut self, (entities, mut targets, positions, viewsheds): Self::SystemData) {
        let invalid_targets: Vec<Entity> = (&entities, &targets, &viewsheds)
            .join()
            .filter(|(_, target, vs)| {
                positions
                    .get(target.0)
                    .map_or(true, |&coord| !vs.is_visible(coord))
            })
            .map(|(entity, _, _)| entity)
            .collect();

        for entity in invalid_targets {
            targets.remove(entity).unwrap();
        }
    }
}
