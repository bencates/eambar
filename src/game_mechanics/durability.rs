use crate::prelude::*;

use super::HasInitiative;

#[derive(Component)]
pub struct Durability {
    health: i32,
    max_health: i32,
    shield: i32,
    max_shield: i32,
    defense: i32,
    shield_defense: i32,
    took_damage: bool,
    shield_regen: i32,
    // TODO: broken_shield_cooldown: i32
}

impl Durability {
    pub fn new(health: i32, defense: i32) -> Self {
        Self {
            health,
            max_health: health,
            shield: 0,
            max_shield: 0,
            defense,
            shield_defense: 0,
            shield_regen: 0,
            took_damage: false,
        }
    }

    pub fn with_shield(mut self, shield: i32, shield_defense: i32) -> Self {
        self.shield = shield;
        self.max_shield = shield;
        self.shield_defense = shield_defense;
        self.shield_regen = 5; // FIXME: don't hardcode

        self
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

    /// Returns the amount actually healed
    pub fn heal(&mut self, healing: i32) -> i32 {
        let healing = i32::min(healing, self.max_health - self.health);
        self.health += healing;

        healing
    }

    /// Returns the amount of damage actually taken
    pub fn take_damage(&mut self, damage: i32) -> i32 {
        self.took_damage = true;

        let (damage_to_shield, unshielded_damage) = if self.shield > 0 {
            let blocked_damage = i32::max(0, damage - self.shield_defense);
            let damage_to_shield = i32::min(blocked_damage, self.shield);
            self.shield -= damage_to_shield;

            (damage_to_shield, blocked_damage - damage_to_shield)
        } else {
            (0, damage)
        };

        let damage_to_health = i32::clamp(unshielded_damage - self.defense, 0, self.health);
        self.health -= damage_to_health;

        damage_to_shield + damage_to_health
    }
}

pub struct ShieldRegenSystem;

impl<'a> System<'a> for ShieldRegenSystem {
    type SystemData = (WriteStorage<'a, Durability>, ReadStorage<'a, HasInitiative>);

    fn run(&mut self, (mut durabilities, has_initiative): Self::SystemData) {
        for (durability, _) in (&mut durabilities, &has_initiative).join() {
            if durability.took_damage {
                durability.took_damage = false;
                continue;
            }

            durability.shield = i32::min(
                durability.shield + durability.shield_regen,
                durability.max_shield,
            );
        }
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

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_DURABILITY: Durability = Durability {
        health: 30,
        max_health: 30,
        shield: 10,
        max_shield: 10,
        defense: 2,
        shield_defense: 1,
        shield_regen: 5,
        took_damage: false,
    };

    #[test]
    fn cant_heal_past_max_health() {
        for (healing, expected_healing_done, expected_health) in [
            (0, 0, (28, 30)),
            (1, 1, (29, 30)),
            (2, 2, (30, 30)),
            (3, 2, (30, 30)),
        ] {
            let mut durability = Durability {
                health: 28,
                ..EXAMPLE_DURABILITY
            };

            let healing_done = durability.heal(healing);

            assert_eq!(healing_done, expected_healing_done);
            assert_eq!(durability.health(), expected_health);
        }
    }

    #[test]
    fn deals_damage_through_shields_correctly() {
        for (damage, expected_damage_taken, expected_health, expected_shield) in [
            (0, 0, (30, 30), Some((2, 10))),
            (1, 0, (30, 30), Some((2, 10))),
            (2, 1, (30, 30), Some((1, 10))),
            (3, 2, (30, 30), Some((0, 10))),
            (4, 2, (30, 30), Some((0, 10))),
            (5, 2, (30, 30), Some((0, 10))),
            (6, 3, (29, 30), Some((0, 10))),
            (99, 32, (0, 30), Some((0, 10))),
        ] {
            let mut durability = Durability {
                shield: 2,
                ..EXAMPLE_DURABILITY
            };

            let damage_taken = durability.take_damage(damage);

            assert_eq!(damage_taken, expected_damage_taken);
            assert_eq!(durability.health(), expected_health);
            assert_eq!(durability.shield(), expected_shield);
        }
    }

    #[test]
    fn shield_defense_doesnt_apply_when_shield_is_broken_or_missing() {
        let mut no_shield = Durability {
            shield: 0,
            max_shield: 0,
            ..EXAMPLE_DURABILITY
        };

        let mut broken_shield = Durability {
            shield: 0,
            ..EXAMPLE_DURABILITY
        };

        let no_shield_damage_taken = no_shield.take_damage(3);
        let broken_shield_damage_taken = broken_shield.take_damage(3);

        assert_eq!(no_shield_damage_taken, 1);
        assert_eq!(no_shield.health(), (29, 30));
        assert_eq!(no_shield.shield(), None);

        assert_eq!(broken_shield_damage_taken, 1);
        assert_eq!(broken_shield.health(), (29, 30));
        assert_eq!(broken_shield.shield(), Some((0, 10)));
    }
}
