use crate::{playing::hp_ui::HPUi, state};
use bevy::prelude::*;
pub mod bullet;
pub mod enemy;
pub mod hp_ui;
pub mod player;

#[derive(Component, PartialEq, Eq, Clone, Copy)]
pub enum Character {
    Player,
    Enemy,
}
pub struct OnGamePlugin;

impl Plugin for OnGamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StopWatch::new(false))
            .init_state::<OnGameState>()
            .add_plugins(player::PlayerPlugin)
            .add_plugins(bullet::BulletPlugin)
            .add_plugins(enemy::EnemyPlugin)
            .add_systems(
                OnEnter(state::GameState::OnGame),
                (setup_playing, start_stopwatch_res),
            )
            .add_systems(
                Update,
                (
                    update_stopwatch,
                    update_time_ui,
                    update_pause_button,
                    update_start_button,
                    (hp_ui::check_player_hp, hp_ui::check_hp).chain(),
                )
                    .run_if(in_state(state::GameState::OnGame)),
            );
    }
}

#[derive(Component)]
struct UI;

#[derive(Component)]
struct PauseButton;

#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct TimeUI;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum OnGameState {
    #[default]
    Running,
    Paused,
}

fn setup_ui(asset_server: &AssetServer) -> impl Bundle {
    (
        UI,
        DespawnOnExit(state::GameState::OnGame),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::FlexEnd,
            ..default()
        },
        children![
            (
                Button,
                StartButton,
                Node {
                    width: percent(20),
                    height: percent(10),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::MAX,
                    ..default()
                },
                BorderColor::all(Color::WHITE),
                BackgroundColor(Color::WHITE),
                children![(
                    Text::new("Start"),
                    TextFont {
                        font: asset_server.load(
                            "embedded://bevy_invader_for_live_coding/fonts/NotoSansJP-Bold.ttf"
                        ),
                        font_size: 40.0,
                        ..default()
                    },
                    TextLayout::new_with_justify(Justify::Center),
                    TextColor::BLACK,
                )]
            ),
            (
                Button,
                PauseButton,
                Node {
                    width: percent(20),
                    height: percent(10),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::MAX,
                    ..default()
                },
                BorderColor::all(Color::WHITE),
                BackgroundColor(Color::WHITE),
                children![(
                    Text::new("Pause"),
                    TextFont {
                        font: asset_server.load(
                            "embedded://bevy_invader_for_live_coding/fonts/NotoSansJP-Bold.ttf"
                        ),
                        font_size: 40.0,
                        ..default()
                    },
                    TextLayout::new_with_justify(Justify::Center),
                    TextColor::BLACK,
                )]
            ),
            (
                Text::new(""),
                TimeUI,
                TextFont {
                    font: asset_server
                        .load("embedded://bevy_invader_for_live_coding/fonts/NotoSansJP-Bold.ttf"),
                    font_size: 40.0,
                    ..default()
                },
                TextLayout::new_with_justify(Justify::Center),
                TextColor::BLACK,
            ),
            (
                Text::new(""),
                HPUi,
                TextFont {
                    font: asset_server
                        .load("embedded://bevy_invader_for_live_coding/fonts/NotoSansJP-Bold.ttf"),
                    font_size: 40.0,
                    ..default()
                },
                TextLayout::new_with_justify(Justify::Center),
                TextColor::WHITE,
            ),
        ],
    )
}

fn setup_playing(mut commands: Commands, asset_server: Res<AssetServer>) {
    // spawn a camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 30.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
        DespawnOnExit(state::GameState::OnGame),
    ));
    commands.spawn(setup_ui(&asset_server));
}

#[derive(Resource, Default)]
pub struct StopWatch {
    time: f32,
    is_running: bool,
}

impl StopWatch {
    pub fn new(run: bool) -> Self {
        Self {
            time: 0.0,
            is_running: run,
        }
    }
    pub fn now(&self) -> f32 {
        self.time
    }
    pub fn start(&mut self) {
        self.is_running = true;
    }
    pub fn pause(&mut self) {
        self.is_running = false;
    }
    pub fn reset(&mut self) {
        self.time = 0.0;
    }
    pub fn is_running(&self) -> bool {
        self.is_running
    }
}

fn update_stopwatch(time: Res<Time>, mut stopwatch: ResMut<StopWatch>) {
    if stopwatch.is_running() {
        stopwatch.time += time.delta_secs();
    }
}

fn start_stopwatch_res(mut stopwatch: ResMut<StopWatch>) {
    stopwatch.reset();
    stopwatch.start();
}

fn update_time_ui(stopwatch: Res<StopWatch>, mut time_ui_query: Query<&mut Text, With<TimeUI>>) {
    for mut time_ui in &mut time_ui_query {
        **time_ui = format!("Time: {:.2}", stopwatch.now())
    }
}

type StartButtonInputs = (Changed<Interaction>, With<StartButton>);
fn update_start_button(
    mut query: Query<(&Interaction, &mut BackgroundColor), StartButtonInputs>,
    mut stopwatch: ResMut<StopWatch>,
    mut game_state: ResMut<NextState<OnGameState>>,
) {
    for (interaction, mut background_color) in query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                background_color.0 = Color::srgb(0.5, 0.5, 0.5);
                stopwatch.start();
                game_state.set(OnGameState::Running);
            }
            Interaction::Hovered => {
                background_color.0 = Color::srgb(0.7, 0.7, 0.7);
            }
            Interaction::None => {
                background_color.0 = Color::srgb(0.9, 0.9, 0.9);
            }
        }
    }
}
type PauseButtonInputs = (Changed<Interaction>, With<PauseButton>);
fn update_pause_button(
    mut query: Query<(&Interaction, &mut BackgroundColor), PauseButtonInputs>,
    mut stopwatch: ResMut<StopWatch>,
    mut game_state: ResMut<NextState<OnGameState>>,
) {
    for (interaction, mut background_color) in query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                background_color.0 = Color::srgb(0.5, 0.5, 0.5);
                stopwatch.pause();
                game_state.set(OnGameState::Paused);
            }
            Interaction::Hovered => {
                background_color.0 = Color::srgb(0.7, 0.7, 0.7);
            }
            Interaction::None => {
                background_color.0 = Color::srgb(0.9, 0.9, 0.9);
            }
        }
    }
}
