use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{
        camera::{ScalingMode, Viewport},
        view::RenderLayers,
    },
    window::WindowResized,
};
use bevy_mod_picking::PickingCameraBundle;

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

use super::{
    log::BattleLogText,
    {AvailableActionsNode, Battle, BattleState},
};

const BOTTOM_PANE_HEIGHT: f32 = 150.0;
const RIGHT_PANE_WIDTH: f32 = 200.0;

#[derive(Component)]
pub struct TopText;

#[derive(Component)]
pub struct BattleCamera;

pub fn update_top_text(state: Res<State<BattleState>>, mut query: Query<&mut Text, With<TopText>>) {
    if state.is_changed() {
        let update_text = match state.0 {
            BattleState::BattleInit => "",
            BattleState::BattleEnd => "",
            BattleState::AbilityChoosingPlayer => "Select an ability",
            BattleState::AbilityTargeting => "Select a target (Esc or RMB to cancel)",
            BattleState::AbilityCastingEnemy => "Enemy's turn",
            BattleState::AbilityResolution => "Resolving an ability",
        };

        for mut text in &mut query {
            if let Some(section) = text.sections.first_mut() {
                section.value = update_text.into();
            }
        }
    }
}

pub fn build_right_pane(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                size: Size::width(Val::Px(RIGHT_PANE_WIDTH)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: Color::rgb(0.65, 0.65, 0.65).into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::width(Val::Percent(100.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // text
                    parent
                        .spawn(
                            TextBundle::from_section(
                                "Battle Log",
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
                        )
                        .insert(BattleLogText);
                });
        });
}

pub fn build_bottom_pane(parent: &mut ChildBuilder) {
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                size: Size::new(Val::Percent(100.0), Val::Px(BOTTOM_PANE_HEIGHT)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: Color::rgb(0.65, 0.65, 0.65).into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::height(Val::Percent(100.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .insert(AvailableActionsNode);
        });
}

fn get_viewport(window_width: f32, window_height: f32, scale_factor: f64) -> Viewport {
    let x =
        (scale_factor as f32 * (window_width - RIGHT_PANE_WIDTH)).clamp(0.0, f32::INFINITY) as u32;
    let y = (scale_factor as f32 * (window_height - BOTTOM_PANE_HEIGHT)).clamp(0.0, f32::INFINITY)
        as u32;

    info!("Getting viewport: {x}, {y} from {window_width}, {window_height}, {scale_factor}");

    Viewport {
        physical_position: UVec2::new(0, 0),
        physical_size: UVec2::new(x, y),
        ..default()
    }
}

pub fn setup_battle_ui(
    mut commands: Commands,
    windows: Query<&Window>,
    asset_server: Res<AssetServer>,
) {
    let window = windows.single();

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: 1,
                viewport: Some(get_viewport(
                    window.resolution.width(),
                    window.resolution.height(),
                    window.scale_factor(),
                )),
                ..default()
            },
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::AutoMin {
                    min_width: WINDOW_WIDTH,
                    min_height: WINDOW_HEIGHT,
                },
                ..default()
            },
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
            },
            ..default()
        },
        RenderLayers::layer(1),
        UiCameraConfig { show_ui: false },
        PickingCameraBundle::default(),
        Battle,
        BattleCamera,
    ));

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::width(Val::Percent(100.0)),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ..default()
            },
            Battle,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::AUTO,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(TextBundle::from_section(
                                    "",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                                        font_size: 50.0,
                                        color: Color::WHITE,
                                    },
                                ))
                                .insert(TopText);
                        });
                    build_bottom_pane(parent);
                });
            build_right_pane(parent, &asset_server);
        });
}

pub fn resize_battle_camera_viewport(
    mut camera_query: Query<&mut Camera, With<BattleCamera>>,
    windows: Query<&Window>,
    mut resize_reader: EventReader<WindowResized>,
) {
    let mut camera = camera_query.single_mut();
    let window = windows.single();

    for e in resize_reader.iter() {
        camera.viewport = Some(get_viewport(e.width, e.height, window.scale_factor()));
    }
}
