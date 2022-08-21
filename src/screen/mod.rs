use std::fs;

use screenshots::{Screen};
use opencv::{
    highgui,
    imgcodecs,
    core::MatTraitManual, prelude::MatTraitConstManual
};

const FILENAME: &str = "target/screenshot.png";

pub fn screen_test() {
    // write_ss();
    imshow();
}

fn write_ss() {
    let screens = Screen::all();
    let main_screen = screens.unwrap()[0];
    let capture = main_screen.capture().unwrap();
    let buffer = capture.buffer();
    fs::write(FILENAME, &buffer).unwrap();
}

#[allow(unused_must_use)]
fn imshow() {
    let image = imgcodecs::imread(FILENAME, 1).unwrap();
    println!("{:?}", image);
	// highgui::named_window("hello opencv!", 0);
	// highgui::imshow("hello opencv!", &(image.unwrap()));
	// highgui::wait_key(10000);
    // highgui::destroy_all_windows();
}