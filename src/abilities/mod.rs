pub mod choose_ability_screen;

use bevy::prelude::*;
use enumset::{EnumSet, EnumSetType};

use crate::battle::battle_field::{BattleField, Tile};
use crate::battle::lifecycle::BattleLifecycleEvent;
use crate::battle::log::BattleLogEvent;
use crate::character::{Attribute, AttributeType, Attributes, CharacterName};
use crate::AppState;

use self::choose_ability_screen::{cleanup_ability_screen, setup_ability_screen};

pub struct AbilityPlugin;

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TurnEvent>()
            .add_system(resolve_ability.in_set(OnUpdate(AppState::Battle)))
            .add_system(setup_ability_screen.in_schedule(OnEnter(AppState::AbilityChoose)))
            .add_system(cleanup_ability_screen.in_schedule(OnExit(AppState::AbilityChoose)));
    }
}

#[derive(Debug, Clone)]
pub enum TurnEvent {
    Ability {
        ability: Ability,
        by: Entity,
        on: Entity,
    },
    Pass(Entity),
}

#[derive(Debug, Clone)]
pub struct Ability {
    pub name: String,
    pub typ: AbilityType,
    pub target: EnumSet<AbilityTargetType>,
    pub range: i32,
    pub side_effect: Option<Box<Ability>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbilityProximity {
    Melee,
    Ranged,
}

#[derive(EnumSetType, Debug)]
pub enum AbilityTargetType {
    Empty,
    Ally,
    Enemy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbilityType {
    Targeted {
        ab_typ: TargetedAbilityType,
        proximity: AbilityProximity,
    },
    Movement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetedAbilityType {
    ChangeAttribute { at_typ: AttributeType, potency: i32 },
}

fn move_char(
    who: Entity,
    where_to: Entity,
    battle_field: &BattleField,
    caster_name: &str,
    commands: &mut Commands,
    parent_query: &mut Query<&mut Parent, With<CharacterName>>,
    ev_battle_log: &mut EventWriter<BattleLogEvent>,
) {
    let caster_tile = parent_query.get(who).expect("Missing player tile").get();

    let caster_hex = battle_field
        .hex(caster_tile)
        .expect("Missing hex for caster")
        .to_oddr();
    let target_hex = battle_field
        .hex(where_to)
        .expect("Missing hex for target")
        .to_oddr();

    ev_battle_log.send(BattleLogEvent {
        message: format!(
            "{caster_name} moved from ({}, {}) to ({}, {})",
            caster_hex.x, caster_hex.y, target_hex.x, target_hex.y
        ),
    });
    commands.entity(where_to).push_children(&[who]);
}

fn calculate_damage(potency: i32, attack: i32, defense: i32) -> i32 {
    potency * (1.1f32.powi(attack - defense)).round() as i32
}

fn resolve_ability(
    mut commands: Commands,
    battle_field: Option<Res<BattleField>>,
    mut ev_ability: EventReader<TurnEvent>,
    mut ev_battle_log: EventWriter<BattleLogEvent>,
    mut ev_lifecycle: EventWriter<BattleLifecycleEvent>,
    mut set: ParamSet<(
        Query<Option<&Children>, With<Tile>>,
        Query<(&CharacterName, &mut Attributes)>,
        Query<(&CharacterName, &mut Attributes)>,
    )>,
    mut parent_query: Query<&mut Parent, With<CharacterName>>,
    tile_children_query: Query<&Children, With<Tile>>,
) {
    for turn in ev_ability.iter() {
        let battle_field = battle_field
            .as_ref()
            .expect("Missing battle field")
            .as_ref();

        match turn {
            TurnEvent::Ability { ability, by, on } => {
                let char_query = set.p1();
                let (caster_name, caster_attributes) =
                    char_query.get(*by).expect("Missing caster entity");

                let (caster_name, caster_attributes) =
                    (caster_name.0.clone(), caster_attributes.clone());

                let tile_query = set.p0();
                let children = tile_query.get(*on).expect("Missing ability target");

                match ability.typ {
                    AbilityType::Targeted { ab_typ, proximity } => {
                        if let AbilityProximity::Melee = proximity {
                            let from_tile =
                                parent_query.get(*by).expect("Missing caster tile").get();
                            let from = battle_field.hex(from_tile).expect("Caster hex not found");
                            let hex = battle_field.hex(*on).expect("Target hex not found");
                            if from.dist(hex) > 1 {
                                let where_to = battle_field
                                    .get_in_range_and_empty(
                                        hex,
                                        from,
                                        ability.range,
                                        &tile_children_query,
                                    )
                                    .expect("Couldn't find place for the entity to move to");

                                move_char(
                                    *by,
                                    where_to,
                                    battle_field,
                                    caster_name.as_str(),
                                    &mut commands,
                                    &mut parent_query,
                                    &mut ev_battle_log,
                                );
                            }
                        }
                        match ab_typ {
                            TargetedAbilityType::ChangeAttribute { at_typ, potency } => {
                                let entity = *children
                                    .expect("Expected children on a tile")
                                    .iter()
                                    .next()
                                    .expect("Expected an entity on a tile");

                                let mut target_query = set.p2();
                                let (name, mut target_attributes) = target_query
                                    .get_mut(entity)
                                    .expect("Didn't find target entity");
                                let target_defense = target_attributes
                                    .0
                                    .get(&AttributeType::Defense)
                                    .expect("Missing attack attribute")
                                    .get_value();

                                if let Some(attribute) = target_attributes.0.get_mut(&at_typ) {
                                    match attribute {
                                        Attribute::Value(v) => {
                                            *v -= potency;
                                        }
                                        Attribute::Gauge { value, min, max } => {
                                            let final_value = calculate_damage(
                                                potency,
                                                caster_attributes
                                                    .0
                                                    .get(&AttributeType::Attack)
                                                    .expect("Missing attack attribute")
                                                    .get_value(),
                                                target_defense,
                                            );
                                            *value = (*value - final_value).clamp(*min, *max);

                                            ev_battle_log.send(BattleLogEvent {
                                                message: format!(
                                                "{caster_name} used {} on {}. Hp removed to: {}",
                                                ability.name, name.0, *value
                                            ),
                                            });

                                            if at_typ == AttributeType::HitPoints && *value <= 0 {
                                                ev_lifecycle.send(
                                                    BattleLifecycleEvent::CharacterDied(entity),
                                                )
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    AbilityType::Movement => {
                        move_char(
                            *by,
                            *on,
                            battle_field,
                            caster_name.as_str(),
                            &mut commands,
                            &mut parent_query,
                            &mut ev_battle_log,
                        );
                    }
                }
            }
            TurnEvent::Pass(caster) => {
                let caster_name = set
                    .p1()
                    .get(*caster)
                    .expect("Missing caster entity")
                    .0
                     .0
                    .clone();

                ev_battle_log.send(BattleLogEvent {
                    message: format!("{caster_name} waits"),
                });
            }
        }
        ev_lifecycle.send(BattleLifecycleEvent::EndTurn)
    }
}
