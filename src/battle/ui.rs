use std::f32::consts::PI;

use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{
        camera::{ScalingMode, Viewport},
        view::RenderLayers,
    },
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    utils::HashMap,
    window::WindowResized,
};
use bevy_mod_picking::{PickableBundle, PickingCameraBundle};

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

#[derive(Component)]
pub struct Tile;

#[derive(Resource)]
pub struct BattleField {
    tiles: HashMap<IVec2, Entity>,
    rev_map: HashMap<Entity, IVec2>,
    size: IVec2,
}

impl BattleField {
    pub fn new(tiles: HashMap<IVec2, Entity>) -> Self {
        let rev_map = tiles.iter().map(|(pos, entity)| (*entity, *pos)).collect();
        let size = (
            tiles.iter().map(|(p, _)| p.x).max().unwrap_or(0),
            tiles.iter().map(|(p, _)| p.y).max().unwrap_or(0),
        )
            .into();
        BattleField {
            tiles,
            rev_map,
            size,
        }
    }
    pub fn tile(&self, pos: &IVec2) -> Option<Entity> {
        self.tiles.get(&pos).copied()
    }
    pub fn pos(&self, entity: Entity) -> Option<IVec2> {
        self.rev_map.get(&entity).copied()
    }
}

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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = windows.single();

    let min_world_size = Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: 1,
                viewport: Some(get_viewport(
                    window.resolution.width() as f32,
                    window.resolution.height() as f32,
                    window.scale_factor(),
                )),
                ..default()
            },
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::AutoMin {
                    min_width: min_world_size.x,
                    min_height: min_world_size.y,
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

    setup_battle_field(&mut commands, &mut meshes, &mut materials, min_world_size);
}

pub fn setup_battle_field(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    min_world_size: Vec2,
) {
    let size = IVec2::new(18, 8);

    let bottom_left_corner = Vec2::ZERO - 0.5 * min_world_size;
    info!("world size: {min_world_size:?}");

    let tile_size = (min_world_size.x / (size.x as f32 * 1.5 + 0.5))
        .min(min_world_size.y / (3.0f32.sqrt() * (size.y as f32 + 0.5)));

    info!("tile size: {tile_size:?}");
    let hor_spacing = 1.5 * tile_size;
    let ver_spacing = 3.0f32.sqrt() * tile_size;
    let corner_pos = bottom_left_corner
        + 0.5 * Vec2::new(2.0 * tile_size, ver_spacing)
        + 0.5
            * Vec2::new(
                min_world_size.x - tile_size * (1.5 * size.x as f32 + 0.5),
                min_world_size.y - tile_size * 3.0f32.sqrt() * (size.y as f32 + 0.5),
            );

    let mut tiles = HashMap::new();
    let tile_mesh: Mesh2dHandle = meshes
        .add(Mesh::from(shape::RegularPolygon::new(tile_size - 1.0, 6)))
        .into();
    let tile_material = materials.add(ColorMaterial::from(Color::GRAY));

    for x in 0..size.x {
        for y in 0..size.y {
            let transform = Transform::from_xyz(
                corner_pos.x + x as f32 * hor_spacing,
                corner_pos.y + (y as f32 + 0.5 * (x % 2) as f32) * ver_spacing,
                0.0,
            )
            .with_rotation(Quat::from_rotation_z(0.5 * PI));

            let id = commands
                .spawn((
                    Tile,
                    MaterialMesh2dBundle {
                        mesh: tile_mesh.clone(),
                        transform,
                        material: tile_material.clone(),
                        ..default()
                    },
                    Battle,
                    RenderLayers::layer(1),
                    PickableBundle::default(),
                ))
                .id();

            tiles.insert((x, y).into(), id);
        }
    }

    commands.insert_resource(BattleField::new(tiles));
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
