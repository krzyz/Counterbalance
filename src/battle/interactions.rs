use crate::{
    abilities::{Ability, AbilityCastEvent, AbilityTargetType},
    character::{Abilities, Group},
    GameState, HOVERED_BUTTON, NORMAL_BUTTON,
};
use bevy::prelude::*;
use rand::seq::IteratorRandom;

use super::{
    battle_field::{BattleField, Tile},
    Battle, BattleQueue, BattleState,
};

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
    query_groups: Query<(&Group, &Parent)>,
    mut ev_ability: EventWriter<AbilityCastEvent>,
) {
    let mut rng = rand::thread_rng();

    let parent = query_groups
        .iter()
        .filter_map(|(group, parent)| (*group == Group::Player).then_some(parent))
        .choose(&mut rng)
        .expect("Couldn't find a player character");

    let target = parent.get();

    let active_entity = res_queue.get_current();

    if let Ok(abilities) = query_abilities.get(active_entity) {
        if let Some(ability) = abilities
            .0
            .iter()
            .choose(&mut rng)
            .map(|(_, ability)| ability)
        {
            ev_ability.send(AbilityCastEvent {
                ability: ability.clone(),
                by: active_entity,
                on: target,
            });
        }
    }
}

pub fn choose_target(
    mut commands: Commands,
    res_queue: ResMut<BattleQueue>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    res_ability: Option<Res<ChosenAbility>>,
    mut ev_ability: EventWriter<AbilityCastEvent>,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut Handle<ColorMaterial>,
            Option<&Children>,
        ),
        (Changed<Interaction>, Without<Button>, With<Tile>),
    >,
    group_query: Query<&Group, (Without<Button>, Without<Tile>)>,
    mut ability_buttons_query: Query<&mut BackgroundColor, (With<AbilityButton>, With<Button>)>,
) {
    for (entity, interaction, mut color_handle, children) in interaction_query.iter_mut() {
        let target_type = children
            .and_then(|children| {
                children.iter().next().and_then(|child| {
                    match *group_query
                        .get(*child)
                        .expect("Missing group for a character on a tile")
                    {
                        Group::Player => Some(AbilityTargetType::Ally),
                        Group::Enemy => Some(AbilityTargetType::Enemy),
                    }
                })
            })
            .unwrap_or(AbilityTargetType::Empty);

        if let Some(ability) = res_ability.as_ref().map(|ca| &ca.ability) {
            let is_valid = ability.target.contains(target_type);

            match (*interaction, is_valid) {
                (Interaction::Hovered, true) => {
                    *color_handle = materials.add(ColorMaterial::from(Color::LIME_GREEN));
                }
                (Interaction::Hovered, false) => {
                    *color_handle = materials.add(ColorMaterial::from(Color::RED));
                }
                (Interaction::Clicked, true) => {
                    ev_ability.send(AbilityCastEvent {
                        ability: ability.clone(),
                        by: res_queue.get_current(),
                        on: entity,
                    });

                    commands.remove_resource::<ChosenAbility>();

                    for mut color in &mut ability_buttons_query {
                        *color = NORMAL_BUTTON.into();
                    }

                    *color_handle = materials.add(ColorMaterial::from(Color::GRAY));
                }
                (Interaction::Clicked, false) => (),
                (Interaction::None, _) => {
                    *color_handle = materials.add(ColorMaterial::from(Color::GRAY));
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
