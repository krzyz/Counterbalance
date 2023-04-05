use bevy::{prelude::*, utils::HashMap};

use crate::abilities::{Ability, AbilityType};

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
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
pub struct Abilities(pub HashMap<String, Ability>);

#[derive(Component)]
pub struct Attributes(pub HashMap<AttributeType, Attribute>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttributeType {
    HitPoints,
    //Stamina,
    //Enmity,
}

#[derive(Debug, Clone, Copy)]
pub enum Attribute {
    Value(i32),
    Gauge { value: i32, min: i32, max: i32 },
}

pub fn get_hit_ability(potency: i32) -> Ability {
    Ability {
        name: "hit".to_string(),
        typ: AbilityType::ChangeAttribute(AttributeType::HitPoints),
        potency,
        side_effect: None,
    }
}

pub fn spawn_character(
    commands: &mut Commands,
    name: &str,
    category: CharacterCategory,
    group: Group,
    ability: Ability,
) -> Entity {
    commands
        .spawn(CharacterBundle {
            name: CharacterName(name.to_string()),
            category,
            abilities: Abilities([(ability.name.clone(), ability)].into_iter().collect()),
            attributes: Attributes(
                [(
                    AttributeType::HitPoints,
                    Attribute::Gauge {
                        value: 50,
                        min: 0,
                        max: 50,
                    },
                )]
                .into_iter()
                .collect(),
            ),
            group,
        })
        .id()
}
