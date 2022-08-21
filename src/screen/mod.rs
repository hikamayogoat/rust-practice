use screenshots::{Screen, Image};
use show_image::{ImageView, create_window, ImageInfo, winit::event};

pub fn screen_test() {
    let screens = Screen::all();
    let main_screen = screens.unwrap()[0];

    let capture = main_screen.capture().unwrap();
    let image = ImageView::new(ImageInfo::rgb8(capture.width(), capture.height()), capture.buffer());
    let window = create_window("image", Default::default()).unwrap();
    window.set_image("name", image);
    // for event in window.event_channel() {
    //      println!("{:#?}", event);
    // }
}