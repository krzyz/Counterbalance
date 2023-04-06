// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod abilities;
mod available_abilities;
mod battle;
mod character;
mod enemies;
mod main_menu;
mod utils;

use abilities::AbilityPlugin;
use available_abilities::init_available_abilities;
use battle::battle_plugin::BattlePlugin;
use bevy::prelude::*;
use bevy_mod_picking::PickingCameraBundle;
use bevy_prototype_lyon::prelude::*;
use character::{
    Abilities, Attributes, Character, CharacterBundle, CharacterCategory, CharacterName, Group,
};
use enemies::init_available_enemies;
use main_menu::MainMenuPlugin;

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .init_resource::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_state::<InitState>()
        .add_startup_systems((setup, init_available_abilities))
        .add_system(init_available_enemies.in_schedule(OnEnter(InitState::AfterAbilities)))
        .add_plugin(ShapePlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(BattlePlugin)
        .add_plugin(AbilityPlugin)
        .run();
}

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum InitState {
    #[default]
    BeforeAbilities,
    AfterAbilities,
}

#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Battle,
    AbilityChoose,
}

#[derive(Resource, Debug, Clone)]
pub struct GameState {
    characters: Vec<Character>,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            characters: vec![Character {
                bundle: CharacterBundle {
                    name: CharacterName("player".to_string()),
                    category: CharacterCategory::Human,
                    abilities: Abilities::default(),
                    attributes: Attributes::default(),
                    group: Group::Player,
                },
                image_path: "images/human.png".to_string(),
            }],
        }
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(PickingCameraBundle::default());
}
