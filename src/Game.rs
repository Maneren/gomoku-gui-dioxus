use dioxus::prelude::*;
use gomoku_lib::{Board, Move, Player, TilePointer};

use crate::Board::Board as BoardElement;

pub fn Game(cx: Scope) -> Element {
  let board = use_ref(cx, || Board::get_empty_board(15));
  let current_player = use_state(cx, || Player::X);
  let moves = use_ref(cx, Vec::<TilePointer>::new);
  let loading = use_state(cx, || false);
  let time_limit = use_state(cx, || 5000);

  let on_tile_click = move |ptr: TilePointer| {
    if board.read().get_tile(ptr).is_none() {
      board.write().set_tile(ptr, Some(**current_player));
      moves.write().push(ptr);
      current_player.set(!current_player);
    }
  };

  let calculate = move || {
    loading.set(true);

    let board = board.clone();
    let current_player = current_player.clone();
    let moves = moves.clone();
    let loading = loading.clone();

    let time_limit = **time_limit;

    cx.spawn(async move {
      let mut board_clone = board.read().clone();
      let player = *current_player;

      println!("{board_clone}");

      let (Move { tile: ptr, .. }, ..) = tokio::spawn(async move {
        gomoku_lib::decide(&mut board_clone, player, time_limit).expect("Error running the engine")
      })
      .await
      .expect("Error running tokio thread");

      board.write().set_tile(ptr, Some(*current_player));
      moves.write().push(ptr);
      current_player.set(!&current_player);

      loading.set(false);
    });
  };

  let undo = move || {
    let Some(tile) = moves.write().pop() else {return};

    board.write().set_tile(tile, None);
    current_player.set(!current_player);
  };

  let new_game = move || {
    board.set(Board::get_empty_board(15));
    current_player.set(Player::X);
    moves.set(Vec::new());
  };

  cx.render(rsx!(div {
      class: "game",
      style { include_str!("./Game.css") },
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
          width: "100px",
          onchange: move |evt| time_limit.set(evt.value.parse().unwrap_or(5000)),
        }
      },
      BoardElement {
          board: board.read().clone(),
          highlight: moves.read().last().copied(),
          on_click: move |ptr| { if !loading { on_tile_click(ptr) } }
      },
      if **loading {
          rsx!(div{
              class: "loading",
              "Computing..."
          })
      }
  }))
}
