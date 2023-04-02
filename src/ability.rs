use bevy::prelude::*;

use crate::battle_screen::BattleLogEvent;
use crate::character::{
    Abilities, Attribute, AttributeClass, AttributeType, Attributes, CharacterBundle,
    CharacterCategory, CharacterName, Group,
};
use crate::AppState;

pub struct AbilityPlugin;

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AbilityCastEvent>()
            .add_system(setup_debug.in_schedule(OnEnter(AppState::Battle)))
            .add_system(debug)
            .add_system(cast_ability.in_set(OnUpdate(AppState::Battle)));
    }
}

#[derive(Debug, Clone)]
pub struct AbilityCastEvent {
    ability: Ability,
    by: Entity,
    on: Vec<Entity>,
}

#[derive(Debug, Clone)]
pub struct Ability {
    typ: AbilityType,
    potency: i32,
    side_effect: Option<Box<Ability>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AbilityType {
    ChangeAttribute(AttributeType),
}

fn debug(
    buttons: Res<Input<MouseButton>>,
    mut ev_ability: EventWriter<AbilityCastEvent>,
    query: Query<(Entity, &Abilities, &Group)>,
    query_2: Query<(Entity, &Group)>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        for (entity, abilities, group) in query.iter() {
            for (entity2, group2) in query_2.iter() {
                if let (Group::Player, Group::Enemy) = (group, group2) {
                    ev_ability.send(AbilityCastEvent {
                        ability: abilities.0[0].clone(),
                        by: entity,
                        on: vec![entity2],
                    })
                }
            }
        }
    }
}

fn setup_debug(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(CharacterBundle {
        name: CharacterName("player".to_string()),
        category: CharacterCategory::Human,
        abilities: Abilities(vec![Ability {
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
        group: Group::Player,
        sprite: SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-20.0, 20.0, 0.0),
                ..default()
            },
            texture: asset_server.load("images/human.png"),
            ..default()
        },
    });

    commands.spawn(CharacterBundle {
        name: CharacterName("fungus".to_string()),
        category: CharacterCategory::Fungi,
        abilities: Abilities(vec![Ability {
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
        group: Group::Enemy,
        sprite: SpriteBundle {
            transform: Transform {
                translation: Vec3::new(320.0, 20.0, 0.0),
                ..default()
            },
            texture: asset_server.load("images/fungus.png"),
            ..default()
        },
    });
}

fn cast_ability(
    mut ev_ability: EventReader<AbilityCastEvent>,
    mut ev_battle_log: EventWriter<BattleLogEvent>,
    mut query: Query<(Entity, &CharacterCategory, &mut Attributes)>,
) {
    for ability_cast in ev_ability.iter() {
        for (entity, _category, mut attributes) in query.iter_mut() {
            if ability_cast.on.contains(&entity) {
                match ability_cast.ability.typ {
                    AbilityType::ChangeAttribute(attribute_type) => {
                        if let Some(attribute) = attributes
                            .0
                            .iter_mut()
                            .find(|attr| attr.typ == attribute_type)
                        {
                            match &mut attribute.class {
                                AttributeClass::Value(v) => {
                                    *v -= ability_cast.ability.potency;
                                }
                                AttributeClass::Gauge { value, .. } => {
                                    *value -= ability_cast.ability.potency;
                                    ev_battle_log.send(BattleLogEvent {
                                        message: format!("Hp removed to: {}", *value),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
