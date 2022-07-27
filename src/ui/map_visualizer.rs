use super::{map, TERM_HEIGHT, TERM_WIDTH};
use crate::map::{MapBuilder, SimpleMapBuilder};
use crate::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum MapVisualizerState {
    Setup,
    Idle,
}

pub fn visualize_map(ctx: &mut BTerm, world: &mut World, state: MapVisualizerState) -> RunState {
    if state == MapVisualizerState::Setup {
        let mut map = SimpleMapBuilder::new(TERM_WIDTH, TERM_HEIGHT).build();
        map.reveal();
        world.insert(map);
    }

    ctx.cls();
    map::draw(ctx, world);

    if ctx.key == Some(VirtualKeyCode::Q) {
        ctx.quit();
    }

    RunState::GenerateMap(MapVisualizerState::Idle)
}
