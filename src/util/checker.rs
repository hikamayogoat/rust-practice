use crate::screen::{screen::{get_board_state, BoardState}, values::Mino};

pub fn is_in_game(board_state: &BoardState) -> bool {
   // Nextがひとつでも取得できていなかったら不可
   if !check_nexts(board_state.nexts) {
    return false;
   }

   return true;
}

// もっといい方法ありそう
fn check_nexts(nexts: [Mino; 5]) -> bool {
    for e in nexts {
        if let Mino::UNKNOWN = e {
            return false;
        }
    }
    return true 
}