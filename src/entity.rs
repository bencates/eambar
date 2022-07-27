use crate::prelude::*;

/// Marker trait for the player entity.
#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Player;

/// Marker trait for monsters.
#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Monster;

pub fn player(entity: EntityBuilder) -> EntityBuilder {
    entity
        .with(Player)
        // .with(Name::new("Player"))
        .with(Appearance {
            color: ColorPair::new(YELLOW, BLACK),
            glyph: '@',
        })
        .with(Viewshed::new(8))
    // .with(CombatStats::new(30, 5, 2))
}

pub mod monster {
    use crate::prelude::*;

    pub fn infected_crewmember(entity: EntityBuilder) -> EntityBuilder {
        entity
            .with(Monster)
            // .with(Name::new("Infected Crewmember"))
            .with(Appearance {
                color: ColorPair::new(RED, BLACK),
                glyph: 'z',
            })
            .with(BlocksTile)
            .with(Viewshed::new(8))
        // .with(CombatStats::new(16, 4, 1))
    }

    pub fn alien_hatchling(entity: EntityBuilder) -> EntityBuilder {
        entity
            .with(Monster)
            // .with(Name::new("Alien Hatchling"))
            .with(Appearance {
                color: ColorPair::new(RED, BLACK),
                glyph: 'h',
            })
            .with(BlocksTile)
            .with(Viewshed::new(8))
        // .with(CombatStats::new(16, 4, 1))
    }
}
