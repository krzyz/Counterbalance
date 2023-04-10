use bevy::{
    prelude::*,
    render::view::RenderLayers,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    utils::HashMap,
};
use bevy_mod_picking::PickableBundle;

use crate::{battle::Battle, utils::hex::Hex, GameState, WINDOW_HEIGHT, WINDOW_WIDTH};

use super::BattleInitState;

#[derive(Component)]
pub struct Tile;

#[derive(Debug, Clone)]
pub struct BattleFieldLayout {
    pub size: UVec2,
    pub player_start: Vec<Hex>,
    pub enemy_start: Vec<Hex>,
}

#[derive(Resource, Debug, Clone)]
pub struct BattleField {
    tiles: HashMap<Hex, Entity>,
    rev_map: HashMap<Entity, Hex>,
    tile_size: f32,
}

impl BattleField {
    pub fn new(tiles: HashMap<Hex, Entity>, tile_size: f32) -> Self {
        let rev_map = tiles.iter().map(|(pos, entity)| (*entity, *pos)).collect();

        BattleField {
            tiles,
            rev_map,
            tile_size,
        }
    }

    pub fn tile(&self, pos: &Hex) -> Option<Entity> {
        self.tiles.get(pos).copied()
    }

    pub fn tiles(&self) -> &HashMap<Hex, Entity> {
        &self.tiles
    }

    pub fn hex(&self, entity: Entity) -> Option<Hex> {
        self.rev_map.get(&entity).copied()
    }

    pub fn tile_size(&self) -> f32 {
        self.tile_size
    }

    pub fn hexes_by_dist(&self, pos: &Hex, close_to: Option<Hex>) -> Vec<(i32, Hex)> {
        let mut dists_to_hex = self
            .tiles
            .iter()
            .map(|(h, _)| (pos.dist(*h), close_to.map(|pos2| pos2.dist(*h)), *h))
            .collect::<Vec<_>>();

        dists_to_hex.sort_by_key(|&(dist1, dist2, _)| (dist1, dist2));
        dists_to_hex
            .into_iter()
            .map(|(dist, _, hex)| (dist, hex))
            .collect()
    }

    pub fn in_range_and_empty(
        &self,
        from: Hex,
        hex: Hex,
        range: i32,
        tile_children_query: &Query<&Children, With<Tile>>,
    ) -> Option<Entity> {
        let tile = self.tile(&hex).expect("Couldn't find tile");
        let tile_has_children = tile_children_query.get(tile).is_ok();
        (from.dist(hex) <= range && !tile_has_children).then_some(tile)
    }

    pub fn get_in_range_and_empty(
        &self,
        target_hex: Hex,
        caster_hex: Hex,
        range: i32,
        tile_children_query: &Query<&Children, With<Tile>>,
    ) -> Option<Entity> {
        self.hexes_by_dist(&target_hex, Some(caster_hex))
            .iter()
            .find_map(|(_, hex)| {
                self.in_range_and_empty(caster_hex, *hex, range, tile_children_query)
            })
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

            tiles.insert(Hex::from_oddr((x as i32, y as i32).into()), id);
        }
    }

    commands.insert_resource(BattleField::new(tiles, tile_size));

    next_state.set(BattleInitState::AfterBattleField);
}
