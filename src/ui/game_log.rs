use super::{MAP_HEIGHT, SIDEBAR_WIDTH, TERM_HEIGHT};
use crate::prelude::*;
use std::fmt::Display;

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

    // pub fn player_pickup(&mut self, item: impl Display) {
    //     self.entries.push(format!("You picked up {item}"));
    // }

    pub fn player_death(&mut self) {
        self.log("You died! Resetting your health.");
    }

    pub fn death(&mut self, victim: impl Display) {
        self.log(format!("{} died.", victim));
    }

    // pub fn error(&mut self, error: impl Debug + Display) {
    //     console::log(format!("ERROR: {:?}", error));
    //     self.entries.push(error.to_string());
    // }

    fn log(&mut self, msg: impl Display) {
        log::info!("{msg}");
        self.entries.push(msg.to_string());
    }
}

pub(super) fn draw(ctx: &mut BTerm, world: &World) {
    let game_log = world.fetch::<GameLog>();

    for (idx, entry) in game_log
        .entries
        .iter()
        .rev()
        .enumerate()
        .take(LOG_HEIGHT as usize)
    {
        ctx.print(SIDEBAR_WIDTH + 2, MAP_HEIGHT + 3 + idx as i32, entry);
    }
}
