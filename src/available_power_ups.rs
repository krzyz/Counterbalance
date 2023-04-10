use crate::{
    abilities::choose_ability_screen::{AvailablePowerUp, PowerUp},
    available_abilities::AvailableAbilities,
    character::AttributeType,
    enemies::get_ability,
};
use bevy::{prelude::*, utils::HashMap};

#[derive(Resource)]
pub struct AvailablePowerUps(pub HashMap<String, AvailablePowerUp>);

pub fn init_available_power_ups(mut commands: Commands, abs: Res<AvailableAbilities>) {
    let power_ups = [
        AvailablePowerUp {
            name: "slam".to_string(),
            main_effect: PowerUp::Ability(get_ability("slam", &abs)),
            side_effect: PowerUp::ChangeAttribute {
                r#type: AttributeType::HitPoints,
                value: 20,
            },
        },
        AvailablePowerUp {
            name: "shoot".to_string(),
            main_effect: PowerUp::Ability(get_ability("shoot", &abs)),
            side_effect: PowerUp::ChangeAttribute {
                r#type: AttributeType::Attack,
                value: -5,
            },
        },
        AvailablePowerUp {
            name: "Raise hit points".to_string(),
            main_effect: PowerUp::ChangeAttribute {
                r#type: AttributeType::HitPoints,
                value: 10,
            },
            side_effect: PowerUp::ChangeAttribute {
                r#type: AttributeType::Defense,
                value: -1,
            },
        },
        AvailablePowerUp {
            name: "Raise attack".to_string(),
            main_effect: PowerUp::ChangeAttribute {
                r#type: AttributeType::Attack,
                value: 2,
            },
            side_effect: PowerUp::ChangeAttribute {
                r#type: AttributeType::HitPoints,
                value: -5,
            },
        },
        AvailablePowerUp {
            name: "Raise defense".to_string(),
            main_effect: PowerUp::ChangeAttribute {
                r#type: AttributeType::Defense,
                value: 2,
            },
            side_effect: PowerUp::ChangeAttribute {
                r#type: AttributeType::Attack,
                value: -1,
            },
        },
    ]
    .into_iter()
    .map(|power_up| (power_up.name.clone(), power_up))
    .collect();

    commands.insert_resource(AvailablePowerUps(power_ups));
}
