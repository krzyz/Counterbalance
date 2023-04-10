use bevy::prelude::*;
use rand::seq::IteratorRandom;

use crate::{
    abilities::{AbilityTargetType, TurnEvent},
    character::{Abilities, Attribute, Group},
    enemies::{AvailableEnemies, EnemyTier},
    GameState,
};

use super::{
    battle_field::{BattleField, Tile},
    BattleQueue,
};

pub fn initialize_enemies(enemies: Res<AvailableEnemies>, mut game_state: ResMut<GameState>) {
    let mut rng = rand::thread_rng();

    let number_of_enemies = (game_state.round / 8 + 1).clamp(1, 4);

    for enemy in enemies
        .0
        .get(&EnemyTier::Normal1)
        .expect("Missing normal enemies!")
        .iter()
        .choose_multiple(&mut rng, number_of_enemies as usize)
        .into_iter()
    {
        let mut enemy = enemy.clone();

        let power_multiplier = 1.02f32.powi(game_state.round);

        for (_, val) in enemy.bundle.attributes.0.iter_mut() {
            match val {
                Attribute::Value(v) => {
                    *v = (*v as f32 * power_multiplier).round() as i32;
                }
                Attribute::Gauge { value, max, .. } => {
                    *value = (*value as f32 * power_multiplier).round() as i32;
                    *max = (*max as f32 * power_multiplier).round() as i32;
                }
            };
        }

        game_state.characters.push(enemy.clone());
    }
}

pub fn handle_enemy_turn(
    res_queue: ResMut<BattleQueue>,
    battle_field: Option<Res<BattleField>>,
    abilities_query: Query<&Abilities>,
    group_parent_query: Query<(&Group, &Parent)>,
    tile_children_query: Query<&Children, With<Tile>>,
    mut ev_ability: EventWriter<TurnEvent>,
) {
    let mut rng = rand::thread_rng();

    let player_tile = group_parent_query
        .iter()
        .filter_map(|(group, parent)| (*group == Group::Player).then_some(parent))
        .choose(&mut rng)
        .expect("Couldn't find a player character")
        .get();

    let active_enemy = res_queue.get_current();
    let enemy_tile = group_parent_query
        .get(active_enemy)
        .expect("Couldn't get enemy parent tile")
        .1
        .get();

    let battle_field = battle_field.expect("Missing battle field");
    let player_hex = battle_field.hex(player_tile).expect("Missing player hex");
    let enemy_hex = battle_field.hex(enemy_tile).expect("Missing enemy hex");

    if let Ok(abilities) = abilities_query.get(active_enemy) {
        if let Some(ability) = abilities
            .0
            .iter()
            .map(|(_, ability)| ability)
            .filter(|&ability| {
                let enemy_targeting: bool = ability.target.contains(AbilityTargetType::Enemy);
                let player_in_range = player_hex.dist(enemy_hex) <= ability.range;
                enemy_targeting && player_in_range
            })
            .choose(&mut rng)
        {
            ev_ability.send(TurnEvent::Ability {
                ability: ability.clone(),
                by: active_enemy,
                on: player_tile,
            });
        } else if let Some((ability, target_hex)) = {
            let mut move_abilities = abilities
                .0
                .iter()
                .map(|(_, ability)| ability)
                .filter(|&ability| ability.target.contains(AbilityTargetType::Empty))
                .collect::<Vec<_>>();

            move_abilities.sort_by_key(|ab| ab.range);
            move_abilities.last().and_then(|&ability| {
                battle_field
                    .hexes_by_dist(&player_hex, Some(enemy_hex))
                    .iter()
                    .find_map(|(_, move_target)| {
                        move_target.line(enemy_hex).iter().skip(1).find_map(|&h| {
                            battle_field
                                .in_range_and_empty(
                                    enemy_hex,
                                    h,
                                    ability.range,
                                    &tile_children_query,
                                )
                                .map(|_| (ability, h))
                        })
                    })
            })
        } {
            let target_tile = battle_field.tile(&target_hex).expect("Couldn't find tile");

            ev_ability.send(TurnEvent::Ability {
                ability: ability.clone(),
                by: active_enemy,
                on: target_tile,
            });
        } else {
            ev_ability.send(TurnEvent::Pass(active_enemy));
        }
    } else {
        warn!("Missing abilities of the current active enemy! Skipping turn");
        ev_ability.send(TurnEvent::Pass(active_enemy));
    }
}
