use crate::prelude::*;

pub type SpawnBuilder = fn(EntityBuilder) -> EntityBuilder;

pub const SPAWN_TABLE: [SpawnBuilder; 4] = [
    monster::infected_crewmember,
    monster::alien_hatchling,
    item::repair_kit,
    item::grenade,
];

/// Marker trait for the player entity.
#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Player;

/// Marker trait for monsters.
#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Monster;

#[derive(Component, PartialEq)]
pub enum Item {
    Consumable,
}

pub fn player(entity: EntityBuilder) -> EntityBuilder {
    entity
        .with(Player)
        .with(Appearance::player())
        .with(Durability::new(30, 2))
        .with(Usable::OnTarget { range: 1 })
        .with(DealsDamage(5))
        .with(Viewshed::new(25))
}

pub mod monster {
    use crate::prelude::*;

    pub fn infected_crewmember(entity: EntityBuilder) -> EntityBuilder {
        entity
            .with(Monster)
            .with(Appearance::monster("Infected Crewmember", 'z', RED))
            .with(Durability::new(16, 1))
            .with(Usable::OnTarget { range: 1 })
            .with(DealsDamage(4))
            .with(Viewshed::new(25))
            .with(BlocksTile)
    }

    pub fn alien_hatchling(entity: EntityBuilder) -> EntityBuilder {
        entity
            .with(Monster)
            .with(Appearance::monster("Alien Hatchling", 'h', RED))
            .with(Durability::new(16, 1))
            .with(Usable::OnTarget { range: 1 })
            .with(DealsDamage(4))
            .with(Viewshed::new(25))
            .with(BlocksTile)
    }
}

mod item {
    use crate::prelude::*;

    pub fn repair_kit(entity: EntityBuilder) -> EntityBuilder {
        entity
            .with(Item::Consumable)
            .with(Appearance::item("Repair Kit", 'δ', ORANGE))
            .with(Usable::OnSelf)
            .with(ProvidesHealing(8))
    }

    pub fn grenade(entity: EntityBuilder) -> EntityBuilder {
        entity
            .with(Item::Consumable)
            .with(Appearance::item("Grenade", '*', ORANGE)) // FIXME: better glyph
            .with(Usable::OnTarget { range: 6 })
            .with(DealsDamage(9))
    }
}
