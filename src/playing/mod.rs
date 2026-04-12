use crate::{playing::hp::HPUi, state};
use bevy::prelude::*;
pub mod bullet;
pub mod enemy;
pub mod hp;
pub mod player;
pub mod utils;

const TIME_LIMIT: f32 = 100.0;

pub struct PlayingPlugin;

impl Plugin for PlayingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StopWatch::new(false))
            .insert_resource(CurrentScore(Score::default()))
            .insert_resource(ScoreList(Vec::new()))
            .init_state::<InGameState>()
            .add_plugins(utils::UtilPlugin)
            .add_plugins(player::PlayerPlugin)
            .add_plugins(bullet::BulletPlugin)
            .add_plugins(enemy::EnemyPlugin)
            .add_systems(
                OnEnter(state::GameState::Playing),
                (setup_playing, start_stopwatch_res),
            )
            .add_systems(
                Update,
                (
                    update_stopwatch,
                    update_time_ui,
                    update_pause_button,
                    update_start_button,
                    (hp::update_player_hp, hp::handle_enemy_death).chain(),
                )
                    .run_if(in_state(state::GameState::Playing)),
            )
            .add_systems(OnExit(state::GameState::Playing), push_score_list);
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
pub enum InGameState {
    #[default]
    Running,
    Paused,
}

fn setup_ui(asset_server: &AssetServer) -> impl Bundle {
    (
        UI,
        DespawnOnExit(state::GameState::Playing),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::FlexEnd,
            justify_content: JustifyContent::FlexStart,
            flex_direction: FlexDirection::Column,
            row_gap: px(10.0),
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
                TextColor::WHITE,
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

fn setup_playing(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut current_score: ResMut<CurrentScore>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    reset_current_score(&mut current_score);
    // spawn a camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 30.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
        DespawnOnExit(state::GameState::Playing),
    ));
    commands.spawn(setup_ui(&asset_server));
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(
                asset_server.load("embedded://bevy_invader_for_live_coding/img/invader_background.png"),
            ),
            unlit: true,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, -10.0),
        DespawnOnExit(state::GameState::Playing),
    ));
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

fn update_time_ui(
    stopwatch: Res<StopWatch>,
    mut current_score: ResMut<CurrentScore>,
    mut time_ui_query: Query<&mut Text, With<TimeUI>>,
    mut game_state: ResMut<NextState<crate::state::GameState>>,
) {
    for mut time_ui in &mut time_ui_query {
        let current_time = stopwatch.now();
        **time_ui = format!("Time: {:.2}s / 100s\n Kill: {}", current_time, current_score.0.kill);
        current_score.0.survival_time = current_time;
        if current_time >= TIME_LIMIT {
            game_state.set(state::GameState::Result);
        }
    }
}

type StartButtonInputs = (Changed<Interaction>, With<StartButton>);
fn update_start_button(
    mut query: Query<(&Interaction, &mut BackgroundColor), StartButtonInputs>,
    mut stopwatch: ResMut<StopWatch>,
    mut game_state: ResMut<NextState<InGameState>>,
) {
    for (interaction, mut background_color) in query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                background_color.0 = Color::srgb(0.5, 0.5, 0.5);
                stopwatch.start();
                game_state.set(InGameState::Running);
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
    mut game_state: ResMut<NextState<InGameState>>,
) {
    for (interaction, mut background_color) in query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                background_color.0 = Color::srgb(0.5, 0.5, 0.5);
                stopwatch.pause();
                game_state.set(InGameState::Paused);
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

#[derive(Default, Clone, Copy)]
pub struct Score {
    pub kill: i32,
    pub survival_time: f32,
}

impl Score {
    pub fn score(&self) -> f32 {
        (self.kill as f32 * self.survival_time.min(100.0)).sqrt()
    }
}

impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "Kill: {}\nSurvival Time: {:.2}\nScore: {:.2}",
            self.kill,
            self.survival_time,
            self.score()
        ))
    }
}

#[derive(Resource)]
pub struct CurrentScore(Score);

fn reset_current_score(current_score: &mut CurrentScore) {
    current_score.0 = Score::default();
}

#[derive(Resource)]
pub struct ScoreList(pub Vec<Score>);

fn push_score_list(mut score_list: ResMut<ScoreList>, current_score: Res<CurrentScore>) {
    score_list.0.push(current_score.0);
}
