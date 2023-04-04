use crate::{AppState, HOVERED_BUTTON, NORMAL_BUTTON};
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(cleanup_menu.in_schedule(OnExit(AppState::MainMenu)))
            .add_systems(
                (main_menu_button_looks, main_menu_button_clicks)
                    .in_set(OnUpdate(AppState::MainMenu)),
            );
    }
}

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub enum MainMenuButton {
    StartGame,
}

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .insert(MainMenu)
        .with_children(|parent| {
            // main window
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
                .insert(MainMenuButton::StartGame)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Start Game",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

pub fn main_menu_button_clicks(
    mut interaction_query: Query<
        (&Interaction, &MainMenuButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, button) in &mut interaction_query {
        if let Interaction::Clicked = *interaction {
            match *button {
                MainMenuButton::StartGame => next_state.set(AppState::Battle),
            }
        }
    }
}

pub fn main_menu_button_looks(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
            _ => (),
        }
    }
}

pub fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
