use dioxus::prelude::*;
use gomoku_lib::{Board, Move, Player, TilePointer};

use crate::Board::{find_win, BoardElement};

pub fn Game() -> Element {
  let mut board = use_context_provider(|| Signal::new(Board::new_empty(15)));
  let mut current_player = use_signal(|| Player::X);
  let mut moves = use_signal(Vec::<TilePointer>::new);
  let mut loading = use_signal(|| false);
  let mut time_limit = use_signal(|| 1000);

  let win = find_win(&board.read());

  let mut calculate = move || {
    if loading() || find_win(&board.read()).is_some() {
      return;
    }

    loading.set(true);

    // let board = board.clone();
    // let current_player = current_player.clone();
    // let moves = moves.clone();
    // let loading = loading.clone();
    // let time_limit = time_limit.clone();

    spawn(async move {
      let mut board_clone = board.read().clone();
      let player = current_player();
      let time_limit = time_limit();

      println!("{board_clone}");

      let result =
        tokio::spawn(async move { gomoku_lib::decide(&mut board_clone, player, time_limit) })
          .await
          .expect("Error running tokio thread");

      match result {
        Ok((Move { tile: ptr, .. }, ..)) => {
          board.write().set_tile(ptr, Some(player));
          moves.write().push(ptr);

          current_player.set(!player);
        }
        Err(e) => {
          eprintln!("Error running the engine: {e}");
        }
      }

      loading.set(false);
    });
  };

  let mut undo = move || {
    let Some(tile) = moves.write().pop() else {
      return;
    };

    board.write().set_tile(tile, None);

    let player = current_player();
    *current_player.write() = !player;
  };

  let mut new_game = move || {
    board.set(Board::new_empty(15));
    current_player.set(Player::X);
    moves.set(Vec::new());
  };

  let on_tile_click = move |ptr| {
    if loading() || win.is_some() {
      return;
    }

    if board.read().get_tile(ptr).is_none() {
      board.write().set_tile(ptr, Some(current_player()));
      moves.write().push(ptr);

      let player = current_player();
      *current_player.write() = !player;

      calculate();
    }
  };

  rsx! {
    div {
      class: "game",
      style {
        { include_str!("./Game.css") }
      },
      h1 { "Gomoku" },
      div {
        class: "buttons",
        button {
          onclick: move |_| calculate(),
          "Calculate"
        },
        button {
          onclick: move |_| undo(),
          "Undo"
        },
        button {
          onclick: move |_| new_game(),
          "New game"
        }
      },
      div {
        label {
          r#for: "time-limit",
          "Engine time limit (ms): "
        },
        input {
          id: "time-limit",
          r#type: "number",
          value: "{time_limit}",
          placeholder: "Time limit",
          style: "width: 4rem",
          onchange: move |evt| time_limit.set(evt.value().parse().unwrap_or(1000)),
        }
      },
      BoardElement {
        highlight: moves.read().last().copied(),
        win: win,
        on_click: on_tile_click
      },
      {
        loading().then(||
          rsx!(div {
            class: "loading",
            "Computing..."
          })
        )
      }
    }
  }
}
