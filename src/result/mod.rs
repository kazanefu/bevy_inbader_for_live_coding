use crate::playing::{Score, ScoreList};
use crate::state;
use bevy::prelude::*;

pub struct ResultPlugin;

impl Plugin for ResultPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(state::GameState::Result), setup_result_screen)
            .add_systems(Update, update_retry_button.run_if(in_state(state::GameState::Result)));
    }
}

#[derive(Component)]
struct RetryButton;

#[derive(Component)]
struct ResultText;

fn setup_result_ui(commands: &mut Commands, asset_server: &AssetServer, score_list: &ScoreList) {
    let mut score_score_list = String::from("Ranking\n");
    let mut sorted_score_list = score_list.0.clone();
    sorted_score_list.sort_by(|a, b| a.score().partial_cmp(&b.score()).expect("NaN in Score"));
    sorted_score_list
        .iter()
        .rev()
        .map(|score| score.score())
        .enumerate()
        .for_each(|(rank, score)| score_score_list.push_str(&format!("No. {}: {:.2}\n",rank + 1,score)));
    commands.spawn((
        DespawnOnExit(state::GameState::Result),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: px(10.0),
            ..default()
        },
        children![
            (
                Text::new(score_list.0.last().unwrap_or(&Score::default()).to_string()),
                ResultText,
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
                Button,
                RetryButton,
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
                    Text::new("Retry"),
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
                Text::new(score_score_list),
                ResultText,
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
    ));
}

fn setup_result_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    score_list: Res<ScoreList>,
) {
    // spawn a camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 30.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
        DespawnOnExit(state::GameState::Result),
    ));
    setup_result_ui(&mut commands, &asset_server, &score_list);
}

type RetryButtonInputs = (Changed<Interaction>, With<RetryButton>);
fn update_retry_button(
    mut query: Query<(&Interaction, &mut BackgroundColor), RetryButtonInputs>,
    mut game_state: ResMut<NextState<state::GameState>>,
) {
    for (interaction, mut background_color) in query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                background_color.0 = Color::srgb(0.5, 0.5, 0.5);
                game_state.set(state::GameState::Home);
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