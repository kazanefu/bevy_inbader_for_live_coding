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
    app.add_plugins(state::GameStatePlugin)
        .add_plugins(home::HomePlugin)
        .add_plugins(playing::OnGamePlugin)
        .add_plugins(result::ResultPlugin)
        .run();
}
