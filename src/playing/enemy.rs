use bevy::prelude::*;
use rand::prelude::*;
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (tick_interval,shoot).run_if(
                in_state(crate::state::GameState::OnGame)
                    .and(in_state(super::OnGameState::Running)),
            ),
        );
    }
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Interval {
    time: f32,
    interval: f32,
}

impl Interval {
    pub fn tick(&mut self, delta_time: f32) {
        self.time += delta_time;
    }
    pub fn reset(&mut self) {
        self.time = 0.0;
    }
    pub fn is_ready(&self) -> bool {
        self.time >= self.interval
    }
}

fn tick_interval(time: Res<Time>, query: Query<&mut Interval>) {
    for mut interval in query {
        interval.tick(time.delta_secs());
    }
}

pub fn shoot(
    mut commands: Commands,
    query: Query<(&Transform, &super::Character, &mut Interval), With<Enemy>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (transform, &owner, mut interval) in query {
        let mut rng = rand::rng();
        let random_val = rng.random_range(0..10);
        if interval.is_ready() && random_val <= 3{
            interval.reset();
            super::bullet::spawn_bullet(
                &mut commands,
                owner,
                transform.translation,
                &mut meshes,
                &mut materials,
            );
        }
    }
}
