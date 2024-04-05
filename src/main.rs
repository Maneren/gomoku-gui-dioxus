#![warn(clippy::pedantic)]
#![allow(non_snake_case)]

mod Board;
mod Game;

use dioxus::prelude::{desktop, LaunchBuilder};
use dioxus_desktop::{Config, LogicalSize, WindowBuilder};
use Game::Game;

fn main() {
  LaunchBuilder::desktop()
    .with_cfg(desktop!({
      Config::new().with_window(
        WindowBuilder::new()
          .with_resizable(true)
          .with_title("Gomoku")
          .with_inner_size(LogicalSize::new(800.0, 800.0)),
      )
    }))
    .launch(Game);
}
