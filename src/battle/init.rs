use std::{collections::VecDeque, mem};

use bevy::{prelude::*, render::view::RenderLayers, sprite::Mesh2dHandle};
use bevy_mod_picking::{PickableBundle, PickableMesh};

use crate::{
    character::{AttributeType, Group},
    utils::bar::Bar,
    GameState,
};

use super::{battle_field::BattleField, lifecycle::LifeState, Battle, BattleQueue, BattleState};

pub fn get_scaling(image: Option<&Image>, tile_size: f32) -> Vec3 {
    image
        .map(|image| Vec3::splat(2.0 * tile_size / image.size().distance(Vec2::ZERO)))
        .unwrap_or(Vec3::ONE)
}

pub fn setup_battle(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    images: Res<Assets<Image>>,
    game_state: Res<GameState>,
    battle_field: Res<BattleField>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut next_state: ResMut<NextState<BattleState>>,
) {
    let mut queue = BattleQueue::default();

    let mut player_start = game_state
        .battle_field_layout
        .player_start
        .iter()
        .collect::<VecDeque<_>>();

    let mut enemy_start = game_state
        .battle_field_layout
        .enemy_start
        .iter()
        .collect::<VecDeque<_>>();

    for character in game_state.characters.iter() {
        let texture = asset_server.load(character.image_path.clone());
        let start_tiles = match character.bundle.group {
            Group::Player => &mut player_start,
            Group::Enemy => &mut enemy_start,
        };

        let tile_pos = start_tiles.pop_front().expect(&format!(
            "Too few starting positions for group {:#?}",
            character.bundle.group
        ));

        let tile = battle_field
            .tile(tile_pos)
            .expect(&format!("Missing tile {:#?}", tile_pos));

        commands.entity(tile).with_children(|parent| {
            let id = parent
                .spawn((
                    character.bundle.clone(),
                    SpriteBundle {
                        texture: texture.clone(),
                        transform: Transform::from_scale(get_scaling(
                            images.get(&texture),
                            battle_field.tile_size(),
                        )),
                        visibility: images
                            .get(&texture)
                            .is_some()
                            .then_some(Visibility::Inherited)
                            .unwrap_or(Visibility::Hidden),
                        ..default()
                    },
                    RenderLayers::layer(1),
                    Battle,
                    PickableBundle::default(),
                    Mesh2dHandle::from(
                        meshes.add(Mesh::from(shape::Quad::new(
                            images
                                .get(&texture)
                                .map(|image| image.size())
                                .unwrap_or(Vec2::ZERO),
                        ))),
                    ),
                    Bar::new(AttributeType::HitPoints),
                    LifeState::Alive,
                ))
                .id();

            queue.queue.push_back(id);
        });
    }

    commands.insert_resource(queue);

    next_state.set(BattleState::AbilityChoosingPlayer);
}

pub fn resize_meshes_for_sprites(
    images: Res<Assets<Image>>,
    mut ev_image_asset: EventReader<AssetEvent<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    battle_field: Option<Res<BattleField>>,
    mut query: Query<
        (
            &Handle<Image>,
            &mut Mesh2dHandle,
            &mut Transform,
            &mut Visibility,
        ),
        With<PickableMesh>,
    >,
) {
    for ev in ev_image_asset.iter() {
        match ev {
            AssetEvent::Created { handle } => {
                for (mut mesh, mut transform, mut visibility) in
                    query
                        .iter_mut()
                        .filter_map(|(q_handle, mesh, transform, visibility)| {
                            (q_handle == handle).then_some((mesh, transform, visibility))
                        })
                {
                    let image = images.get(&handle);

                    let new_mesh = meshes.add(Mesh::from(shape::Quad::new(image.unwrap().size())));
                    let old_mesh = mem::replace(&mut mesh.0, new_mesh);

                    meshes.remove(old_mesh);

                    *visibility = Visibility::Inherited;

                    if let Some(battle_field) = battle_field.as_ref().clone() {
                        *transform =
                            transform.with_scale(get_scaling(image, battle_field.tile_size()));
                    }
                }
            }
            _ => (),
        }
    }
}
