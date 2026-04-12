#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::prelude::*;
mod home;
mod state;
mod playing;
mod result;
fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    bevy::asset::embedded_asset!(app, "fonts/NotoSansJP-Bold.ttf");
    bevy::asset::embedded_asset!(app, "img/invader_background.png");
    app.add_plugins(state::GameStatePlugin)
        .add_plugins(home::HomePlugin)
        .add_plugins(playing::PlayingPlugin)
        .add_plugins(result::ResultPlugin)
        .run();
}
