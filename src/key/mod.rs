use std::{thread, time};

use enigo::{Enigo, KeyboardControllable, Key};

pub fn key_test () {
    let mut enigo = Enigo::new();

    let stop_sec = time::Duration::from_millis(3000);
    thread::sleep(stop_sec);

    enigo.key_down(Key::Shift);
    enigo.key_down(Key::Raw(0x41));
    enigo.key_down(Key::Raw(0x42));
    enigo.key_up(Key::Shift);
}