use crate::{
    abilities::{Ability, AbilityCastEvent},
    character::{Abilities, Group},
    GameState, HOVERED_BUTTON, NORMAL_BUTTON,
};
use bevy::{prelude::*, sprite::Mesh2dHandle};
use bevy_mod_picking::PickableMesh;
use rand::seq::IteratorRandom;

use std::mem;

use super::{ui::Tile, Battle, BattleQueue, BattleState};

#[derive(Component)]
pub struct AbilityButton {
    ability_name: String,
}

#[derive(Resource)]
pub struct ChosenAbility {
    ability: Ability,
}

pub fn handle_enemy_turn(
    res_queue: ResMut<BattleQueue>,
    query_abilities: Query<&Abilities>,
    query_groups: Query<(Entity, &Group)>,
    mut ev_ability: EventWriter<AbilityCastEvent>,
) {
    let mut rng = rand::thread_rng();

    let target = query_groups
        .iter()
        .filter_map(|(entity, group)| (*group == Group::Player).then_some(entity))
        .choose(&mut rng);

    let active_entity = res_queue.get_current();

    if let Ok(abilities) = query_abilities.get(active_entity) {
        if let Some(ability) = abilities
            .0
            .iter()
            .choose(&mut rng)
            .map(|(_, ability)| ability)
        {
            if let Some(target) = target {
                ev_ability.send(AbilityCastEvent {
                    ability: ability.clone(),
                    by: active_entity,
                    on: vec![target],
                });
            }
        }
    }
}

pub fn highlight_tile(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (Changed<Interaction>, With<Tile>),
    >,
) {
    for (interaction, mut material) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Hovered => {
                *material = materials.add(ColorMaterial::from(Color::LIME_GREEN))
            }
            Interaction::None => *material = materials.add(ColorMaterial::from(Color::GRAY)),
            _ => (),
        }
    }
}

pub fn choose_target(
    mut commands: Commands,
    res_queue: ResMut<BattleQueue>,
    res_ability: Option<Res<ChosenAbility>>,
    mut ev_ability: EventWriter<AbilityCastEvent>,
    mut interaction_query: Query<
        (Entity, &Interaction, &Group, &mut Sprite),
        (Changed<Interaction>, Without<Button>),
    >,
    mut ability_buttons_query: Query<&mut BackgroundColor, (With<AbilityButton>, With<Button>)>,
) {
    for (entity, interaction, group, mut sprite) in interaction_query.iter_mut() {
        if *group == Group::Enemy {
            match *interaction {
                Interaction::Hovered => sprite.color = Color::rgba(0.7, 1.0, 0.7, 1.0),
                Interaction::Clicked => {
                    sprite.color = Color::default();

                    if let Some(ability) = res_ability.as_ref().map(|r| &r.ability) {
                        ev_ability.send(AbilityCastEvent {
                            ability: ability.clone(),
                            by: res_queue.get_current(),
                            on: vec![entity],
                        });

                        commands.remove_resource::<ChosenAbility>();

                        for mut color in &mut ability_buttons_query {
                            *color = NORMAL_BUTTON.into();
                        }
                    }

                    break;
                }
                Interaction::None => {
                    sprite.color = Color::default();
                }
            }
        }
    }
}

pub fn choose_action(
    mut commands: Commands,
    res_queue: Res<BattleQueue>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &AbilityButton),
        (Changed<Interaction>, With<Button>),
    >,
    abilities_query: Query<&Abilities, Without<Button>>,
    mut next_state: ResMut<NextState<BattleState>>,
) {
    for (interaction, mut color, ability_button) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::Clicked => {
                //let abilities = get_abilities(res_queue.get_current(), &abilities_query);
                let abilities = abilities_query
                    .get(res_queue.get_current())
                    .expect("Can't find abilities for active entity!");

                let chosen_ability = abilities
                    .0
                    .get(&ability_button.ability_name)
                    .expect("Ability chosen can't be found for the current active entity");

                commands.insert_resource(ChosenAbility {
                    ability: chosen_ability.clone(),
                });

                next_state.set(BattleState::AbilityTargeting);

                break;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

#[derive(Component)]
pub struct AvailableActionsNode;

pub fn setup_available_actions(
    mut commands: Commands,
    res_queue: Res<BattleQueue>,
    asset_server: Res<AssetServer>,
    mut query_node: Query<Entity, With<AvailableActionsNode>>,
    query_abilities: Query<&Abilities, Without<AvailableActionsNode>>,
) {
    let caster = res_queue.get_current();

    let abilities = query_abilities
        .get(caster)
        .expect("Couldn't find abilities of current active entity");

    for entity in query_node.iter_mut() {
        commands.entity(entity).despawn_descendants();

        let children = abilities
            .0
            .iter()
            .map(|(name, _)| {
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
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    })
                    .insert(AbilityButton {
                        ability_name: name.clone(),
                    })
                    .with_children(|parent| {
                        parent.spawn(
                            TextBundle::from_section(
                                name.clone(),
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

pub fn resize_meshes_for_sprites(
    images: Res<Assets<Image>>,
    mut ev_image_asset: EventReader<AssetEvent<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(&Handle<Image>, &mut Mesh2dHandle), With<PickableMesh>>,
) {
    for ev in ev_image_asset.iter() {
        match ev {
            AssetEvent::Created { handle } => {
                for mut mesh in query
                    .iter_mut()
                    .filter_map(|(q_handle, mesh)| (q_handle == handle).then_some(mesh))
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

pub fn cleanup_battle(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    query: Query<Entity, With<Battle>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    game_state
        .characters
        .retain(|char| char.bundle.group == Group::Player);
}
