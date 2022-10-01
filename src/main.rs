mod key;
mod screen;

use screen::screen::get_current_field;

fn main() {
   let (field, nexts) = get_current_field(1);
}
