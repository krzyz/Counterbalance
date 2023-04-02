use bevy::prelude::*;

use crate::ability::Ability;

#[derive(Component)]
pub enum Group {
    Player,
    Enemy,
}

#[derive(Component)]
pub struct CharacterName(pub String);

#[derive(Bundle)]
pub struct CharacterBundle {
    pub name: CharacterName,
    pub category: CharacterCategory,
    pub abilities: Abilities,
    pub attributes: Attributes,
    pub group: Group,

    #[bundle]
    pub sprite: SpriteBundle,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterCategory {
    Human,
    //Cat,
    Fungi,
}

#[derive(Component)]
pub struct Abilities(pub Vec<Ability>);

#[derive(Component)]
pub struct Attributes(pub Vec<Attribute>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttributeType {
    HitPoints,
    //Stamina,
    //Enmity,
}

#[derive(Debug, Clone, Copy)]
pub enum AttributeClass {
    Value(i32),
    Gauge { value: i32, min: i32, max: i32 },
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Attribute {
    pub typ: AttributeType,
    pub class: AttributeClass,
}
