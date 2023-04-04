use bevy::prelude::*;

#[derive(Component)]
pub struct BattleLogText;

#[derive(Resource)]
pub struct BattleLog {
    messages: Vec<String>,
}

pub struct BattleLogEvent {
    pub message: String,
}

pub fn update_battle_log(
    asset_server: Res<AssetServer>,
    mut battle_log: ResMut<BattleLog>,
    mut ev_battle_log: EventReader<BattleLogEvent>,
    mut query: Query<&mut Text, With<BattleLogText>>,
) {
    for log_event in ev_battle_log.iter() {
        battle_log.messages.push(log_event.message.clone());

        for mut text in &mut query {
            text.sections.push(TextSection::new(
                format!("\n{}", log_event.message),
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                    font_size: 10.0,
                    color: Color::WHITE,
                },
            ))
        }
    }
}

pub fn setup_battle_log(mut commands: Commands) {
    commands.insert_resource(BattleLog {
        messages: Vec::new(),
    });
}

pub fn cleanup_battle_log(mut commands: Commands) {
    commands.remove_resource::<BattleLog>();
}
