use bevy::prelude::*;
use rand::seq::IteratorRandom;

use crate::{
    enemies::{AvailableEnemies, EnemyTier},
    GameState,
};

pub fn initialize_enemies(enemies: Res<AvailableEnemies>, mut game_state: ResMut<GameState>) {
    let mut rng = rand::thread_rng();

    for enemy in enemies
        .0
        .get(&EnemyTier::Normal1)
        .expect("Missing normal enemies!")
        .into_iter()
        .choose_multiple(&mut rng, 1)
        .into_iter()
    {
        game_state.characters.push(enemy.clone());
    }
}
