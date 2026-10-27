mod app;
pub mod game;
mod literacy;
mod math;
mod web_game;

pub fn run() {
    app::run();
}

#[cfg(target_os = "android")]
#[bevy::prelude::bevy_main]
fn main() {
    run();
}
