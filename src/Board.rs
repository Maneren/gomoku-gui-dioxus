use std::f32::consts::PI;

use dioxus::prelude::*;
use gomoku_lib::{Board, Player, TilePointer};

pub fn find_win_sequence(sequence: &[usize], board: &Board) -> Option<(TilePointer, TilePointer)> {
  let mut current = Player::X;
  let mut start = TilePointer { x: 0, y: 0 };
  let mut end = TilePointer { x: 0, y: 0 };
  let mut consecutive = 0;

  for tile in sequence {
    if let Some(player) = *board.get_tile_raw(*tile) {
      if player == current {
        if consecutive == 0 {
          start = board.get_ptr_from_index(*tile);
        }

        consecutive += 1;
        end = board.get_ptr_from_index(*tile);
        continue;
      }

      if consecutive >= 5 {
        return Some((start, end));
      }

      start = board.get_ptr_from_index(*tile);
      consecutive = 1;
      current = player;
    } else {
      // empty tile
      if consecutive >= 5 {
        return Some((start, end));
      }

      consecutive = 0;
      start = board.get_ptr_from_index(*tile);
    }
  }

  if consecutive >= 5 {
    Some((start, end))
  } else {
    None
  }
}

pub fn find_win(board: &Board) -> Option<(TilePointer, TilePointer)> {
  board
    .sequences()
    .iter()
    .map(|sequence| find_win_sequence(sequence, board))
    .find_map(|x| x)
}

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
  #[props(!optional)]
  win: Option<(TilePointer, TilePointer)>,
  on_click: EventHandler<'a, TilePointer>,
}

pub fn Board<'a>(cx: Scope<'a, Props>) -> Element<'a> {
  let Props {
    board,
    highlight,
    win,
    on_click,
  } = cx.props;

  cx.render(rsx!(div {
    class: "board",
    style { include_str!("./Board.css") }
    if let Some((TilePointer { x: x1, y: y1 }, TilePointer { x: x2, y: y2 })) = win {
      let x1 = f32::from(2 * x1 + 1);
      let y1 = f32::from(2 * y1 + 1);
      let x2 = f32::from(2 * x2 + 1);
      let y2 = f32::from(2 * y2 + 1);

      let len = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();

      let angle = -((x2 - x1) / len).asin() + PI / 2.0;

      rsx!(div {
        class: "win",
        style: "rotate: {angle}rad; top: {y1}rem; left: {x1}rem; width: {len}rem;"
      })
    }
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
