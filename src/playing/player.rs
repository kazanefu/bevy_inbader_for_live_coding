use core::f32;

use bevy::{math::NormedVectorSpace, prelude::*};

use super::bullet;

use super::Character;

const PLAYER_FORCE: f32 = 3.0;
const PLAYER_SPEED_LIMIT: f32 = 10.0;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(crate::state::GameState::OnGame), player_spawn)
            .add_systems(
                Update,
                (update_velocity, move_player, shoot).run_if(
                    in_state(crate::state::GameState::OnGame)
                        .and(in_state(super::OnGameState::Running)),
                ),
            );
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct HP(pub f32);

#[derive(Component, Default)]
pub struct Control {
    pub mass: f32,
    pub force: Vec3,
    pub acceleration: Vec3,
    pub velocity: Vec3,
    pub speed_limit: f32,
}

impl Control {
    pub fn add_force(&mut self, force: Vec3) {
        self.force = force;
    }
    pub fn speed(&self) -> f32 {
        self.velocity.norm()
    }
    pub fn calculate_velocity(&mut self, delta_time: f32) {
        self.acceleration = self.force / self.mass;
        self.velocity += self.acceleration * delta_time;
        if self.speed() >= self.speed_limit {
            self.velocity = self.velocity.normalize() * self.speed_limit;
        }
        self.force = Vec3::ZERO;
    }
}

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
        Transform::from_xyz(0.0, 0.0, -6.0).looking_at(Vec3::ZERO, Vec3::Y),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.7, 1.0))),
        Control {
            speed_limit: PLAYER_SPEED_LIMIT,
            mass: 1.0,
            ..default()
        },
    ));
}

fn update_velocity(query: Query<(&mut Control, &mut Transform)>, time: Res<Time>) {
    for (mut control, mut transform) in query {
        control.calculate_velocity(time.delta_secs());
        transform.translation += control.velocity * time.delta_secs();
        transform.translation.x = transform.translation.x.clamp(-20.0, 20.0);
    }
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
                &mut meshes,
                &mut materials,
            );
        }
    }
}
