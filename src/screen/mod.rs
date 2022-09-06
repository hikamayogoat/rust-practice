use std::fs;

use screenshots::{Screen};
use opencv::{
    highgui::{self, WindowFlags, WINDOW_AUTOSIZE, WINDOW_KEEPRATIO, WINDOW_FULLSCREEN},
    imgcodecs,
    prelude::*, core::*,
};

const SS_FILENAME: &str = "target/screenshot.png";
const TET_FILENAME: &str = "tetris/sprint2.jpg";

// TODO: あとでよしなにやるけど一旦定数で持つ
const UPPER_LEFT: (i32, i32) = (615, 320);
const UPPER_RIGHT: (i32, i32) = (1335, 320);
const BOTTOM_LEFT: (i32, i32) = (615, 1760);
const BOTTOM_RIGHT: (i32, i32) = (1335, 1760);

const FIELD_CELL_WIDTH: i32 = 10;
const FIELD_CELL_HEIGHT: i32 = 20;

pub fn screen_test() {
    // 盤面情報を保持する2次元配列
    let mut field = [[0i32; FIELD_CELL_WIDTH as usize]; FIELD_CELL_HEIGHT as usize];

    // 画像読み込み
    let image = imread(TET_FILENAME);

    // 盤面部分だけ切り取る
    let width = UPPER_RIGHT.0 - UPPER_LEFT.0;
    let height = BOTTOM_LEFT.1 - UPPER_LEFT.1;
    let cliped = Mat::roi(
        &image,
        Rect_ { x: (UPPER_LEFT.0), y: (UPPER_LEFT.1), width: (width), height: (height) }
    ).unwrap();

    // 座標の区切りサイズを計算する
    let cell_size = (width / FIELD_CELL_WIDTH, height / FIELD_CELL_HEIGHT);

    // 1マスごとにブロックがあるかどうかを判定する
    for x in 0..FIELD_CELL_WIDTH {
        for y in 0..FIELD_CELL_HEIGHT {
            check_block_exist(&cliped, x, y, &cell_size);
        }
    }

    // 色でフィルターする
    // let filtered = filter_color(cliped);

    // imshow("filtered", &filtered);
    
}

fn check_block_exist(image: &Mat, x: i32, y: i32, cell_size: &(i32, i32)) {
    // 1マスを切り出して kmeans に渡せる形にする
    let cell_u = Mat::roi(
        image,
        Rect_{x: x * cell_size.0, y: y * cell_size.1, width: cell_size.0, height: cell_size.1}
    ).unwrap();
    let mut cell_f = Mat::default();
    cell_u.convert_to(&mut cell_f, CV_32F, 1.0, 0.0);

    let cell = cell_f.reshape(1,  cell_f.rows() * cell_f.cols()).unwrap();
    imshow("cell", &cell_u);
    
    // kmeans クラスタリングで代表色を抜き出す
    let mut label = Mat::default();
    let mut center= Mat::default();
    let criteria = TermCriteria {
        typ: 100,
        max_count: 10,
        epsilon: 1.0
    };
    kmeans(
        &cell,
        1,
        &mut label, // 謎:no_array() にすると center が continuous ではなくなってしまう
        criteria,
        10,
        KMEANS_RANDOM_CENTERS,
        &mut center
    );
    let mut center_u = Mat::default();
    center.convert_to(&mut center_u, CV_8UC1, 1.0, 0.0);

    let data = center_u.data_typed::<u8>().unwrap();
    println!("({:?})", data);

}

fn write_ss() {
    let screens = Screen::all();
    let main_screen = screens.unwrap()[0];
    let capture = main_screen.capture().unwrap();
    let buffer = capture.buffer();
    fs::write(SS_FILENAME, &buffer).unwrap();
}

fn imread(filename: &str) -> Mat {
    return imgcodecs::imread(filename, 1).unwrap();
}

fn imshow(name: &str, image: &Mat) {
	highgui::named_window(name, WINDOW_AUTOSIZE);
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