use crate::state;
use bevy::prelude::*;

pub struct HomePlugin;

impl Plugin for HomePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(state::GameState::Home), setup_home_screen)
            .add_systems(
                Update,
                update_start_button.run_if(in_state(state::GameState::Home)),
            );
    }
}

#[derive(Component)]
pub struct StartButton;

fn start_button_bundle(assets_server: Res<AssetServer>) -> impl Bundle {
    (
        DespawnOnExit(state::GameState::Home),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
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
            BackgroundColor(Color::BLACK),
            children![(
                Text::new("Start"),
                TextFont {
                    font: assets_server
                        .load("embedded://bevy_invader_for_live_coding/fonts/NotoSansJP-Bold.ttf"),
                    font_size: 40.0,
                    ..default()
                },
                TextLayout::new_with_justify(Justify::Center),
                TextColor::BLACK,
            )]
        )],
    )
}

fn setup_home_screen(mut commands: Commands, assets_server: Res<AssetServer>) {
    // spawn a camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        DespawnOnExit(state::GameState::Home),
    ));
    // spawn UI
    commands.spawn(start_button_bundle(assets_server));
}

type StartButtonInputs = (Changed<Interaction>, With<StartButton>);
fn update_start_button(
    mut query: Query<(&Interaction, &mut BackgroundColor), StartButtonInputs>,
    mut game_state: ResMut<NextState<state::GameState>>,
) {
    for (interaction, mut background_color) in query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                background_color.0 = Color::srgb(0.5, 0.5, 0.5);
                game_state.set(state::GameState::Playing);
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
