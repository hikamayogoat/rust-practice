mod key;
mod screen;
mod util;

use key::{key_test, move_right, move_left, move_drop};
use screen::{screen::{get_board_state, BoardState}, values::{CELL_HEIGHT, CELL_WIDTH, CellState, Mino}};
use util::{display::print_field, checker::is_in_game};

fn main() {
   let screen_number = 1usize;
   let mut last_board_state: &BoardState;
   let mut last_nexts = [Mino::UNKNOWN; 5];
   let mut current_mino = Mino::UNKNOWN;

   loop {
      let board_state = get_board_state(screen_number);
      if is_in_game(&board_state) {
         // 有効な最後の盤面を保存しておく
         last_board_state = &board_state;

         // 最初の一回は単にネクストを保存しておく
         if last_nexts[0] == Mino::UNKNOWN {
            last_nexts = board_state.nexts;
         }

         // ネクストが1つ前に進んだら処理を開始
         // スライスでもっときれいに書けるかも
         if board_state.nexts[0] == last_nexts[1] &&
            board_state.nexts[1] == last_nexts[2] &&
            board_state.nexts[2] == last_nexts[3] &&
            board_state.nexts[3] == last_nexts[4] {
               current_mino = last_nexts[0];
               println!("{:?}", current_mino);
               last_nexts = board_state.nexts;
            }

         match current_mino {
            Mino::I => { 
               move_right();
               current_mino = Mino::UNKNOWN;
            },
            Mino::J => {
               move_right();
               current_mino = Mino::UNKNOWN;
            },
            Mino::L => {
               move_left();
               current_mino = Mino::UNKNOWN;
            },
            Mino::O => {
               move_drop();
               current_mino = Mino::UNKNOWN;
            },
            Mino::S => {
               move_right();
               current_mino = Mino::UNKNOWN;
            },
            Mino::Z => { 
               move_left();
               current_mino = Mino::UNKNOWN;
            },
            Mino::T => {
               move_drop();
               current_mino = Mino::UNKNOWN;
            },
            Mino::UNKNOWN => {}
         }
         
         // 表示
         // print_field(board_state);
      }
   }

}
