use super::util::*;
use bevy::prelude::*;
use rand::prelude::*;
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(crate::state::GameState::OnGame), setup_enemys)
            .add_systems(
                Update,
                (enemy_shoot).run_if(
                    in_state(crate::state::GameState::OnGame)
                        .and(in_state(super::OnGameState::Running)),
                ),
            );
    }
}

#[derive(Component)]
pub struct Enemy;

const SHOOT_RATE: i32 = 3;

type LivingEnemy = (With<Enemy>,Without<Dead>);

pub fn enemy_shoot(
    mut commands: Commands,
    query: Query<(&Transform, &Character, &mut super::util::Interval), LivingEnemy>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (transform, &owner, mut interval) in query {
        let mut rng = rand::rng();
        let random_val = rng.random_range(0..10);
        let is_ready = interval.is_ready();
        if is_ready {
            interval.reset();
        }
        if is_ready && random_val <= SHOOT_RATE {
            super::bullet::spawn_bullet(
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

fn setup_enemys(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for i in -3..=3 {
        spawn_enemy(
            &mut commands,
            &mut meshes,
            &mut materials,
            Vec3{x: (i * 3) as f32, y: 0.0, z: 10.0},
        );
    }
}

const ENEMY_SPEED_LIMIT: f32 = 2.0;

pub fn spawn_enemy(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    translation: Vec3,
) {
    commands.spawn((
        DespawnOnExit(crate::state::GameState::OnGame),
        Character::Enemy,
        Enemy,
        super::util::Interval {
            time: 0.0,
            interval: 0.3,
        },
        HP(100.0),
        Transform::from_translation(translation).looking_to(-Vec3::Z, Vec3::Y),
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(1.0, 0.1, 0.1))),
        Control {
            speed_limit: ENEMY_SPEED_LIMIT,
            mass: 1.0,
            ..default()
        },
    ));
}


