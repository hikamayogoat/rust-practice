use crate::screen::{screen::{get_board_state, BoardState}, values::Mino};

pub fn is_in_game(board_state: &BoardState) -> bool {
   // Nextがひとつでも取得できていなかったら不可
   if !check_nexts_valid(board_state.nexts) {
    return false;
   }

   return true;
}

// もっといい方法ありそう
fn check_nexts_valid(nexts: [Mino; 5]) -> bool {
    for e in nexts {
        if e == Mino::UNKNOWN {
            return false;
        }
    }
    return true 
}