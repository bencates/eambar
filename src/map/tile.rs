use crate::prelude::*;

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Floor,
    Wall,
}

pub struct Tile {
    tile_type: TileType,
    blocked: bool,
    revealed: bool,
    visible: bool,
    contents: Vec<Entity>,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            tile_type: TileType::Wall,
            blocked: true,
            revealed: false,
            visible: false,
            contents: Vec::new(),
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
            blocked: false,
            ..Default::default()
        }
    }

    pub fn is_opaque(&self) -> bool {
        self.tile_type == TileType::Wall
    }

    pub fn is_blocked(&self) -> bool {
        self.blocked
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    // pub fn contents(&self) -> &[Entity] {
    //     &self.contents
    // }

    // pub fn entity<T: Component>(&self, world: &World) -> Option<Entity> {
    //     let storage = world.read_component::<T>();

    //     self.contents
    //         .iter()
    //         .find(|entity| storage.contains(**entity))
    //         .copied()
    // }

    pub(super) fn reset_index(&mut self) {
        self.blocked = self.tile_type == TileType::Wall;
        self.visible = false;
        self.contents.clear();
    }

    pub(super) fn block(&mut self) {
        self.blocked = true;
    }

    pub(super) fn reveal(&mut self) {
        self.visible = true;
        self.revealed = true;
    }

    pub(super) fn add_entity(&mut self, entity: Entity) {
        self.contents.push(entity);
    }
}

impl TryFrom<&Tile> for Appearance {
    type Error = ();

    fn try_from(tile: &Tile) -> Result<Self, Self::Error> {
        if !tile.revealed {
            return Err(());
        }

        let (mut color, glyph) = match tile.tile_type {
            TileType::Floor => (ColorPair::new(TEAL, BLACK), '.'),
            TileType::Wall => (ColorPair::new(GREEN, BLACK), '#'),
        };

        if !tile.visible {
            color.fg = color.fg.to_greyscale();
        }

        Ok(Self { color, glyph })
    }
}
