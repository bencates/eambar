use crate::prelude::*;

#[derive(Component)]
pub struct CharacterSheet {
    hp: i32,
    max_hp: i32,
    power: i32,
    defense: i32,
}

impl CharacterSheet {
    pub fn new(hp: i32, power: i32, defense: i32) -> Self {
        Self {
            hp,
            max_hp: hp,
            power,
            defense,
        }
    }

    pub fn hp(&self) -> (i32, i32) {
        (self.hp, self.max_hp)
    }

    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    pub fn melee_damage(&self) -> i32 {
        self.power
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

pub struct MaintainCharacterSheetSystem;

impl<'a> System<'a> for MaintainCharacterSheetSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, CharacterSheet>,
        Write<'a, GameLog>,
    );

    fn run(
        &mut self,
        (entities, players, names, mut character_sheets, mut game_log): Self::SystemData,
    ) {
        for (entity, name, character) in (&entities, &names, &mut character_sheets).join() {
            if !character.is_alive() {
                if players.contains(entity) {
                    // TODO: handle player death
                    game_log.player_death();
                    character.hp = character.max_hp;
                } else {
                    game_log.death(name);

                    // // Removing the position clears the entity off the map immediately.
                    // // All other components will be removed automatically after the turn.
                    // positions.remove(entity).unwrap();
                    entities.delete(entity).unwrap();
                }
            }
        }
    }
}
