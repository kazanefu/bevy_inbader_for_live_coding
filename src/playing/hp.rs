use crate::playing::CurrentScore;

use super::utils::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct HpUI;

pub fn update_player_hp(
    mut commands: Commands,
    mut ui_query: Query<&mut Text, With<HpUI>>,
    player: Query<(Entity, &HP), With<super::player::Player>>,
    mut game_state: ResMut<NextState<crate::state::GameState>>,
) {
    let mut ui = match ui_query.single_mut() {
        Ok(ui) => ui,
        Err(_) => {
            warn!("Expected exactly one HP UI entity, but found none or multiple.");
            return;
        }
    };
    let (player_entity, hp) = match player.single() {
        Ok(p) => p,
        Err(_) => {
            warn!("Expected exactly one Player entity, but found none or multiple.");
            return;
        }
    };
    **ui = format!("HP: {}", hp.0);
    if hp.0 <= 0.0 {
        commands.entity(player_entity).despawn();
        game_state.set(crate::state::GameState::Result);
    }
}

pub fn handle_enemy_death(
    mut commands: Commands,
    query: Query<(Entity, &HP, &Character)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut current_score: ResMut<CurrentScore>,
) {
    for (entity, hp, &character) in query {
        if character == Character::Player {
            continue;
        }
        if hp.0 <= 0.0 {
            commands.entity(entity).despawn();
            if character == Character::Enemy {
                current_score.0.kill += 1;
                super::enemy::spawn_random_enemy(&mut commands, &mut meshes, &mut materials);
            }
        }
    }
}
