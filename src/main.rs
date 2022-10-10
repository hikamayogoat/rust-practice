mod key;
mod screen;
mod util;

use screen::{screen::get_board_state, values::{CELL_HEIGHT, CELL_WIDTH, CellState}};
use util::display::print_field;

fn main() {

   let board_state = get_board_state(1);

   match board_state {
      Some(b) => {
         print_field(b)
      },
      None => return
   }
   
}
