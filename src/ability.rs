use bevy::prelude::*;

use crate::battle_screen::BattleLogEvent;
use crate::character::{
    AttributeClass, AttributeType, Attributes, CharacterCategory, CharacterName,
};
use crate::AppState;

pub struct AbilityPlugin;

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AbilityCastEvent>()
            .add_system(cast_ability.in_set(OnUpdate(AppState::Battle)));
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
    pub potency: i32,
    pub side_effect: Option<Box<Ability>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbilityType {
    ChangeAttribute(AttributeType),
}

fn cast_ability(
    mut ev_ability: EventReader<AbilityCastEvent>,
    mut ev_battle_log: EventWriter<BattleLogEvent>,
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
                                        message: format!(
                                            "{caster_name} used {} on {}. Hp removed to: {}",
                                            ability_cast.ability.name, name.0, *value
                                        ),
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
