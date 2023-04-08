pub mod choose_ability_screen;

use bevy::prelude::*;
use enumset::{EnumSet, EnumSetType};

use crate::battle::lifecycle::BattleLifecycleEvent;
use crate::battle::log::BattleLogEvent;
use crate::character::{Attribute, AttributeType, Attributes, CharacterCategory, CharacterName};
use crate::AppState;

pub struct AbilityPlugin;

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AbilityCastEvent>()
            .add_system(resolve_ability.in_set(OnUpdate(AppState::Battle)));
    }
}

#[derive(Debug, Clone)]
pub struct AbilityCastEvent {
    pub ability: Ability,
    pub by: Entity,
    pub on: Vec<Entity>,
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
    mut ev_ability: EventReader<AbilityCastEvent>,
    mut ev_battle_log: EventWriter<BattleLogEvent>,
    mut ev_lifecycle: EventWriter<BattleLifecycleEvent>,
    mut set: ParamSet<(
        Query<(Entity, &CharacterName, &mut Attributes)>,
        Query<(Entity, &CharacterName, &CharacterCategory, &mut Attributes)>,
    )>,
) {
    for ability_cast in ev_ability.iter() {
        let caster_name = {
            let mut caster_name: Option<String> = None;
            for (entity, name, mut _attributes) in set.p0().iter() {
                if ability_cast.by == entity {
                    caster_name = Some(name.0.clone());
                    break;
                }
            }
            caster_name.expect("Missing caster entity")
        };

        for (entity, name, _category, mut attributes) in set.p1().iter_mut() {
            if ability_cast.on.contains(&entity) {
                match ability_cast.ability.typ {
                    AbilityType::ChangeAttribute { typ, potency } => {
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
                                            ability_cast.ability.name, name.0, *value
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
                    AbilityType::Movement => unimplemented!(),
                }
            }
        }
        ev_lifecycle.send(BattleLifecycleEvent::EndTurn)
    }
}
