use bevy::prelude::*;

use crate::playing::player;

use super::Character;

#[derive(Component)]
pub struct Bullet {
    owner: Character,
    velocity: Vec3,
    damage: f32,
}

const BULLET_SPEED: f32 = 30.0;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (bullet_collision, move_bullet).run_if(
                in_state(crate::state::GameState::OnGame)
                    .and(in_state(super::OnGameState::Running)),
            ),
        );
    }
}

pub fn move_bullet(query: Query<(&mut Transform, &Bullet)>,time: Res<Time>) {
    for (mut transform, bullet) in query {
        transform.translation += bullet.velocity * time.delta_secs();
    }
}

pub fn spawn_bullet(
    commands: &mut Commands,
    owner: Character,
    translation: Vec3,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    match owner {
        Character::Player => {
            commands.spawn((
                Bullet {
                    owner,
                    velocity: Vec3::Z * BULLET_SPEED,
                    damage: 30.0,
                },
                Transform::from_translation(translation),
                Mesh3d(meshes.add(Cuboid::new(0.2, 0.2, 1.0))),
                MeshMaterial3d(materials.add(Color::srgb(0.2, 0.2, 1.0))),
            ));
        }
        Character::Enemy => {
            commands.spawn((
                Bullet {
                    owner,
                    velocity: -Vec3::Z * BULLET_SPEED,
                    damage: 10.0,
                },
                Transform::from_translation(translation),
                Mesh3d(meshes.add(Cuboid::new(0.2, 0.2, 1.0))),
                MeshMaterial3d(materials.add(Color::srgb(1.0, 0.2, 0.2))),
            ));
        }
    }
}

pub fn bullet_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform, &Bullet)>,
    mut character_query: Query<(&Transform, &Character, &mut super::player::HP)>,
) {
    for (bullet_entity, bullet_transform, bullet) in bullet_query {
        for (character_transform, character, mut hp) in character_query.iter_mut() {
            if *character == bullet.owner {
                continue;
            }
            let distance = bullet_transform
                .translation
                .distance(character_transform.translation);
            if distance <= 2.0 {
                hp.0 -= bullet.damage;
                commands.entity(bullet_entity).despawn();
            }
        }
    }
}
