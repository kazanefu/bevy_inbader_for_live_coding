use core::f32;

use bevy::prelude::*;

use super::bullet;
use super::util::*;

const PLAYER_FORCE: f32 = 3.0;
const PLAYER_SPEED_LIMIT: f32 = 10.0;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(crate::state::GameState::OnGame), player_spawn)
            .add_systems(
                Update,
                (move_player, shoot).run_if(
                    in_state(crate::state::GameState::OnGame)
                        .and(in_state(super::OnGameState::Running)),
                ),
            );
    }
}

#[derive(Component)]
pub struct Player;

pub fn player_spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        DespawnOnExit(crate::state::GameState::OnGame),
        Character::Player,
        Player,
        HP(100.0),
        Transform::from_xyz(0.0, 0.0, -8.0).looking_at(Vec3::ZERO, Vec3::Y),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::BLACK,
            emissive: Color::srgb(0.1, 0.1, 1.0).to_linear(),
            ..default()
        })),
        Control {
            speed_limit: PLAYER_SPEED_LIMIT,
            mass: 1.0,
            ..default()
        },
    ));
}

fn move_player(mut query: Query<&mut Control, With<Player>>, keyboard: Res<ButtonInput<KeyCode>>) {
    let mut control = match query.single_mut() {
        Ok(control) => control,
        Err(_) => {
            println!("player exist more than one");
            return;
        }
    };
    if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
        control.add_force(Vec3 {
            x: PLAYER_FORCE,
            y: 0.0,
            z: 0.0,
        });
    }
    if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
        control.add_force(Vec3 {
            x: -PLAYER_FORCE,
            y: 0.0,
            z: 0.0,
        });
    }
}

pub fn shoot(
    mut commands: Commands,
    query: Query<(&Transform, &Character), With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (transform, &owner) in query {
        if keyboard.just_pressed(KeyCode::Space) {
            bullet::spawn_bullet(
                &mut commands,
                owner,
                transform.translation,
                transform.forward(),
                &mut meshes,
                &mut materials,
            );
        }
    }
}
