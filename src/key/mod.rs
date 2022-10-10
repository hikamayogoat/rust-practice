use std::{thread, time::{self, Duration}};

use enigo::{Enigo, KeyboardControllable, Key};

pub fn key_test () {
    let mut enigo = Enigo::new();

    input_key(&mut enigo, Key::LeftArrow);
    input_key(&mut enigo, Key::Raw(0x5A));
    input_key(&mut enigo, Key::Space);

}

pub fn move_right() {
    let mut enigo = Enigo::new();

    input_key(&mut enigo, Key::RightArrow);
    input_key(&mut enigo, Key::RightArrow);
    input_key(&mut enigo, Key::RightArrow);
    input_key(&mut enigo, Key::RightArrow);
    input_key(&mut enigo, Key::RightArrow);
    input_key(&mut enigo, Key::Space);
}

pub fn move_left() {
    let mut enigo = Enigo::new();

    input_key(&mut enigo, Key::LeftArrow);
    input_key(&mut enigo, Key::LeftArrow);
    input_key(&mut enigo, Key::LeftArrow);
    input_key(&mut enigo, Key::LeftArrow);
    input_key(&mut enigo, Key::LeftArrow);
    input_key(&mut enigo, Key::Space);
}

pub fn move_drop() {
    let mut enigo = Enigo::new();

    input_key(&mut enigo, Key::Space);
}

fn input_key(enigo: &mut Enigo, key: Key) {
    enigo.key_down(key);
    thread::sleep(Duration::from_millis(15));
    enigo.key_up(key);
    thread::sleep(Duration::from_millis(15));
}