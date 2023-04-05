use bevy::{prelude::*, utils::HashMap};

use crate::abilities::Ability;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Group {
    Player,
    Enemy,
}

#[derive(Component, Debug, Clone)]
pub struct CharacterName(pub String);

#[derive(Bundle, Debug, Clone)]
pub struct CharacterBundle {
    pub name: CharacterName,
    pub category: CharacterCategory,
    pub abilities: Abilities,
    pub attributes: Attributes,
    pub group: Group,
}

impl CharacterBundle {
    pub fn new(
        name: &str,
        category: CharacterCategory,
        abilities: &[Ability],
        attributes: &[(AttributeType, i32)],
        group: Group,
    ) -> Self {
        Self {
            name: CharacterName(name.to_string()),
            category,
            abilities: Abilities(
                abilities
                    .into_iter()
                    .map(|ability| (ability.name.clone(), ability.clone()))
                    .collect(),
            ),
            attributes: Attributes(
                attributes
                    .into_iter()
                    .copied()
                    .map(|(typ, value)| {
                        (
                            typ,
                            match typ.get_corresponding_value_type() {
                                AttributeValueType::Value => Attribute::Value(value),
                                AttributeValueType::Gauge => Attribute::Gauge {
                                    value,
                                    min: 0,
                                    max: value,
                                },
                            },
                        )
                    })
                    .collect(),
            ),
            group,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Character {
    pub bundle: CharacterBundle,
    pub image_path: String,
}

impl Character {
    pub fn new(bundle: CharacterBundle, image_path: &str) -> Self {
        Self {
            bundle,
            image_path: image_path.to_string(),
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterCategory {
    Human,
    //Cat,
    Fungus,
}

#[derive(Component, Debug, Clone, Default)]
pub struct Abilities(pub HashMap<String, Ability>);

#[derive(Component, Debug, Clone)]
pub struct Attributes(pub HashMap<AttributeType, Attribute>);

impl Default for Attributes {
    fn default() -> Self {
        Attributes(
            [
                (
                    AttributeType::HitPoints,
                    Attribute::Gauge {
                        value: 50,
                        min: 0,
                        max: 50,
                    },
                ),
                (AttributeType::Attack, Attribute::Value(10)),
                (AttributeType::Defense, Attribute::Value(10)),
            ]
            .into_iter()
            .collect(),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttributeType {
    HitPoints,
    Attack,
    Defense,
}

impl AttributeType {
    pub fn get_corresponding_value_type(&self) -> AttributeValueType {
        use AttributeValueType::*;

        match self {
            Self::HitPoints => Gauge,
            Self::Attack => Value,
            Self::Defense => Value,
        }
    }
}

// I was hoping to find a crate with a macro
// to autogenerate that...
#[derive(Debug, Clone, Copy)]
pub enum AttributeValueType {
    Value,
    Gauge,
}

#[derive(Debug, Clone, Copy)]
pub enum Attribute {
    Value(i32),
    Gauge { value: i32, min: i32, max: i32 },
}
