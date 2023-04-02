use bevy::prelude::*;

use crate::ability::{Ability, AbilityType};

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

pub fn spawn_character(
    commands: &mut Commands,
    name: &str,
    category: CharacterCategory,
    group: Group,
) -> Entity {
    commands
        .spawn(CharacterBundle {
            name: CharacterName(name.to_string()),
            category,
            abilities: Abilities(vec![Ability {
                name: "hit".to_string(),
                typ: AbilityType::ChangeAttribute(AttributeType::HitPoints),
                potency: 5,
                side_effect: None,
            }]),
            attributes: Attributes(vec![Attribute {
                typ: AttributeType::HitPoints,
                class: AttributeClass::Gauge {
                    value: 50,
                    min: 0,
                    max: 50,
                },
            }]),
            group,
        })
        .id()
}
