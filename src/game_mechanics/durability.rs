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
    use {super::*, test_case::test_case};

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

    #[test_case(0 => (0, (28, 30)); "no healing")]
    #[test_case(1 => (1, (29, 30)); "partial healing")]
    #[test_case(2 => (2, (30, 30)); "full healing")]
    #[test_case(3 => (2, (30, 30)); "overhealing")]
    fn cant_heal_past_max_health(healing: i32) -> (i32, (i32, i32)) {
        let mut durability = Durability {
            health: 28,
            ..EXAMPLE_DURABILITY
        };

        let healing_done = durability.heal(healing);

        (healing_done, durability.health())
    }

    #[test_case(0 => (0, (30, 30), (2, 10)); "no damage")]
    #[test_case(1 => (0, (30, 30), (2, 10)); "damage absorbed by shield defense")]
    #[test_case(2 => (1, (30, 30), (1, 10)); "some damage to shield")]
    #[test_case(3 => (2, (30, 30), (0, 10)); "broken shield")]
    #[test_case(5 => (2, (30, 30), (0, 10)); "broken shield with overflow absorbed")]
    #[test_case(6 => (3, (29, 30), (0, 10)); "damage to shield and health")]
    #[test_case(99 => (32, (0, 30), (0, 10)); "overkill")]
    fn deals_damage_through_shields_correctly(damage: i32) -> (i32, (i32, i32), (i32, i32)) {
        let mut durability = Durability {
            shield: 2,
            ..EXAMPLE_DURABILITY
        };

        let damage_taken = durability.take_damage(damage);

        (
            damage_taken,
            durability.health(),
            durability.shield().unwrap(),
        )
    }

    #[test_case(0, 0; "no shield")]
    #[test_case(0, 10; "broken shield")]
    fn shield_defense_doesnt_apply_when_shield_is_down(shield: i32, max_shield: i32) {
        let mut durability = Durability {
            shield,
            max_shield,
            ..EXAMPLE_DURABILITY
        };

        let damage_taken = durability.take_damage(3);

        assert_eq!(damage_taken, 1);
        assert_eq!(durability.health(), (29, 30));
    }
}
