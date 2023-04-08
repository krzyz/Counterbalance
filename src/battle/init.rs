use bevy::{prelude::*, render::view::RenderLayers, sprite::Mesh2dHandle};
use bevy_mod_picking::PickableBundle;

use crate::{character::AttributeType, utils::bar::Bar, GameState};

use super::{lifecycle::LifeState, Battle, BattleQueue, BattleState};

pub fn setup_battle(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    images: Res<Assets<Image>>,
    game_state: Res<GameState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut next_state: ResMut<NextState<BattleState>>,
) {
    let mut queue = BattleQueue::default();

    for (i, character) in game_state.characters.iter().enumerate() {
        let texture = asset_server.load(character.image_path.clone());
        let id = commands
            .spawn((
                character.bundle.clone(),
                SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(-300.0 + (200 * i) as f32, 20.0, 0.0),
                        ..default()
                    },
                    texture: texture.clone(),
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
    }

    commands.insert_resource(queue);

    next_state.set(BattleState::AbilityChoosingPlayer);
}
