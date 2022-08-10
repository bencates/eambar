use crate::prelude::*;
use std::fmt::{self, Display};

#[derive(Component)]
pub struct Appearance {
    pub(super) name: String,
    pub(super) glyph: char,
    pub(super) color: ColorPair,
    pub(super) z_order: i32,
}

impl Appearance {
    pub fn player() -> Self {
        Self {
            name: "Player".to_string(),
            glyph: '@',
            color: ColorPair::new(YELLOW, RGBA::new()),
            z_order: 4,
        }
    }

    pub fn monster(name: impl ToString, glyph: char, color: impl Into<RGBA>) -> Self {
        Self {
            name: name.to_string(),
            glyph,
            color: ColorPair::new(color, RGBA::new()),
            z_order: 3,
        }
    }

    pub fn item(name: impl ToString, glyph: char, color: impl Into<RGBA>) -> Self {
        Self {
            name: name.to_string(),
            glyph,
            color: ColorPair::new(color, RGBA::new()),
            z_order: 2,
        }
    }

    pub fn map_tile(glyph: char, color: impl Into<RGBA>) -> Self {
        Self {
            name: String::default(),
            glyph,
            color: ColorPair::new(color, BLACK),
            z_order: 0,
        }
    }
}

impl Display for Appearance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name)
    }
}
