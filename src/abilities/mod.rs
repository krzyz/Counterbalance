pub mod choose_ability_screen;

use bevy::prelude::*;
use enumset::{EnumSet, EnumSetType};

use crate::battle::battle_field::{BattleField, Tile};
use crate::battle::lifecycle::BattleLifecycleEvent;
use crate::battle::log::BattleLogEvent;
use crate::character::{Attribute, AttributeType, Attributes, CharacterName};
use crate::AppState;

pub struct AbilityPlugin;

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TurnEvent>()
            .add_system(resolve_ability.in_set(OnUpdate(AppState::Battle)));
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

#[derive(EnumSetType, Debug)]
pub enum AbilityTargetType {
    Empty,
    Ally,
    Enemy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbilityType {
    ChangeAttribute { typ: AttributeType, potency: i32 },
    Movement,
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
        Query<&mut Parent, With<CharacterName>>,
    )>,
) {
    for turn in ev_ability.iter() {
        match turn {
            TurnEvent::Ability { ability, by, on } => {
                let caster_name = set
                    .p1()
                    .get(*by)
                    .expect("Missing caster entity")
                    .0
                     .0
                    .clone();

                let tile_query = set.p0();
                let children = tile_query.get(*on).expect("Missing ability target");

                match ability.typ {
                    AbilityType::ChangeAttribute { typ, potency } => {
                        let entity = *children
                            .expect("Expected children on a tile")
                            .iter()
                            .next()
                            .expect("Expected an entity on a tile");

                        let mut target_query = set.p2();
                        let (name, mut attributes) = target_query
                            .get_mut(entity)
                            .expect("Didn't find target entity");

                        if let Some(attribute) = attributes.0.get_mut(&typ) {
                            match attribute {
                                Attribute::Value(v) => {
                                    *v -= potency;
                                }
                                Attribute::Gauge { value, min, max } => {
                                    *value = (*value - potency).clamp(*min, *max);

                                    ev_battle_log.send(BattleLogEvent {
                                        message: format!(
                                            "{caster_name} used {} on {}. Hp removed to: {}",
                                            ability.name, name.0, *value
                                        ),
                                    });

                                    if typ == AttributeType::HitPoints && *value <= 0 {
                                        ev_lifecycle
                                            .send(BattleLifecycleEvent::CharacterDied(entity))
                                    }
                                }
                            }
                        }
                    }
                    AbilityType::Movement => {
                        let caster_tile = set.p3().get(*by).expect("Missing player tile").get();
                        let battle_field = battle_field
                            .as_ref()
                            .expect("Missing battle field")
                            .as_ref();

                        let caster_hex = battle_field
                            .hex(caster_tile)
                            .expect("Missing hex for caster")
                            .to_oddr();
                        let target_hex = battle_field
                            .hex(*on)
                            .expect("Missing hex for target")
                            .to_oddr();

                        ev_battle_log.send(BattleLogEvent {
                            message: format!(
                                "{caster_name} moved from ({}, {}) to ({}, {})",
                                caster_hex.x, caster_hex.y, target_hex.x, target_hex.y
                            ),
                        });
                        commands.entity(*on).push_children(&[*by]);
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
