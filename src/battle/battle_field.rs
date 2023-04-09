use bevy::{
    prelude::*,
    render::view::RenderLayers,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    utils::HashMap,
};
use bevy_mod_picking::PickableBundle;

use crate::{battle::Battle, GameState, WINDOW_HEIGHT, WINDOW_WIDTH};

use super::BattleInitState;

#[derive(Component)]
pub struct Tile;

#[derive(Debug, Clone)]
pub struct BattleFieldLayout {
    pub size: UVec2,
    pub player_start: Vec<UVec2>,
    pub enemy_start: Vec<UVec2>,
}

#[derive(Resource, Debug, Clone)]
pub struct BattleField {
    tiles: HashMap<UVec2, Entity>,
    rev_map: HashMap<Entity, UVec2>,
    tile_size: f32,
}

impl BattleField {
    pub fn new(tiles: HashMap<UVec2, Entity>, tile_size: f32) -> Self {
        let rev_map = tiles.iter().map(|(pos, entity)| (*entity, *pos)).collect();

        BattleField {
            tiles,
            rev_map,
            tile_size,
        }
    }

    pub fn tile(&self, pos: &UVec2) -> Option<Entity> {
        self.tiles.get(&pos).copied()
    }

    pub fn pos(&self, entity: Entity) -> Option<UVec2> {
        self.rev_map.get(&entity).copied()
    }

    pub fn tile_size(&self) -> f32 {
        self.tile_size
    }
}

pub fn setup_battle_field(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_state: Res<GameState>,
    mut next_state: ResMut<NextState<BattleInitState>>,
) {
    let size = game_state.battle_field_layout.size;

    let world_size = Vec2::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    let bottom_left_corner = Vec2::ZERO - 0.5 * world_size;

    let tile_size = (world_size.x / (3.0f32.sqrt() * (size.x as f32 + 0.5)))
        .min(world_size.y / (size.y as f32 * 1.5 + 0.5));

    let hor_spacing = 3.0f32.sqrt() * tile_size;
    let ver_spacing = 1.5 * tile_size;
    let corner_pos = bottom_left_corner
        + 0.5 * Vec2::new(2.0 * tile_size, ver_spacing)
        + 0.5
            * Vec2::new(
                world_size.x - tile_size * 3.0f32.sqrt() * (size.x as f32 + 0.5),
                world_size.y - tile_size * (1.5 * size.y as f32 + 0.5),
            );

    let mut tiles = HashMap::new();
    let tile_mesh: Mesh2dHandle = meshes
        .add(Mesh::from(shape::RegularPolygon::new(tile_size - 1.0, 6)))
        .into();
    let tile_material = materials.add(ColorMaterial::from(Color::GRAY));

    for x in 0..size.x {
        for y in 0..size.y {
            let transform = Transform::from_xyz(
                corner_pos.x + (x as f32 + 0.5 * (y % 2) as f32) * hor_spacing,
                corner_pos.y + y as f32 * ver_spacing,
                0.0,
            );

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

    commands.insert_resource(BattleField::new(tiles, tile_size));

    next_state.set(BattleInitState::AfterBattleField);
}
