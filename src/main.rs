// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod ability;
mod battle;
mod character;
mod main_menu;
mod utils;

use ability::AbilityPlugin;
use battle::battle_plugin::BattlePlugin;
use bevy::prelude::*;
use bevy_mod_picking::PickingCameraBundle;
use bevy_prototype_lyon::prelude::*;
use main_menu::MainMenuPlugin;

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_startup_system(setup)
        .add_plugin(ShapePlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(BattlePlugin)
        .add_plugin(AbilityPlugin)
        .run();
}

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Battle,
    AbilityChoose,
}

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(PickingCameraBundle::default());
}
