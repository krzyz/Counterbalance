use crate::AppState;
use bevy::prelude::*;

pub struct BattleScreenPlugin;

impl Plugin for BattleScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BattleLogEvent>()
            .add_system(setup_battle.in_schedule(OnEnter(AppState::Battle)))
            .add_system(cleanup_battle.in_schedule(OnExit(AppState::Battle)))
            .add_system(update_battle_log.in_set(OnUpdate(AppState::Battle)));
    }
}

#[derive(Component)]
pub struct Battle;

#[derive(Component)]
pub struct BattleLogText;

#[derive(Resource)]
pub struct BattleLog {
    messages: Vec<String>,
}

pub struct BattleLogEvent {
    pub message: String,
}

fn update_battle_log(
    asset_server: Res<AssetServer>,
    mut battle_log: ResMut<BattleLog>,
    mut ev_battle_log: EventReader<BattleLogEvent>,
    mut query: Query<&mut Text, With<BattleLogText>>,
) {
    for log_event in ev_battle_log.iter() {
        battle_log.messages.push(log_event.message.clone());

        for mut text in &mut query {
            text.sections.push(TextSection::new(
                format!("\n{}", log_event.message),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                    font_size: 10.0,
                    color: Color::WHITE,
                },
            ))
        }
    }
}

fn build_right_pane(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                size: Size::width(Val::Px(200.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: Color::rgb(0.65, 0.65, 0.65).into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::width(Val::Percent(100.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // text
                    parent
                        .spawn(
                            TextBundle::from_section(
                                "Battle Log",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                            )
                            .with_style(Style {
                                margin: UiRect::all(Val::Px(5.0)),
                                ..default()
                            }),
                        )
                        .insert(BattleLogText);
                });
        });
}

fn build_bottom_pane(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                size: Size::new(Val::Percent(100.0), Val::Px(150.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: Color::rgb(0.65, 0.65, 0.65).into(),
            ..default()
        })
        .with_children(|parent| {
            // right vertical fill (content)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::height(Val::Percent(100.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // text
                    parent.spawn(
                        TextBundle::from_section(
                            "Text example",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                                font_size: 30.0,
                                color: Color::WHITE,
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(5.0)),
                            ..default()
                        }),
                    );
                });
        });
}

pub fn setup_battle(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(BattleLog {
        messages: Vec::new(),
    });

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .insert(Battle)
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(NodeBundle {
                        style: Style {
                            size: Size::AUTO,
                            ..default()
                        },
                        ..default()
                    });
                    build_bottom_pane(parent, &asset_server);
                });
            build_right_pane(parent, &asset_server);
        });
}

pub fn cleanup_battle(mut commands: Commands, query: Query<Entity, With<Battle>>) {
    commands.remove_resource::<BattleLog>();

    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
