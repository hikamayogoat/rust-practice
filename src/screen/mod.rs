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

const NO_BLOCK_COLOR_LOWER: u8 = 25;
const NO_BLOCK_COLOR_UPPER: u8 = 65;
const NO_BLOCK_COLOR_DIFF_UPPER: u8 = 40;

pub fn screen_test() {
    // 盤面情報を保持する2次元配列
    let mut field = [[false; FIELD_CELL_HEIGHT as usize]; FIELD_CELL_WIDTH as usize];

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
    for y in 0..FIELD_CELL_HEIGHT{
        for x in 0..FIELD_CELL_WIDTH{
            field[x as usize][y as usize] = check_block_exist(&cliped, x, y, &cell_size);
            // if field[x as usize][y as usize] {
            //     print!("o");
            // } else {
            //     print!(".");
            // }
        }
        println!("");
    }

    
}

fn check_block_exist(image: &Mat, x: i32, y: i32, cell_size: &(i32, i32)) -> bool {
    // 1マスを切り出して kmeans に渡せる形にする
    let cell_u = Mat::roi(
        image,
        Rect_{x: x * cell_size.0, y: y * cell_size.1, width: cell_size.0, height: cell_size.1}
    ).unwrap();

    let mut cell_f = Mat::default();
    cell_u.convert_to(&mut cell_f, CV_32F, 1.0, 0.0);

    let cell = cell_f.reshape(1,  cell_f.rows() * cell_f.cols()).unwrap();
    
    // kmeans クラスタリングで代表色を抜き出す
    const CLUSTER_NUM: i32 = 3;
    let mut label = Mat::default();
    let mut center= Mat::default();
    let criteria = TermCriteria {
        typ: 100,
        max_count: 30,
        epsilon: 1.0
    };
    kmeans(
        &cell,
        CLUSTER_NUM,
        &mut label, // 謎:no_array() にすると center が continuous ではなくなってしまう
        criteria,
        10,
        KMEANS_RANDOM_CENTERS,
        &mut center
    );

    // 出現数のラベルを発見して代表色のインデックスを見つける
    let mut label_u = Mat::default();
    label.convert_to(&mut label_u, CV_8UC1, 1.0, 0.0);
    let l = label_u.data_typed::<u8>().unwrap();
    let mut label_count = [0i32; CLUSTER_NUM as usize];
    for i in 0..l.len() {
        let index = l[i];
        label_count[index as usize] = label_count[index as usize] + 1;
    }
    let mut max_index = 0;
    for i in 0..(label_count.len() - 1) {
        if label_count[i] < label_count[i+1] {
            max_index = i;
        }
    }

    // 代表色のBGRを取得する
    let mut center_u = Mat::default();
    center.convert_to(&mut center_u, CV_8UC1, 1.0, 0.0);
    let data = center_u.data_typed::<u8>().unwrap();

    let offset = max_index * CLUSTER_NUM as usize;
    let color = (data[offset+2], data[offset+1], data[offset]);
    println!("{:?}", color);
    imshow("cell", &cell_u);

    // if 
    //     data[0] >= NO_BLOCK_COLOR_LOWER
    //     && data[1] >= NO_BLOCK_COLOR_LOWER
    //     && data[2] >= NO_BLOCK_COLOR_LOWER
    //     && data[0] <= NO_BLOCK_COLOR_UPPER
    //     && data[1] <= NO_BLOCK_COLOR_UPPER
    //     && data[2] <= NO_BLOCK_COLOR_UPPER
    //     && std::cmp::max(data[0], std::cmp::max(data[1], data[2])) 
    //     - std::cmp::min(data[0], std::cmp::min(data[1], data[2])) <= NO_BLOCK_COLOR_DIFF_UPPER
    // {
    //     return false;
    // } else {
    //     return true;
    // }
    return true

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