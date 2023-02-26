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

#[derive(Props)]
struct RowProps<'a> {
  board: &'a Board,
  #[props(!optional)]
  highlight: Option<TilePointer>,
  y: u8,
  on_click: EventHandler<'a, TilePointer>,
}

fn Row<'a>(cx: Scope<'a, RowProps>) -> Element<'a> {
  let RowProps {
    board,
    highlight,
    y,
    on_click,
  } = cx.props;

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
                  highlight: *highlight == Some(ptr),
                  board: board,
              }
          )
      })
  }))
}

#[derive(Props)]
pub struct Props<'a> {
  board: Board,
  #[props(!optional)]
  highlight: Option<TilePointer>,
  on_click: EventHandler<'a, TilePointer>,
}

pub fn Board<'a>(cx: Scope<'a, Props>) -> Element<'a> {
  let Props {
    board,
    highlight,
    on_click,
  } = cx.props;

  cx.render(rsx!(div {
      style { include_str!("./Board.css") }
      (0..board.get_size()).map(|y| {
          rsx!(Row {
              y: y,
              board: board,
              highlight: *highlight,
              on_click: move |ptr| on_click.call(ptr)
          })
      })
  }))
}
