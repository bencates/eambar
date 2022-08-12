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
    contents: Vec<Entity>,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            tile_type: TileType::Wall,
            blocked: true,
            revealed: false,
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

    pub fn iter(&self) -> impl Iterator<Item = Entity> + '_ {
        self.contents.iter().copied()
    }

    pub fn entity<T: Component>(&self, storage: &ReadStorage<T>) -> Option<Entity> {
        self.iter().find(|&entity| storage.contains(entity))
    }

    pub(super) fn reset_index(&mut self) {
        self.blocked = self.tile_type == TileType::Wall;
        self.contents.clear();
    }

    pub(super) fn block(&mut self) {
        self.blocked = true;
    }

    pub fn reveal(&mut self) {
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

        Ok(match tile.tile_type {
            TileType::Floor => Self::map_tile('░', TEAL),
            TileType::Wall => Self::map_tile('#', GREEN),
        })
    }
}
