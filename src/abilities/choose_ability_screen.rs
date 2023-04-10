use bevy::{prelude::*, utils::HashSet};
use rand::seq::IteratorRandom;

use crate::{
    available_power_ups::AvailablePowerUps,
    character::{Attribute, AttributeType},
    AppState, GameState, HOVERED_BUTTON, NORMAL_BUTTON,
};

use super::Ability;

#[derive(Component)]
pub struct AbilityScreen;

#[derive(Component)]
pub struct PowerUpToChoose(String);

#[derive(Debug)]
pub enum PowerUp {
    Ability(Ability),
    ChangeAttribute { r#type: AttributeType, value: i32 },
}

#[derive(Debug)]
pub struct AvailablePowerUp {
    pub name: String,
    pub main_effect: PowerUp,
    pub side_effect: PowerUp,
}

pub fn interact_pick_power_up(
    available_power_ups: Res<AvailablePowerUps>,
    mut game_state: ResMut<GameState>,
    mut interaction_query: Query<
        (&Interaction, &PowerUpToChoose, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, button, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
            Interaction::Clicked => {
                let player = game_state
                    .characters
                    .get_mut(0)
                    .expect("Missing player character!");

                let chosen = available_power_ups.0.get(&button.0).unwrap();
                for power_up in [&chosen.main_effect, &chosen.side_effect] {
                    match power_up {
                        PowerUp::Ability(ability) => {
                            player
                                .bundle
                                .abilities
                                .0
                                .insert(ability.name.clone(), ability.clone());
                        }
                        PowerUp::ChangeAttribute { r#type, value } => {
                            let change_value = value;
                            let attr = player
                                .bundle
                                .attributes
                                .0
                                .get_mut(r#type)
                                .expect("Missing attribute to change");

                            match attr {
                                Attribute::Value(v) => *v += change_value,
                                Attribute::Gauge { value, max, .. } => {
                                    *value += change_value;
                                    *max += change_value;
                                }
                            }
                        }
                    }
                }
                next_state.set(AppState::Battle);
            }
        }
    }
}

pub fn setup_ability_screen(
    mut commands: Commands,
    game_state: Res<GameState>,
    available_power_ups: Res<AvailablePowerUps>,
    asset_server: Res<AssetServer>,
) {
    let mut rng = rand::thread_rng();

    let player = game_state
        .characters
        .get(0)
        .expect("Missing player character!");

    let player_abilities = player
        .bundle
        .abilities
        .0
        .iter()
        .map(|(name, _)| name)
        .collect::<HashSet<_>>();

    let mut ability_already = false;

    let available_power_ups = available_power_ups
        .0
        .iter()
        .filter(|(_, av_power_up)| match &av_power_up.main_effect {
            PowerUp::Ability(ab) => {
                let ok = !ability_already && !player_abilities.contains(&ab.name);
                if ok {
                    ability_already = true;
                }
                ok
            }
            _ => true,
        })
        .choose_multiple(&mut rng, 2);

    let button_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Medium.ttf"),
        font_size: 50.0,
        color: Color::WHITE,
    };

    let text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Medium.ttf"),
        font_size: 20.0,
        color: Color::WHITE,
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::width(Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            AbilityScreen,
        ))
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Pick a power up!",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                        font_size: 70.0,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(20.)),
                    ..default()
                }),
            );

            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::all(Val::Percent(80.0)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: Color::rgb(0.0, 0.0, 0.0).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::AUTO,
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Start,
                                justify_content: JustifyContent::SpaceEvenly,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            for &(power_up_name, power_up) in available_power_ups.iter() {
                                parent
                                    .spawn(NodeBundle {
                                        style: Style {
                                            size: Size::AUTO,
                                            flex_direction: FlexDirection::Column,
                                            align_items: AlignItems::Center,
                                            justify_content: JustifyContent::SpaceEvenly,
                                            margin: UiRect::all(Val::Px(20.0)),
                                            ..default()
                                        },
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        parent
                                            .spawn((
                                                ButtonBundle {
                                                    style: Style {
                                                        size: Size::new(
                                                            Val::Px(400.0),
                                                            Val::Px(65.0),
                                                        ),
                                                        // horizontally center child text
                                                        justify_content: JustifyContent::Center,
                                                        // vertically center child text
                                                        align_items: AlignItems::Center,
                                                        margin: UiRect::all(Val::Px(5.0)),
                                                        padding: UiRect::all(Val::Px(5.0)),
                                                        ..default()
                                                    },
                                                    background_color: NORMAL_BUTTON.into(),
                                                    ..default()
                                                },
                                                PowerUpToChoose(power_up_name.clone()),
                                            ))
                                            .with_children(|parent| {
                                                parent.spawn(TextBundle::from_section(
                                                    power_up_name.as_str(),
                                                    button_style.clone(),
                                                ));
                                            });
                                        parent.spawn(
                                            TextBundle::from_section(
                                                format!("Main effect: {:#?}", power_up.main_effect),
                                                text_style.clone(),
                                            )
                                            .with_style(Style {
                                                flex_wrap: FlexWrap::Wrap,
                                                padding: UiRect::all(Val::Px(5.0)),
                                                ..default()
                                            }),
                                        );
                                        parent.spawn(
                                            TextBundle::from_section(
                                                format!("Side effect: {:#?}", power_up.side_effect),
                                                text_style.clone(),
                                            )
                                            .with_style(Style {
                                                flex_wrap: FlexWrap::Wrap,
                                                padding: UiRect::all(Val::Px(5.0)),
                                                ..default()
                                            }),
                                        );
                                    });
                            }
                        });
                });
        });
}

pub fn cleanup_ability_screen(mut commands: Commands, query: Query<Entity, With<AbilityScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
