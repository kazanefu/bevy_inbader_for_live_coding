use bevy::prelude::*;

use crate::playing::player;

#[derive(Component)]
pub struct HPUi;

pub fn check_player_hp(
    mut commands: Commands,
    mut ui_query: Query<&mut Text, With<HPUi>>,
    player: Query<(Entity, &super::player::HP), With<super::player::Player>>,
    mut game_state: ResMut<NextState<crate::state::GameState>>,
) {
    let mut ui = match ui_query.single_mut() {
        Ok(ui) => ui,
        Err(_) => {
            println!("more than 1 HP UI exist");
            return;
        }
    };
    let (player_entity, hp) = match player.single() {
        Ok(p) => p,
        Err(_) => {
            println!("more than 1 player exist");
            return;
        }
    };
    **ui = format!("HP: {}", hp.0);
    if hp.0 <= 0.0 {
        commands.entity(player_entity).despawn();
        game_state.set(crate::state::GameState::Result);
    }
}

pub fn check_hp(mut commands: Commands, query: Query<(Entity, &super::player::HP)>) {
    for (entity, hp) in query {
        if hp.0 <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
