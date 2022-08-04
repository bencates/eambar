use super::{FULL_PAINT, MAP_HEIGHT, MAP_WIDTH, SIDEBAR_WIDTH, TERM_HEIGHT};
use crate::prelude::*;
use std::fmt::Display;

const LOG_WIDTH: i32 = MAP_WIDTH - 2;
const LOG_HEIGHT: i32 = TERM_HEIGHT - MAP_HEIGHT - 5;

pub struct GameLog {
    entries: Vec<String>,
}

impl Default for GameLog {
    fn default() -> Self {
        Self {
            entries: vec!["Welcome to Rusty Roguelike".to_string()],
        }
    }
}

impl GameLog {
    pub fn attack(&mut self, attacker: impl Display, target: impl Display, damage: i32) {
        self.log(if damage == 0 {
            format!("{} is unable to hurt {}.", attacker, target)
        } else {
            format!("{} hits {} for {} damage.", attacker, target, damage)
        });
    }

    pub fn player_pickup(&mut self, item: impl Display) {
        self.entries.push(format!("You picked up {item}"));
    }

    pub fn player_death(&mut self) {
        self.log("You died! Resetting your health.");
    }

    pub fn death(&mut self, victim: impl Display) {
        self.log(format!("{} died.", victim));
    }

    fn log(&mut self, msg: impl Display) {
        log::info!("{msg}");
        self.entries.push(msg.to_string());
    }
}

pub struct RenderGameLogSystem;

impl<'a> System<'a> for RenderGameLogSystem {
    type SystemData = Read<'a, GameLog>;

    fn run(&mut self, game_log: Self::SystemData) {
        let mut draw_batch = DrawBatch::new();
        let mut text = TextBuilder::empty();

        for entry in game_log.entries.iter().rev().take(LOG_HEIGHT as usize) {
            text.line_wrap(entry).ln();
        }

        let mut text_block =
            TextBlock::new(SIDEBAR_WIDTH + 2, MAP_HEIGHT + 3, LOG_WIDTH, LOG_HEIGHT);
        text_block.print(&text).ok(); // Ignore OutOfSpace
        text_block.render_to_draw_batch(&mut draw_batch);
        draw_batch.submit(2 * FULL_PAINT).unwrap();
    }
}
