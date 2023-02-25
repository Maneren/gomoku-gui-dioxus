#![warn(clippy::pedantic)]
#![allow(non_snake_case)]

mod Board;
mod Game;

use dioxus_desktop::{Config, WindowBuilder};
use Game::Game;

fn main() {
  dioxus_desktop::launch_cfg(
    Game,
    Config::new().with_window(
      WindowBuilder::new()
        .with_resizable(true)
        .with_title("Gomoku")
        .with_inner_size(dioxus_desktop::wry::application::dpi::LogicalSize::new(
          800.0, 800.0,
        )),
    ),
  );
}
