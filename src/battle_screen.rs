use crate::{
    ability::{Ability, AbilityCastEvent},
    character::{spawn_character, Abilities, CharacterCategory, Group},
    utils::bar::{spawn_bar, Bar, BarPlugin},
    AppState,
};
use bevy::{ecs::query::ReadOnlyWorldQuery, prelude::*, sprite::Mesh2dHandle};
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle, PickableMesh};
use rand::seq::IteratorRandom;

use std::{collections::VecDeque, mem};

const NORMAL_ABILITY_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_ABILITY_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);

pub struct BattleScreenPlugin;

impl Plugin for BattleScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<BattleState>()
            .add_event::<BattleLogEvent>()
            .add_plugins(DefaultPickingPlugins)
            .add_plugin(BarPlugin)
            .add_systems(
                (setup_battle_ui, setup_battle)
                    .chain()
                    .in_schedule(OnEnter(AppState::Battle)),
            )
            .add_system(cleanup_battle.in_schedule(OnExit(AppState::Battle)))
            .add_system(choose_action.in_set(OnUpdate(BattleState::AbilityChoosingPlayer)))
            .add_system(choose_target.in_set(OnUpdate(BattleState::AbilityTargeting)))
            .add_system(
                setup_available_actions.in_schedule(OnEnter(BattleState::AbilityChoosingPlayer)),
            )
            .add_system(handle_enemy_turn.in_schedule(OnEnter(BattleState::AbilityCastingEnemy)))
            .add_systems(
                (
                    resize_meshes_for_sprites,
                    update_battle_log,
                    update_top_text,
                )
                    .in_set(OnUpdate(AppState::Battle)),
            );
    }
}

#[derive(Resource)]
pub struct BattleQueue {
    queue: VecDeque<Entity>,
}

impl BattleQueue {
    fn get_current(&self) -> Entity {
        *self.queue.get(0).expect("Error: turn queue is empty!")
    }
}

#[derive(Resource)]
pub struct ChosenAbility {
    ability: Ability,
}

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum BattleState {
    #[default]
    BattleInit,
    AbilityChoosingPlayer,
    AbilityTargeting,
    AbilityCastingEnemy,
    AbilityResolution,
}

#[derive(Component)]
pub struct Battle;

#[derive(Component)]
pub struct BattleLogText;

#[derive(Component)]
struct TopText;

#[derive(Resource)]
pub struct BattleLog {
    messages: Vec<String>,
}

pub struct BattleLogEvent {
    pub message: String,
}

fn get_abilities<'q, 'w, 's, F>(
    entity: Entity,
    query: &'q Query<'w, 's, (Entity, &Abilities), F>,
) -> &'q Abilities
where
    F: ReadOnlyWorldQuery,
{
    query
        .iter()
        .find_map(|(q_entity, abilities)| (q_entity == entity).then_some(abilities))
        .expect("Missing entity whose turn it is now!")
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

#[derive(Component)]
struct AbilityButton {
    ability_name: String,
}

fn handle_enemy_turn(
    mut res_queue: ResMut<BattleQueue>,
    mut set: ParamSet<(Query<(Entity, &Abilities)>, Query<(Entity, &Group)>)>,
    mut ev_ability: EventWriter<AbilityCastEvent>,
    mut next_state: ResMut<NextState<BattleState>>,
) {
    let mut rng = rand::thread_rng();

    let target = set
        .p1()
        .iter()
        .filter_map(|(entity, group)| (*group == Group::Player).then_some(entity))
        .choose(&mut rng);

    let active_entity = res_queue.get_current();

    if let Some(ability) = get_abilities(active_entity, &set.p0())
        .0
        .iter()
        .choose(&mut rng)
    {
        if let Some(target) = target {
            ev_ability.send(AbilityCastEvent {
                ability: ability.clone(),
                by: active_entity,
                on: vec![target],
            });
        }
    }

    res_queue.queue.rotate_left(1);
    next_state.set(BattleState::AbilityChoosingPlayer)
}

fn update_top_text(state: Res<State<BattleState>>, mut query: Query<&mut Text, With<TopText>>) {
    if state.is_changed() {
        let update_text = match state.0 {
            BattleState::BattleInit => "",
            BattleState::AbilityChoosingPlayer => "Select an ability",
            BattleState::AbilityTargeting => "Select a target",
            BattleState::AbilityCastingEnemy => "Enemy's turn",
            BattleState::AbilityResolution => "Resolving an ability",
        };

        for mut text in &mut query {
            if let Some(section) = text.sections.first_mut() {
                section.value = update_text.into();
            }
        }
    }
}

fn choose_target(
    mut commands: Commands,
    mut res_queue: ResMut<BattleQueue>,
    res_ability: Res<ChosenAbility>,
    mut ev_ability: EventWriter<AbilityCastEvent>,
    mut interaction_query: Query<
        (Entity, &Interaction, &Group, &mut Sprite),
        (Changed<Interaction>, (Without<Button>, Without<Bar>)),
    >,
    mut ability_buttons_query: Query<&mut BackgroundColor, (With<AbilityButton>, With<Button>)>,
    mut bar_query: Query<&mut Bar, Without<Button>>,
    mut next_state: ResMut<NextState<BattleState>>,
) {
    for (entity, interaction, group, mut sprite) in interaction_query.iter_mut() {
        if *group == Group::Enemy {
            match *interaction {
                Interaction::Hovered => sprite.color = Color::rgba(0.7, 1.0, 0.7, 1.0),
                Interaction::Clicked => {
                    sprite.color = Color::default();

                    ev_ability.send(AbilityCastEvent {
                        ability: res_ability.ability.clone(),
                        by: res_queue.get_current(),
                        on: vec![entity],
                    });

                    commands.remove_resource::<ChosenAbility>();

                    res_queue.queue.rotate_left(1);

                    for mut color in &mut ability_buttons_query {
                        *color = NORMAL_ABILITY_BUTTON.into();
                    }

                    for mut bar in &mut bar_query {
                        let value = bar.value();
                        bar.set_value(value - res_ability.ability.potency as f32);
                    }

                    next_state.set(BattleState::AbilityCastingEnemy);

                    break;
                }
                Interaction::None => {
                    sprite.color = Color::default();
                }
            }
        }
    }
}

fn choose_action(
    mut commands: Commands,
    res_queue: Res<BattleQueue>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &AbilityButton),
        (Changed<Interaction>, With<Button>),
    >,
    abilities_query: Query<(Entity, &Abilities), Without<Button>>,
    mut next_state: ResMut<NextState<BattleState>>,
) {
    for (interaction, mut color, ability_button) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                *color = HOVERED_ABILITY_BUTTON.into();
            }
            Interaction::Clicked => {
                let abilities = get_abilities(res_queue.get_current(), &abilities_query);

                let chosen_ability = abilities
                    .0
                    .iter()
                    .find(|ability| ability_button.ability_name == ability.name)
                    .expect("Ability chosen can't be found for the current active entity");

                commands.insert_resource(ChosenAbility {
                    ability: chosen_ability.clone(),
                });

                next_state.set(BattleState::AbilityTargeting);

                break;
            }
            Interaction::None => {
                *color = NORMAL_ABILITY_BUTTON.into();
            }
        }
    }
}

#[derive(Component)]
struct AvailableActionsNode;

fn setup_available_actions(
    mut commands: Commands,
    res_queue: Res<BattleQueue>,
    asset_server: Res<AssetServer>,
    mut query_node: Query<Entity, With<AvailableActionsNode>>,
    query_abilities: Query<(Entity, &Abilities), Without<AvailableActionsNode>>,
) {
    let caster = res_queue.get_current();

    let abilities = get_abilities(caster, &query_abilities);

    for entity in query_node.iter_mut() {
        commands.entity(entity).despawn_descendants();

        let children = abilities
            .0
            .iter()
            .map(|ability| {
                commands
                    .spawn(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(250.0), Val::Px(65.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: NORMAL_ABILITY_BUTTON.into(),
                        ..default()
                    })
                    .insert(AbilityButton {
                        ability_name: ability.name.clone(),
                    })
                    .with_children(|parent| {
                        parent.spawn(
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
                        );
                    })
                    .id()
            })
            .collect::<Vec<_>>();

        commands.entity(entity).push_children(children.as_ref());
    }
}

pub fn setup_battle(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    images: Res<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut next_state: ResMut<NextState<BattleState>>,
) {
    let player_character = spawn_character(
        &mut commands,
        "player",
        CharacterCategory::Human,
        Group::Player,
    );
    commands
        .entity(player_character)
        .insert(Battle)
        .insert(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-20.0, 20.0, 0.0),
                ..default()
            },
            texture: asset_server.load("images/human.png"),
            ..default()
        });

    let bar = spawn_bar(&mut commands);
    commands.entity(bar).insert(Bar::new(0.0, 50.0, 50.0));

    let enemy_character = spawn_character(
        &mut commands,
        "fungus",
        CharacterCategory::Fungi,
        Group::Enemy,
    );

    let enemy_image = asset_server.load("images/fungus.png");
    commands
        .entity(enemy_character)
        .insert(Battle)
        .insert((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(320.0, 20.0, 0.0),
                    ..default()
                },
                texture: enemy_image.clone(),
                ..default()
            },
            PickableBundle::default(),
        ))
        .insert(Mesh2dHandle::from(
            meshes.add(Mesh::from(shape::Quad::new(
                images
                    .get(&enemy_image)
                    .map(|image| image.size())
                    .unwrap_or(Vec2::ZERO),
            ))),
        ));

    commands.insert_resource(BattleQueue {
        queue: [player_character, enemy_character].into(),
    });

    next_state.set(BattleState::AbilityChoosingPlayer);
}

fn resize_meshes_for_sprites(
    images: Res<Assets<Image>>,
    mut ev_image_asset: EventReader<AssetEvent<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(&Handle<Image>, &mut Mesh2dHandle), With<PickableMesh>>,
) {
    for ev in ev_image_asset.iter() {
        match ev {
            AssetEvent::Created { handle } => {
                if let Some(mut mesh) = query
                    .iter_mut()
                    .find_map(|(q_handle, mesh)| (q_handle == handle).then_some(mesh))
                {
                    let size = images
                        .get(&handle)
                        .expect("Should have gotten new image, asset event lied")
                        .size();

                    let new_mesh = meshes.add(Mesh::from(shape::Quad::new(size)));
                    let old_mesh = mem::replace(&mut mesh.0, new_mesh);

                    meshes.remove(old_mesh);
                }
            }
            _ => (),
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

fn build_bottom_pane(parent: &mut ChildBuilder) {
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
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::AUTO,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(TextBundle::from_section(
                                    "",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                                        font_size: 50.0,
                                        color: Color::WHITE,
                                    },
                                ))
                                .insert(TopText);
                        });
                    build_bottom_pane(parent);
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
