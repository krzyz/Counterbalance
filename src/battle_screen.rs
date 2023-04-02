use crate::{
    character::{spawn_character, Abilities, CharacterCategory, Group},
    AppState,
};
use bevy::prelude::*;

pub struct BattleScreenPlugin;

impl Plugin for BattleScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<BattleState>()
            .add_event::<BattleLogEvent>()
            .add_systems(
                (setup_battle_ui, setup_battle)
                    .chain()
                    .in_schedule(OnEnter(AppState::Battle)),
            )
            .add_system(cleanup_battle.in_schedule(OnExit(AppState::Battle)))
            .add_system(choose_action.in_set(OnUpdate(BattleState::AbilityCastingPlayer)))
            .add_system(
                setup_available_actions.in_schedule(OnEnter(BattleState::AbilityCastingPlayer)),
            )
            .add_system(update_battle_log.in_set(OnUpdate(AppState::Battle)));
    }
}

#[derive(Resource)]
pub struct BattleTurn {
    turn: Entity,
}

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum BattleState {
    #[default]
    BattleInit,
    AbilityCastingPlayer,
    AbilityCastingEnemy,
    AbilityResolution,
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

fn choose_action(mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>) {
    for interaction in &mut interaction_query {
        if let Interaction::Clicked = *interaction {}
    }
}

#[derive(Component)]
struct AvailableActionsNode;

fn setup_available_actions(
    mut commands: Commands,
    res_turn: Res<BattleTurn>,
    asset_server: Res<AssetServer>,
    mut query_node: Query<Entity, With<AvailableActionsNode>>,
    query_abilities: Query<(Entity, &Abilities), Without<AvailableActionsNode>>,
) {
    let abilities = query_abilities
        .iter()
        .find_map(|(entity, abilities)| (entity == res_turn.turn).then_some(abilities))
        .expect("Missing entity whose turn it is now!");

    for entity in query_node.iter_mut() {
        commands.entity(entity).despawn_descendants();

        let children = abilities
            .0
            .iter()
            .map(|ability| {
                commands
                    .spawn(
                        TextBundle::from_section(
                            ability.name.clone(),
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
                    .id()
            })
            .collect::<Vec<_>>();

        commands.entity(entity).push_children(children.as_ref());
    }
}

pub fn setup_battle(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<BattleState>>,
) {
    let player_character = spawn_character(
        &mut commands,
        "player",
        CharacterCategory::Human,
        Group::Player,
    );
    commands.entity(player_character).insert(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(-20.0, 20.0, 0.0),
            ..default()
        },
        texture: asset_server.load("images/human.png"),
        ..default()
    });

    let enemy_character = spawn_character(
        &mut commands,
        "fungus",
        CharacterCategory::Fungi,
        Group::Enemy,
    );

    commands.entity(enemy_character).insert(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(320.0, 20.0, 0.0),
            ..default()
        },
        texture: asset_server.load("images/fungus.png"),
        ..default()
    });

    commands.insert_resource(BattleTurn {
        turn: player_character,
    });

    next_state.set(BattleState::AbilityCastingPlayer);
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
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::height(Val::Percent(100.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .insert(AvailableActionsNode);
        });
}

pub fn setup_battle_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
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
