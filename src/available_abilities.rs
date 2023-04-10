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
            r#type: AbilityType::Movement,
            target: AbilityTargetType::Empty.into(),
            range: 5,
        },
        Ability {
            name: "hit".to_string(),
            r#type: AbilityType::Targeted {
                ab_type: TargetedAbilityType::ChangeAttribute {
                    at_type: AttributeType::HitPoints,
                    potency: 15,
                },
                proximity: AbilityProximity::Melee,
            },
            target: AbilityTargetType::Enemy.into(),
            range: 2,
        },
        Ability {
            name: "shoot".to_string(),
            r#type: AbilityType::Targeted {
                ab_type: TargetedAbilityType::ChangeAttribute {
                    at_type: AttributeType::HitPoints,
                    potency: 10,
                },
                proximity: AbilityProximity::Ranged,
            },
            target: AbilityTargetType::Enemy.into(),
            range: 5,
        },
        Ability {
            name: "slam".to_string(),
            r#type: AbilityType::Targeted {
                ab_type: TargetedAbilityType::ChangeAttribute {
                    at_type: AttributeType::HitPoints,
                    potency: 20,
                },
                proximity: AbilityProximity::Ranged,
            },
            target: AbilityTargetType::Enemy.into(),
            range: 1,
        },
    ]
    .into_iter()
    .map(|ability| (ability.name.clone(), ability))
    .collect();

    commands.insert_resource(AvailableAbilities(abilities));
    next_state.set(InitState::AfterAbilities)
}
