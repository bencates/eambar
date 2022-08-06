use crate::prelude::*;

#[derive(Component)]
pub struct Durability {
    health: i32,
    max_health: i32,
    shield: i32,
    max_shield: i32,
    defense: i32,
}

impl Durability {
    pub fn new(health: i32, shield: i32, defense: i32) -> Self {
        Self {
            health,
            max_health: health,
            shield,
            max_shield: shield,
            defense,
        }
    }

    pub fn health(&self) -> (i32, i32) {
        (self.health, self.max_health)
    }

    pub fn shield(&self) -> Option<(i32, i32)> {
        (self.max_shield > 0).then(|| (self.shield, self.max_shield))
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn block_damage(&self, raw_damage: i32) -> i32 {
        i32::max(0, raw_damage - self.defense)
    }

    pub fn heal(&mut self, healing: i32) {
        self.health = i32::min(self.max_health, self.health + healing);
    }

    pub fn apply_damage(&mut self, damage: i32) {
        self.health -= damage;
    }
}

pub struct DeathSystem;

impl<'a> System<'a> for DeathSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Appearance>,
        WriteStorage<'a, Durability>,
        WriteStorage<'a, Coordinate>,
        Write<'a, GameLog>,
    );

    fn run(
        &mut self,
        (entities, players, names, mut durabilities, mut positions, mut game_log): Self::SystemData,
    ) {
        for (entity, appearance, durability) in (&entities, &names, &mut durabilities).join() {
            if !durability.is_alive() {
                if players.contains(entity) {
                    // TODO: handle player death
                    game_log.player_death();
                    durability.health = durability.max_health;
                } else {
                    game_log.death(appearance);

                    // Removing the position clears the entity off the map immediately.
                    // All other components will be removed automatically after the turn.
                    positions.remove(entity).unwrap();
                    entities.delete(entity).unwrap();
                }
            }
        }
    }
}
