use dioxus::prelude::*;
use gomoku_lib::{Board, TilePointer};

#[inline_props]
fn TileElement<'a>(
  cx: Scope,
  board: &'a Board,
  ptr: TilePointer,
  highlight: bool,
  on_click: EventHandler<'a, TilePointer>,
) -> Element {
  cx.render(rsx!(div {
      class: if *highlight { "tile highlight" } else { "tile" },
      onclick: move |_| on_click.call(*ptr),
      board.get_tile(*ptr).map_or(" ".to_owned(), |player| player.char().to_uppercase().to_string())
  }))
}

#[inline_props]
fn Row<'a>(
  cx: Scope,
  board: &'a Board,
  moves: &'a [TilePointer],
  y: u8,
  on_click: EventHandler<'a, TilePointer>,
) -> Element {
  cx.render(rsx!(div{
      key: "{y}",
      class: "row",
      (0..board.get_size()).map(|x| {
          let ptr = TilePointer {x, y: *y};
          rsx!(
              TileElement {
                  key: "{x}",
                  on_click: move |ptr| on_click.call(ptr),
                  ptr: ptr,
                  highlight: moves.last() == Some(&ptr),
                  board: board,
              }
          )
      })
  }))
}

#[inline_props]
pub fn Board<'a>(
  cx: Scope<'a>,
  board: Board,
  moves: Vec<TilePointer>,
  on_click: EventHandler<'a, TilePointer>,
) -> Element {
  cx.render(rsx!(div {
      style { include_str!("./Board.css") }
      (0..board.get_size()).map(|y| {
          rsx!(Row {
              y: y,
              board: board,
              moves: moves,
              on_click: move |ptr| on_click.call(ptr)
          })
      })
  }))
}
