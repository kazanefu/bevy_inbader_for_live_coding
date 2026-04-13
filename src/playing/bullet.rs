use bevy::prelude::*;

use super::utils::*;
use crate::state;

#[derive(Component)]
pub struct Bullet {
    owner: Character,
    velocity: Vec3,
    damage: f32,
}

const BULLET_SPEED: f32 = 30.0;
const PLAYER_BULLET_DAMAGE: f32 = 30.0;
const ENEMY_BULLET_DAMAGE: f32 = 3.0;
const BULLET_COLLISION_RADIUS: f32 = 1.0;
const BULLET_LIFE_TIME: f32 = 0.7;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (bullet_collision, move_bullet, remove_time_out_bullet).run_if(
                in_state(crate::state::GameState::Playing)
                    .and(in_state(super::InGameState::Running)),
            ),
        );
    }
}

fn move_bullet(query: Query<(&mut Transform, &Bullet)>, time: Res<Time>) {
    for (mut transform, bullet) in query {
        transform.translation += bullet.velocity * time.delta_secs();
    }
}

pub fn spawn_bullet(
    commands: &mut Commands,
    owner: Character,
    translation: Vec3,
    forward: Dir3,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let (color, damage) = match owner {
        Character::Player => (Color::srgb(0.0, 1.0, 1.0), PLAYER_BULLET_DAMAGE),
        Character::Enemy => (Color::srgb(1.0, 0.0, 1.0), ENEMY_BULLET_DAMAGE),
    };
    commands.spawn((
        Bullet {
            owner,
            velocity: forward.normalize() * BULLET_SPEED,
            damage,
        },
        super::utils::Interval {
            time: 0.0,
            interval: BULLET_LIFE_TIME,
        },
        DespawnOnExit(state::GameState::Playing),
        Transform::from_translation(translation).looking_to(forward, Vec3::Y),
        Mesh3d(meshes.add(Cuboid::new(0.2, 0.2, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::BLACK,
            emissive: color.to_linear(),
            ..default()
        })),
    ));
}

fn bullet_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform, &Bullet)>,
    mut character_query: Query<(&Transform, &Character, &mut HP)>,
) {
    for (bullet_entity, bullet_transform, bullet) in bullet_query {
        for (character_transform, character, mut hp) in character_query.iter_mut() {
            if *character == bullet.owner {
                continue;
            }
            let distance_sq = bullet_transform
                .translation
                .distance_squared(character_transform.translation);
            if distance_sq <= BULLET_COLLISION_RADIUS * BULLET_COLLISION_RADIUS {
                hp.0 -= bullet.damage;
                commands.entity(bullet_entity).despawn();
                break ;
            }
        }
    }
}


fn remove_time_out_bullet(
    mut commands: Commands,
    bullet_query: Query<(Entity, &super::utils::Interval), With<Bullet>>,
) {
    for (entity, interval) in bullet_query {
        if interval.is_ready() {
            commands.entity(entity).despawn();
        }
    }
}
