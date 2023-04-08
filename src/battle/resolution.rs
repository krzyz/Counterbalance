use bevy::prelude::*;

use crate::{character::Group, AppState, HOVERED_BUTTON, NORMAL_BUTTON};

use super::Battle;

#[derive(Resource)]
pub struct BattleResolution {
    pub winner: Group,
}

#[derive(Component)]
pub enum BattleResolutionButton {
    MainMenu,
    Continue,
}

pub fn battle_resolution_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &BattleResolutionButton, &mut BackgroundColor),
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
            Interaction::Clicked => match button {
                BattleResolutionButton::MainMenu => next_state.set(AppState::MainMenu),
                BattleResolutionButton::Continue => next_state.set(AppState::AbilityChoose),
            },
        }
    }
}

pub fn setup_battle_resolution(
    mut commands: Commands,
    res_resolution: Res<BattleResolution>,
    asset_server: Res<AssetServer>,
) {
    let text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Medium.ttf"),
        font_size: 50.0,
        color: Color::WHITE,
    };
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    position_type: PositionType::Absolute,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            Battle,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::AUTO,
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    z_index: ZIndex::Global(1),
                    ..default()
                })
                .with_children(|parent| match res_resolution.winner {
                    Group::Player => {
                        parent.spawn(TextBundle::from_section("Victory!", text_style.clone()));
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        size: Size::AUTO,
                                        // horizontally center child text
                                        justify_content: JustifyContent::Center,
                                        // vertically center child text
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                BattleResolutionButton::Continue,
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    "Continue",
                                    text_style.clone(),
                                ));
                            });
                    }
                    Group::Enemy => {
                        parent.spawn(TextBundle::from_section("You lost!", text_style.clone()));
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        size: Size::AUTO,
                                        // horizontally center child text
                                        justify_content: JustifyContent::Center,
                                        // vertically center child text
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                BattleResolutionButton::MainMenu,
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    "Back to main menu",
                                    text_style.clone(),
                                ));
                            });
                    }
                });
        });
}
