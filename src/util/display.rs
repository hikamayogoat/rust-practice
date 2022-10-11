use crate::screen::{screen::{BoardState}, values::{CELL_HEIGHT, CELL_WIDTH, CellState}};

pub fn print_field(board_state: BoardState) {
    for y in 0..CELL_HEIGHT {
        for x in 0..CELL_WIDTH {
        match board_state.field[x][y] {
            CellState::EXIST => print!("o"),
            CellState::NONE => print!("."),
            CellState::UNKNOWN => print!("?"),
        }
        }
        println!()
    }

    println!("{:?}", board_state.nexts);
}