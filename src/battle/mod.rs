pub mod battle_field;
pub mod enemies;
pub mod init;
pub mod interactions;
pub mod lifecycle;
pub mod log;
pub mod resolution;
pub mod ui;

use std::collections::VecDeque;

use bevy::prelude::*;
use bevy_mod_picking::{InteractablePickingPlugin, PickingPlugin};

use crate::{utils::bar::BarPlugin, AppState};

use self::{
    battle_field::*, enemies::*, init::*, interactions::*, lifecycle::*, log::*, resolution::*,
    ui::*,
};

pub struct BattlePlugin;

impl Plugin for BattlePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<BattleInitState>()
            .add_state::<BattleState>()
            .add_event::<BattleLogEvent>()
            .add_event::<BattleLifecycleEvent>()
            .add_plugin(PickingPlugin)
            .add_plugin(InteractablePickingPlugin)
            .add_plugin(BarPlugin)
            .add_systems(
                (
                    setup_battle_log,
                    setup_battle_ui,
                    initialize_enemies,
                    setup_battle_field,
                )
                    .chain()
                    .in_schedule(OnEnter(AppState::Battle)),
            )
            .add_system(setup_battle.in_schedule(OnEnter(BattleInitState::AfterBattleField)))
            .add_systems((cleanup_battle, cleanup_battle_log).in_schedule(OnExit(AppState::Battle)))
            .add_system(choose_action.in_set(OnUpdate(BattleState::AbilityChoosingPlayer)))
            .add_systems(
                (choose_target, cancel_action).in_set(OnUpdate(BattleState::AbilityTargeting)),
            )
            .add_system(init_targeting.in_schedule(OnEnter(BattleState::AbilityTargeting)))
            .add_system(cleanup_targeting.in_schedule(OnExit(BattleState::AbilityTargeting)))
            .add_system(
                setup_available_actions.in_schedule(OnEnter(BattleState::AbilityChoosingPlayer)),
            )
            .add_system(handle_enemy_turn.in_schedule(OnEnter(BattleState::AbilityCastingEnemy)))
            .add_systems(
                (
                    resize_meshes_for_sprites,
                    resize_battle_camera_viewport,
                    update_battle_log,
                    update_top_text,
                    handle_lifecycle_event,
                )
                    .in_set(OnUpdate(AppState::Battle)),
            )
            .add_system(setup_battle_resolution.in_schedule(OnEnter(BattleState::BattleEnd)))
            .add_system(
                battle_resolution_button_interaction.in_set(OnUpdate(BattleState::BattleEnd)),
            );
    }
}

#[derive(Resource, Default)]
pub struct BattleQueue {
    pub queue: VecDeque<Entity>,
}

impl BattleQueue {
    pub fn get_current(&self) -> Entity {
        *self.queue.get(0).expect("Error: turn queue is empty!")
    }
}

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum BattleInitState {
    #[default]
    BeforeBattleField,
    AfterBattleField,
}

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum BattleState {
    #[default]
    BattleInit,
    BattleEnd,
    AbilityChoosingPlayer,
    AbilityTargeting,
    AbilityCastingEnemy,
    AbilityResolution,
}

#[derive(Component)]
pub struct Battle;
