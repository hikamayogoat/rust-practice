use opencv::{prelude::Mat, core::{Scalar, in_range, copy_to}, highgui, imgcodecs};

pub fn filter_color(image: Mat) -> Mat {

    let lower = Scalar::new(100.0, 0.0, 0.0, 0.0);
    let upper = Scalar::new(255.0, 100.0, 100.0, 255.0);

    let mut mask = Mat::default();

    in_range(&image, &lower, &upper, &mut mask);

    let mut filtered = Mat::default();
    
    copy_to(&image, &mut filtered, &mask);

    return filtered;

}

pub fn imshow(name: &str, image: &Mat) {
	// highgui::named_window(name, WINDOW_AUTOSIZE);
    highgui::named_window(name, 0);
	highgui::imshow(name, image);
	highgui::wait_key(0);
    highgui::destroy_all_windows();
}

pub fn read_image_file(filename: &str) -> Mat {
    return imgcodecs::imread(filename, 1).unwrap();
}