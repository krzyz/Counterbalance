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
            abilities: Abilities::from_arr(abilities),
            attributes: Attributes(
                attributes
                    .iter()
                    .copied()
                    .map(|(typ, value)| {
                        (
                            typ,
                            match typ.get_corresponding_value_type() {
                                AttributeValueType::Value => Attribute::value(value),
                                AttributeValueType::Gauge => Attribute::gauge(value),
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

impl Abilities {
    pub fn from_arr(abilities: &[Ability]) -> Self {
        Self(
            abilities
                .iter()
                .map(|ability| (ability.name.clone(), ability.clone()))
                .collect(),
        )
    }
}

#[derive(Component, Debug, Clone)]
pub struct Attributes(pub HashMap<AttributeType, Attribute>);

impl Default for Attributes {
    fn default() -> Self {
        Attributes(
            [
                (AttributeType::HitPoints, Attribute::gauge(150)),
                (AttributeType::Attack, Attribute::value(10)),
                (AttributeType::Defense, Attribute::value(10)),
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

impl Attribute {
    pub fn get_value(&self) -> i32 {
        match self {
            Self::Value(v) => *v,
            Self::Gauge { value, .. } => *value,
        }
    }
    pub fn value(value: i32) -> Self {
        Self::Value(value)
    }

    pub fn gauge(max: i32) -> Self {
        Self::Gauge {
            value: max,
            min: 0,
            max,
        }
    }
}
