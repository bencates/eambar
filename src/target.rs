use crate::prelude::*;

#[derive(Component)]
pub struct Target(pub Entity);

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
            log::debug!("Clearing target for {entity:?}");
            targets.remove(entity).unwrap();
        }
    }
}
