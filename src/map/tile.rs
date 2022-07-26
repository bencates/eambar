use crate::prelude::*;

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Floor,
    Wall,
}

pub struct Tile {
    tile_type: TileType,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            tile_type: TileType::Wall,
        }
    }
}

impl Tile {
    pub fn wall() -> Self {
        Default::default()
    }

    pub fn floor() -> Self {
        Self {
            tile_type: TileType::Floor,
            ..Default::default()
        }
    }

    pub fn is_blocked(&self) -> bool {
        self.tile_type == TileType::Wall
    }
}

impl TryFrom<&Tile> for Appearance {
    type Error = ();

    fn try_from(tile: &Tile) -> Result<Self, Self::Error> {
        let (color, glyph) = match tile.tile_type {
            TileType::Floor => (ColorPair::new(TEAL, BLACK), '.'),
            TileType::Wall => (ColorPair::new(GREEN, BLACK), '#'),
        };

        Ok(Self { color, glyph })
    }
}
