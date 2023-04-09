pub mod choose_ability_screen;

use bevy::prelude::*;
use enumset::{EnumSet, EnumSetType};

use crate::battle::battle_field::Tile;
use crate::battle::lifecycle::BattleLifecycleEvent;
use crate::battle::log::BattleLogEvent;
use crate::character::{Attribute, AttributeType, Attributes, CharacterName};
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
    pub on: Entity,
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
    mut ev_ability: EventReader<AbilityCastEvent>,
    mut ev_battle_log: EventWriter<BattleLogEvent>,
    mut ev_lifecycle: EventWriter<BattleLifecycleEvent>,
    mut set: ParamSet<(
        Query<Option<&Children>, With<Tile>>,
        Query<(&CharacterName, &mut Attributes)>,
        Query<(&CharacterName, &mut Attributes)>,
        //Query<&mut Parent, With<CharacterName>>,
    )>,
) {
    for ability_cast in ev_ability.iter() {
        let caster_name = set
            .p1()
            .get(ability_cast.by)
            .expect("Missing caster entity")
            .0
             .0
            .clone();

        let tile_query = set.p0();
        let children = tile_query
            .get(ability_cast.on)
            .expect("Missing ability target");

        match ability_cast.ability.typ {
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
                                    ability_cast.ability.name, name.0, *value
                                ),
                            });

                            if typ == AttributeType::HitPoints && *value <= 0 {
                                ev_lifecycle.send(BattleLifecycleEvent::CharacterDied(entity))
                            }
                        }
                    }
                }
            }
            AbilityType::Movement => {
                commands
                    .entity(ability_cast.on)
                    .push_children(&[ability_cast.by]);
                /*
                let mut parent_query = set.p3();

                let mut parent = parent_query
                    .get_mut(ability_cast.by)
                    .expect("Missing caster's parent");

                let parent_tile = parent.get();

                *parent = Parent(ability_cast.on);
                */
            }
        }
        ev_lifecycle.send(BattleLifecycleEvent::EndTurn)
    }
}
