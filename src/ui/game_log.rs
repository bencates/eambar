use super::{FULL_PAINT, MAP_HEIGHT, MAP_WIDTH, SIDEBAR_WIDTH, TERM_HEIGHT};
use crate::prelude::*;

const LOG_WIDTH: i32 = MAP_WIDTH - 2;
const LOG_HEIGHT: i32 = TERM_HEIGHT - MAP_HEIGHT - 5;

pub struct GameLog {
    entries: Vec<TextBuilder>,
}

impl Default for GameLog {
    fn default() -> Self {
        let mut entries = vec![TextBuilder::empty()];
        entries[0].append("Welcome to Rusty Roguelike").ln();

        Self { entries }
    }
}

impl GameLog {
    pub fn attack(&mut self, attacker: &Appearance, target: &Appearance, damage: i32) {
        self.log(|text| {
            if damage == 0 {
                log::info!("{attacker} is unable to hurt {target}.");
                text.append(&format!("{attacker} is unable to hurt {target}."));
            } else {
                log::info!("{attacker} hits {target} for {damage} damage.");
                text.append(&format!("{attacker} hits {target} for {damage} damage."));
            }
        });
    }

    pub fn player_pickup(&mut self, item: &Appearance) {
        log::info!("You picked up {item}");
        self.log(|text| {
            text.append(&format!("You picked up {item}"));
        });
    }

    pub fn player_death(&mut self) {
        log::info!("You died! Resetting your health.");
        self.log(|text| {
            text.append("You died! Resetting your health.");
        });
    }

    pub fn death(&mut self, victim: &Appearance) {
        log::info!("{victim} died.");
        self.log(|text| {
            text.append(&format!("{victim} died."));
        });
    }

    fn log(&mut self, f: impl FnOnce(&mut TextBuilder)) {
        let mut text = TextBuilder::empty();
        f(&mut text);
        text.ln();
        self.entries.push(text);
    }
}

pub struct RenderGameLogSystem;

impl<'a> System<'a> for RenderGameLogSystem {
    type SystemData = Read<'a, GameLog>;

    fn run(&mut self, game_log: Self::SystemData) {
        let mut draw_batch = DrawBatch::new();
        let mut text_block =
            TextBlock::new(SIDEBAR_WIDTH + 2, MAP_HEIGHT + 3, LOG_WIDTH, LOG_HEIGHT);

        for entry in game_log.entries.iter().rev().take(LOG_HEIGHT as usize) {
            text_block.print(entry).ok(); // Ignore OutOfSpace
        }

        text_block.render_to_draw_batch(&mut draw_batch);
        draw_batch.submit(2 * FULL_PAINT).unwrap();
    }
}
