use bevy::prelude::*;

use crate::character::Group;

use super::{battle_plugin::BattleState, battle_resolution::BattleResolution};

pub enum BattleLifecycleEvent {
    CharacterDied(Entity),
}

#[derive(Component)]
pub struct Alive;

pub fn handle_lifecycle_event(
    mut commands: Commands,
    mut ev_lifecycle: EventReader<BattleLifecycleEvent>,
    query: Query<(Entity, &Group), With<Alive>>,
    mut next_state: ResMut<NextState<BattleState>>,
) {
    for lifecycle_event in ev_lifecycle.iter() {
        match lifecycle_event {
            BattleLifecycleEvent::CharacterDied(entity) => {
                if query
                    .iter()
                    .filter(|&(q_entity, group)| *group == Group::Enemy && !(q_entity == *entity))
                    .count()
                    == 0
                {
                    commands.insert_resource(BattleResolution {
                        winner: Group::Player,
                    });
                } else if query
                    .iter()
                    .filter(|&(q_entity, group)| *group == Group::Player && !(q_entity == *entity))
                    .count()
                    == 0
                {
                    commands.insert_resource(BattleResolution {
                        winner: Group::Enemy,
                    });
                }

                commands.entity(*entity).remove::<Alive>();

                next_state.set(BattleState::BattleEnd)
            }
        }
    }
}
