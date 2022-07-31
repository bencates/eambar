mod bisection_generator;
mod template;

use crate::prelude::*;

pub struct DeckBuilder {
    map: Map,
}

impl DeckBuilder {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            map: template::empty_deck(width, height),
        }
    }

    pub fn with_engines(mut self) -> Self {
        template::add_engines(&mut self.map);
        self
    }

    pub fn with_walls(mut self, rng: &mut RandomNumberGenerator) -> Self {
        bisection_generator::add_walls(&mut self.map, rng);
        self
    }

    pub fn build(self) -> Map {
        self.map
    }
}
