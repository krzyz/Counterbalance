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
use battle::BattlePlugin;
use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use bevy_prototype_lyon::prelude::*;
use character::{
    Abilities, Attributes, Character, CharacterBundle, CharacterCategory, CharacterName, Group,
};
use enemies::init_available_enemies;
use main_menu::MainMenuPlugin;

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);

pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .init_resource::<GameState>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Counterbalance".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
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

#[derive(Component)]
pub struct MainCamera;

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
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
            },
            ..default()
        },
        MainCamera,
    ));
}
