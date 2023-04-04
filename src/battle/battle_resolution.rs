use bevy::prelude::*;

use crate::{character::Group, NORMAL_BUTTON};

#[derive(Resource)]
pub struct BattleResolution {
    pub winner: Group,
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
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(400.0), Val::Px(500.0)),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .with_children(|parent| match res_resolution.winner {
                    Group::Player => {
                        parent.spawn(TextBundle::from_section("Victory!", text_style.clone()));
                        parent
                            .spawn(ButtonBundle {
                                style: Style {
                                    size: Size::new(Val::Px(250.0), Val::Px(65.0)),
                                    // horizontally center child text
                                    justify_content: JustifyContent::Center,
                                    // vertically center child text
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    "Continue!",
                                    text_style.clone(),
                                ));
                            });
                    }
                    Group::Enemy => {
                        parent.spawn(TextBundle::from_section("You lost!", text_style.clone()));
                        parent
                            .spawn(ButtonBundle {
                                style: Style {
                                    size: Size::new(Val::Px(250.0), Val::Px(65.0)),
                                    // horizontally center child text
                                    justify_content: JustifyContent::Center,
                                    // vertically center child text
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            })
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
