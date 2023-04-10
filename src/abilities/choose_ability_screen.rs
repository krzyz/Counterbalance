use bevy::prelude::*;

#[derive(Component)]
pub struct AbilityScreen;

pub fn setup_ability_screen(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        },
        AbilityScreen,
    ));
}

pub fn cleanup_ability_screen(mut commands: Commands, query: Query<Entity, With<AbilityScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
