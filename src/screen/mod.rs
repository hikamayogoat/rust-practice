use std::fs;

use screenshots::{Screen};
use opencv::{
    highgui::{self},
    imgproc::{cvt_color, COLOR_BGR2HSV},
    imgcodecs,
    prelude::*, core::*, 
};

const ORIGINAL_FILENAME: &str = "tetris/original.jpg";
const TET_FILENAME: &str = "tetris/sprint2.jpg";

// TODO: あとでよしなにやるけど一旦定数で持つ
const UPPER_LEFT: (usize, usize) = (615, 320);
const UPPER_RIGHT: (usize, usize) = (1335, 320);
const BOTTOM_LEFT: (usize, usize) = (615, 1760);
const BOTTOM_RIGHT: (usize, usize) = (1335, 1760);

const FIELD_CELL_WIDTH: usize = 10;
const FIELD_CELL_HEIGHT: usize = 20;


pub fn screen_test() {
    // 盤面情報を保持する2次元配列
    let mut field = [[false; FIELD_CELL_HEIGHT]; FIELD_CELL_WIDTH];

    // 画像読み込み
    let original_image = imread(ORIGINAL_FILENAME);
    let cliped_original = cut_field(&original_image);

    let image = imread(TET_FILENAME);
    let cliped_image = cut_field(&image);

    // 座標の区切りサイズを計算する
    let width = UPPER_RIGHT.0 - UPPER_LEFT.0;
    let height = BOTTOM_LEFT.1 - UPPER_LEFT.1;
    let cell_size = ((width / FIELD_CELL_WIDTH), (height / FIELD_CELL_HEIGHT));

    // 盤面を取得する
    field = get_field_info(&cliped_original, &cliped_image, &cell_size);

    for y in 0..FIELD_CELL_HEIGHT {
        for x in 0..FIELD_CELL_WIDTH {
            if field[x][y] {
                print!("o");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

// 盤面部分だけ切り取る
fn cut_field(image: &Mat) -> Mat {
    let width = UPPER_RIGHT.0 - UPPER_LEFT.0;
    let height = BOTTOM_LEFT.1 - UPPER_LEFT.1;
    let cliped = Mat::roi(
        &image,
        Rect_ { x: (UPPER_LEFT.0) as i32, y: (UPPER_LEFT.1) as i32, width: (width) as i32, height: (height) as i32 }
    ).unwrap();
    return cliped;
}

fn get_field_info(original: &Mat, image: &Mat, cell_size: &(usize, usize)) -> [[bool; FIELD_CELL_HEIGHT]; FIELD_CELL_WIDTH] {
    let mut field = [[false; FIELD_CELL_HEIGHT]; FIELD_CELL_WIDTH];

    // 背景差分を計算する
    let mut diff_rgb = Mat::default();
    absdiff(original, image, &mut diff_rgb);

    // 差分をHSVに変換する
    let mut diff_hsv = Mat::default();
    cvt_color(&diff_rgb, &mut diff_hsv,COLOR_BGR2HSV, diff_rgb.channels());

    // 各マスごとに明度でブロックの有無を判定する
    for y in 0..FIELD_CELL_HEIGHT {
        for x in 0..FIELD_CELL_WIDTH {

            let cell_hsv = Mat::roi(
                &diff_hsv,
                Rect_ { x: (x * cell_size.0) as i32, y: (y * cell_size.1) as i32, width: (cell_size.0) as i32, height: (cell_size.1) as i32 }
            ).unwrap();

            // なんかこれをやるとデータが連続になって取れるようになるのでやる
            let mut cell_c = Mat::default();
            cell_hsv.convert_to(&mut cell_c, CV_8UC3, 1.0, 0.0);

            let cell_data = cell_c.data_typed::<Vec3b>().unwrap();
            let mean_hsv = mean_vec(cell_data);

            field[x][y] = mean_hsv.2 >= 100;
        }
    }
    return field;

}

fn mean_vec(vec: &[Vec3b]) -> (u8, u8, u8) {
    let mut h: u32 = 0;
    let mut s: u32 = 0;
    let mut v: u32 = 0;
    for elem in vec {
        h = h + (elem[0] as u32);
        s = s + (elem[1] as u32);
        v = v + (elem[2] as u32);
    }
    h = h / (vec.len() as u32);
    s = s / (vec.len() as u32);
    v = v / (vec.len() as u32);
    return (h as u8, s as u8, v as u8);
}

fn write_ss() {
    let screens = Screen::all();
    let main_screen = screens.unwrap()[0];
    let capture = main_screen.capture().unwrap();
    let buffer = capture.buffer();
    // fs::write(SS_FILENAME, &buffer).unwrap();
}

fn imread(filename: &str) -> Mat {
    return imgcodecs::imread(filename, 1).unwrap();
}

fn imshow(name: &str, image: &Mat) {
	// highgui::named_window(name, WINDOW_AUTOSIZE);
    highgui::named_window(name, 0);
	highgui::imshow(name, image);
	highgui::wait_key(0);
    highgui::destroy_all_windows();
}

fn filter_color(image: Mat) -> Mat {

    let lower = Scalar::new(100.0, 0.0, 0.0, 0.0);
    let upper = Scalar::new(255.0, 100.0, 100.0, 255.0);

    let mut mask = Mat::default();

    in_range(&image, &lower, &upper, &mut mask);

    let mut filtered = Mat::default();
    
    copy_to(&image, &mut filtered, &mask);

    return filtered;

}