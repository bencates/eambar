use crate::prelude::*;

pub type SpawnBuilder = fn(EntityBuilder) -> EntityBuilder;

pub const SPAWN_TABLE: [SpawnBuilder; 3] = [
    monster::infected_crewmember,
    monster::alien_hatchling,
    item::repair_kit,
];

/// Marker trait for the player entity.
#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Player;

/// Marker trait for monsters.
#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Monster;

/// Marker trait for monsters.
#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Item;

pub fn player(entity: EntityBuilder) -> EntityBuilder {
    entity
        .with(Player)
        .with(Name::new("Player"))
        .with(Appearance {
            color: ColorPair::new(YELLOW, RGBA::new()),
            glyph: '@',
            z_order: 3,
        })
        .with(CharacterSheet::new(30, 5, 2))
        .with(Viewshed::new(25))
}

pub mod monster {
    use crate::prelude::*;

    pub fn infected_crewmember(entity: EntityBuilder) -> EntityBuilder {
        entity
            .with(Monster)
            .with(Name::new("Infected Crewmember"))
            .with(Appearance {
                color: ColorPair::new(RED, RGBA::new()),
                glyph: 'z',
                z_order: 2,
            })
            .with(CharacterSheet::new(16, 4, 1))
            .with(Viewshed::new(25))
            .with(BlocksTile)
    }

    pub fn alien_hatchling(entity: EntityBuilder) -> EntityBuilder {
        entity
            .with(Monster)
            .with(Name::new("Alien Hatchling"))
            .with(Appearance {
                color: ColorPair::new(RED, RGBA::new()),
                glyph: 'h',
                z_order: 2,
            })
            .with(CharacterSheet::new(16, 4, 1))
            .with(Viewshed::new(25))
            .with(BlocksTile)
    }
}

mod item {
    use crate::prelude::*;

    pub fn repair_kit(entity: EntityBuilder) -> EntityBuilder {
        entity
            .with(Item)
            .with(Name::new("Repair Kit"))
            .with(Appearance {
                color: ColorPair::new(ORANGE, RGBA::new()),
                glyph: 'δ',
                z_order: 1,
            })
            .with(ProvidesHealing(8))
    }
}
