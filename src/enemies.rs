use bevy::{prelude::*, utils::HashMap};

use crate::{
    abilities::Ability,
    available_abilities::AvailableAbilities,
    character::{Abilities, AttributeType, Character, CharacterBundle, CharacterCategory, Group},
    GameState,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EnemyTier {
    Normal1,
    //Normal2,
    //Boss1,
}

#[derive(Resource, Debug)]
pub struct AvailableEnemies(pub HashMap<EnemyTier, Vec<Character>>);

pub fn get_abilities(
    names: &[&str],
    available_abilities: &Res<AvailableAbilities>,
) -> Vec<Ability> {
    names
        .iter()
        .map(|name| name.to_string())
        .map(|name| {
            available_abilities
                .0
                .get(&name)
                .expect("Can't find the ability during init!")
                .clone()
        })
        .collect()
}

pub fn init_available_enemies(
    mut commands: Commands,
    abs: Res<AvailableAbilities>,
    mut game_state: ResMut<GameState>,
) {
    let player = game_state.characters.get_mut(0).unwrap();

    player.bundle.abilities = Abilities::from_arr(get_abilities(&["move", "hit"], &abs).as_ref());

    info!("Starting enemies init");
    use AttributeType::*;
    use CharacterCategory::*;

    let mut enemies = HashMap::new();

    enemies.insert(
        EnemyTier::Normal1,
        [
            Character::new(
                CharacterBundle::new(
                    "fungus",
                    Fungus,
                    get_abilities(&["hit"], &abs).as_ref(),
                    &[(HitPoints, 50), (Attack, 5), (Defense, 5)],
                    Group::Enemy,
                ),
                "images/fungus.png",
            ),
            Character::new(
                CharacterBundle::new(
                    "angry fungus",
                    Fungus,
                    get_abilities(&["slam"], &abs).as_ref(),
                    &[(HitPoints, 50), (Attack, 7), (Defense, 3)],
                    Group::Enemy,
                ),
                "images/fungus.png",
            ),
        ]
        .into_iter()
        .collect(),
    );

    commands.insert_resource(AvailableEnemies(enemies));
}
