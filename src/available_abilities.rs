use crate::{
    abilities::{Ability, AbilityProximity, AbilityTargetType, AbilityType, TargetedAbilityType},
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
            typ: AbilityType::Targeted {
                ab_typ: TargetedAbilityType::ChangeAttribute {
                    at_typ: AttributeType::HitPoints,
                    potency: 15,
                },
                proximity: AbilityProximity::Melee,
            },
            target: AbilityTargetType::Enemy.into(),
            range: 2,
            side_effect: None,
        },
        Ability {
            name: "shoot".to_string(),
            typ: AbilityType::Targeted {
                ab_typ: TargetedAbilityType::ChangeAttribute {
                    at_typ: AttributeType::HitPoints,
                    potency: 1,
                },
                proximity: AbilityProximity::Ranged,
            },
            target: AbilityTargetType::Enemy.into(),
            range: 5,
            side_effect: None,
        },
        Ability {
            name: "slam".to_string(),
            typ: AbilityType::Targeted {
                ab_typ: TargetedAbilityType::ChangeAttribute {
                    at_typ: AttributeType::HitPoints,
                    potency: 20,
                },
                proximity: AbilityProximity::Ranged,
            },
            target: AbilityTargetType::Enemy.into(),
            range: 1,
            side_effect: None,
        },
    ]
    .into_iter()
    .map(|ability| (ability.name.clone(), ability))
    .collect();

    commands.insert_resource(AvailableAbilities(abilities));
    next_state.set(InitState::AfterAbilities)
}
