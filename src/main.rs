// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod ability;
mod battle_screen;
mod character;
mod main_menu;

use ability::AbilityPlugin;
use battle_screen::BattleScreenPlugin;
use bevy::prelude::*;
use bevy_mod_picking::PickingCameraBundle;
use main_menu::MainMenuPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_startup_system(setup)
        .add_plugin(MainMenuPlugin)
        .add_plugin(BattleScreenPlugin)
        .add_plugin(AbilityPlugin)
        .run();
}

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Battle,
}

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(PickingCameraBundle::default());
}
