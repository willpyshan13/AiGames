mod app;
pub mod game;
pub mod puzzle_game;
pub mod cloud_walk_game;
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
