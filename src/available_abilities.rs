use crate::{
    abilities::{Ability, AbilityType},
    character::AttributeType,
    InitState,
};
use bevy::{prelude::*, utils::HashMap};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AbilityTier {
    //Normal,
    //SideEffect,
}

#[derive(Resource)]
pub struct AvailableAbilities(pub HashMap<String, Ability>);

pub fn init_available_abilities(
    mut commands: Commands,
    mut next_state: ResMut<NextState<InitState>>,
) {
    let abilities = vec![
        Ability {
            name: "hit".to_string(),
            typ: AbilityType::ChangeAttribute(AttributeType::HitPoints),
            potency: 15,
            side_effect: None,
        },
        Ability {
            name: "slam".to_string(),
            typ: AbilityType::ChangeAttribute(AttributeType::HitPoints),
            potency: 30,
            side_effect: None,
        },
        Ability {
            name: "weaken defense".to_string(),
            typ: AbilityType::ChangeAttribute(AttributeType::Defense),
            potency: -1,
            side_effect: None,
        },
        Ability {
            name: "weaken attack".to_string(),
            typ: AbilityType::ChangeAttribute(AttributeType::Attack),
            potency: -1,
            side_effect: None,
        },
    ]
    .into_iter()
    .map(|ability| (ability.name.clone(), ability.clone()))
    .collect();

    commands.insert_resource(AvailableAbilities(abilities));
    next_state.set(InitState::AfterAbilities)
}
