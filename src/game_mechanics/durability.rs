use crate::prelude::*;

#[derive(Component)]
pub struct Durability {
    hp: i32,
    max_hp: i32,
    defense: i32,
}

impl Durability {
    pub fn new(hp: i32, defense: i32) -> Self {
        Self {
            hp,
            max_hp: hp,
            defense,
        }
    }

    pub fn hp(&self) -> (i32, i32) {
        (self.hp, self.max_hp)
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    pub fn block_damage(&self, raw_damage: i32) -> i32 {
        i32::max(0, raw_damage - self.defense)
    }

    pub fn heal(&mut self, healing: i32) {
        self.hp = i32::min(self.max_hp, self.hp + healing);
    }

    pub fn apply_damage(&mut self, damage: i32) {
        self.hp -= damage;
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
                    durability.hp = durability.max_hp;
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
