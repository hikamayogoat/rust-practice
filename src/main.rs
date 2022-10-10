mod key;
mod screen;
mod util;

use key::key_test;
use screen::{screen::get_board_state, values::{CELL_HEIGHT, CELL_WIDTH, CellState}};
use util::{display::print_field, checker::is_in_game};

fn main() {
   let screen_number = 1usize;

   loop {
      let board_state = get_board_state(screen_number);
      if is_in_game(&board_state) {
         print_field(board_state);
         key_test();
      }
   }

   
}
