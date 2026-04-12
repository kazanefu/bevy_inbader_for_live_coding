use bevy::{math::NormedVectorSpace, prelude::*};

pub struct UtilPlugin;

impl Plugin for UtilPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (tick_interval, update_velocity, cleanup_dead).run_if(
                in_state(crate::state::GameState::Playing)
                    .and(in_state(super::InGameState::Running)),
            ),
        );
    }
}

#[derive(Component, PartialEq, Eq, Clone, Copy)]
pub enum Character {
    Player,
    Enemy,
}

#[derive(Component)]
pub struct Interval {
    pub time: f32,
    pub interval: f32,
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

fn update_velocity(query: Query<(&mut Control, &mut Transform)>, time: Res<Time>) {
    for (mut control, mut transform) in query {
        control.calculate_velocity(time.delta_secs());
        transform.translation += control.velocity * time.delta_secs();
        transform.translation.x = transform.translation.x.clamp(-20.0, 20.0);
    }
}

#[derive(Component)]
pub struct Dead;

pub fn cleanup_dead(mut commands: Commands, query: Query<Entity, With<Dead>>) {
    for e in &query {
        commands.entity(e).despawn();
    }
}
