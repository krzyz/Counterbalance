use crate::{
    abilities::{Ability, AbilityTargetType, AbilityType},
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
            name: "move".to_string(),
            typ: AbilityType::Movement,
            target: AbilityTargetType::Empty.into(),
            range: 5,
            side_effect: None,
        },
        Ability {
            name: "hit".to_string(),
            typ: AbilityType::ChangeAttribute {
                typ: AttributeType::HitPoints,
                potency: 15,
            },
            target: AbilityTargetType::Enemy.into(),
            range: 2,
            side_effect: None,
        },
        Ability {
            name: "slam".to_string(),
            typ: AbilityType::ChangeAttribute {
                typ: AttributeType::HitPoints,
                potency: 20,
            },
            target: AbilityTargetType::Enemy.into(),
            range: 1,
            side_effect: None,
        },
    ]
    .into_iter()
    .map(|ability| (ability.name.clone(), ability.clone()))
    .collect();

    commands.insert_resource(AvailableAbilities(abilities));
    next_state.set(InitState::AfterAbilities)
}
