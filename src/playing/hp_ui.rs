use crate::playing::CurrentScore;

use super::util::*;
use bevy::prelude::*;
use rand::prelude::*;

#[derive(Component)]
pub struct HPUi;

pub fn check_player_hp(
    mut commands: Commands,
    mut ui_query: Query<&mut Text, With<HPUi>>,
    player: Query<(Entity, &HP), With<super::player::Player>>,
    mut game_state: ResMut<NextState<crate::state::GameState>>,
) {
    let mut ui = match ui_query.single_mut() {
        Ok(ui) => ui,
        Err(_) => {
            println!("0 or more than 1 HP UI exist");
            return;
        }
    };
    let (player_entity, hp) = match player.single() {
        Ok(p) => p,
        Err(_) => {
            println!("0 or more than 1 player exist");
            return;
        }
    };
    **ui = format!("HP: {}", hp.0);
    if hp.0 <= 0.0 {
        commands.entity(player_entity).despawn();
        game_state.set(crate::state::GameState::Result);
    }
}

pub fn check_hp(
    mut commands: Commands,
    query: Query<(Entity, &HP, &Character), Without<Dead>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut current_score: ResMut<CurrentScore>,
) {
    for (entity, hp, &character) in query {
        if character == Character::Player {
            continue;
        }
        if hp.0 <= 0.0 {
            commands.entity(entity).insert(Dead);
            if character == Character::Enemy {
                current_score.0.kill += 1;
                let mut rng = rand::rng();
                let x = rng.random_range(-9..=9);
                super::enemy::spawn_enemy(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    Vec3 {
                        x: x as f32,
                        y: 0.0,
                        z: 10.0,
                    },
                );
            }
        }
    }
}
