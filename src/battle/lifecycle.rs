use bevy::prelude::*;

use crate::character::{CharacterName, Group};

use super::{
    log::BattleLogEvent,
    resolution::BattleResolution,
    {BattleQueue, BattleState},
};

pub enum BattleLifecycleEvent {
    CharacterDied(Entity),
    EndTurn,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifeState {
    Alive,
    Dead,
}

pub fn handle_lifecycle_event(
    mut commands: Commands,
    mut res_queue: Option<ResMut<BattleQueue>>,
    mut ev_lifecycle: EventReader<BattleLifecycleEvent>,
    mut ev_battle_log: EventWriter<BattleLogEvent>,
    mut group_alive_query: Query<(&mut LifeState, &Group)>,
    mut vis_query: Query<(&mut Visibility, &CharacterName)>,
    mut next_state: ResMut<NextState<BattleState>>,
) {
    for lifecycle_event in ev_lifecycle.iter() {
        match lifecycle_event {
            BattleLifecycleEvent::CharacterDied(entity) => {
                let (mut life_state, group) = group_alive_query.get_mut(*entity).unwrap();
                *life_state = LifeState::Dead;
                if *group == Group::Enemy {
                    if let Ok((mut visibility, name)) = vis_query.get_mut(*entity) {
                        *visibility = Visibility::Hidden;
                        ev_battle_log.send(BattleLogEvent {
                            message: format!("{} defeated!", name.0),
                        })
                    }
                }
            }
            BattleLifecycleEvent::EndTurn => {
                if group_alive_query
                    .iter()
                    .filter(|&(life_state, group)| {
                        *group == Group::Enemy && *life_state == LifeState::Alive
                    })
                    .count()
                    == 0
                {
                    commands.insert_resource(BattleResolution {
                        winner: Group::Player,
                    });
                    next_state.set(BattleState::BattleEnd)
                } else if group_alive_query
                    .iter()
                    .filter(|&(life_state, group)| {
                        *group == Group::Player && *life_state == LifeState::Alive
                    })
                    .count()
                    == 0
                {
                    commands.insert_resource(BattleResolution {
                        winner: Group::Enemy,
                    });
                    next_state.set(BattleState::BattleEnd)
                } else {
                    let res_queue = res_queue
                        .as_mut()
                        .expect("Turn ended before battle queue got initialized");
                    res_queue.queue.rotate_left(1);
                    let active_entity = res_queue.get_current();
                    let (_, group) = group_alive_query
                        .get(active_entity)
                        .expect("Current character missing group");

                    match group {
                        Group::Player => next_state.set(BattleState::AbilityChoosingPlayer),
                        Group::Enemy => next_state.set(BattleState::AbilityCastingEnemy),
                    }
                }
            }
        }
    }
}
