use super::BeingUsed;
use crate::prelude::*;
use anyhow::{bail, ensure, Context, Result};

#[derive(SystemData)]
pub struct EffectUsage<'a> {
    map: ReadExpect<'a, Map>,
    usables: ReadStorage<'a, Usable>,
    positions: ReadStorage<'a, Coordinate>,
    being_used: WriteStorage<'a, BeingUsed>,
}

impl<'a> EffectUsage<'a> {
    pub fn use_on_self(&mut self, effect: Entity, user: Entity) -> Result<()> {
        match self.usables.get(effect) {
            Some(&Usable::OnSelf) => {}
            _ => bail!("not usable on self"),
        };

        self.being_used.insert(effect, BeingUsed(smallvec![user]))?;

        Ok(())
    }

    pub fn use_on_target(&mut self, effect: Entity, user: Entity, target: Entity) -> Result<()> {
        let range = match self.usables.get(effect) {
            Some(&Usable::OnTarget { range }) => range,
            _ => bail!("not usable on target"),
        };

        let user_pos = *self.positions.get(user).context("invalid user")?;
        let target_pos = *self.positions.get(target).context("invalid target")?;

        ensure!(
            user_pos.distance(target_pos) <= range,
            "target out of range"
        );

        self.being_used
            .insert(effect, BeingUsed(smallvec![target]))?;

        Ok(())
    }

    pub fn use_on_ground(
        &mut self,
        effect: Entity,
        user: Entity,
        target_pos: Coordinate,
    ) -> Result<()> {
        let range = match self.usables.get(effect) {
            Some(&Usable::OnGround { range }) => range,
            _ => bail!("not usable on ground"),
        };

        let user_pos = *self.positions.get(user).context("invalid user")?;
        ensure!(
            user_pos.distance(target_pos) <= range,
            "target out of range"
        );

        let targets = self.map[target_pos].contents().into();
        self.being_used.insert(effect, BeingUsed(targets))?;

        Ok(())
    }
}
