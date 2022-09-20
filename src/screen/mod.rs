
use screenshots::{Screen};
use opencv::{
    highgui::{self, WINDOW_AUTOSIZE},
    imgproc::{cvt_color, COLOR_BGR2HSV},
    imgcodecs::{self, imread, imdecode, imdecode_to},
    prelude::*, core::*, sys::cv_imdecode_const__InputArrayR_int_MatX, types::VectorOfu8, 
};

const ORIGINAL_FILENAME: &str = "tetris/original.jpg";
const TET_FILENAME: &str = "tetris/sprint.jpg";

// TODO: あとでよしなにやるけど一旦定数で持つ
// memo: 検出がきびしそうなら画面サイズからの比率で出してもいけるかも

const UPPER_LEFT: (usize, usize) = (615, 320);
const UPPER_RIGHT: (usize, usize) = (1335, 320);
const BOTTOM_LEFT: (usize, usize) = (615, 1760);
const BOTTOM_RIGHT: (usize, usize) = (1335, 1760);

// 比率版
// const X_LOWER_RATIO: f32 = 6.2;
// const X_UPPER_RATIO: f32 = 2.85;
// const Y_LOWER_RATIO: f32 = 6.75;
// const Y_UPPER_RATIO: f32 = 1.23;

// 整数型で演算するので割り切れるように気をつける
const CELL_WIDTH: usize = 10;
const CELL_HEIGHT: usize = 20;

// ネクストの座標: x_lower, x_upper, y_lower, y_upper
const NEXT_1: (usize, usize, usize, usize) = (1400, 1650, 350, 500);
const NEXT_2: (usize, usize, usize, usize) = (1400, 1600, 570, 680);
const NEXT_3: (usize, usize, usize, usize) = (1400, 1600, 770, 890);
const NEXT_4: (usize, usize, usize, usize) = (1400, 1600, 960, 1070);
const NEXT_5: (usize, usize, usize, usize) = (1400, 1600, 1170, 1280);

// ミノの基準 HSV
const S_MINO_HSV: (Mino, u16, u16, u16) =  (Mino::S, 100, 190, 180);
const Z_MINO_HSV: (Mino, u16, u16, u16) = (Mino::Z, 0, 220, 130);
const L_MINO_HSV: (Mino, u16, u16, u16) = (Mino::L, 25, 250, 240);
const J_MINO_HSV: (Mino, u16, u16, u16) = (Mino::J, 210, 250, 170);
const I_MINO_HSV: (Mino, u16, u16, u16) = (Mino::I, 200, 240, 200);
const T_MINO_HSV: (Mino, u16, u16, u16) = (Mino::T, 290, 175, 150);
const O_MINO_HSV: (Mino, u16, u16, u16) = (Mino::O, 45, 215, 200);
const HS_GAP: u16 = 10;
const V_GAP: u16 = 50;

#[derive(Debug)]
enum Mino {
    S, Z, L, J, I, O, T, UNKNOWN
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
enum Field {
    EXIST, NONE, UNKNOWN
}

fn read_image_file(filename: &str) -> Mat {
    return imgcodecs::imread(filename, 1).unwrap();
}

pub fn screen_test() {
    // 画像読み込み
    let original_image = read_image_file(ORIGINAL_FILENAME);
    let cliped_original = cut_field(&original_image);

    let image = read_image_file(TET_FILENAME);
    let cliped_image = cut_field(&image);

    // 座標の区切りサイズを計算する
    let width = UPPER_RIGHT.0 - UPPER_LEFT.0;
    let height = BOTTOM_LEFT.1 - UPPER_LEFT.1;
    let cell_size = ((width / CELL_WIDTH), (height / CELL_HEIGHT));

    // 盤面を取得する
    let mut field = [[Field::UNKNOWN; CELL_HEIGHT]; CELL_WIDTH];
    field = get_field_info(&cliped_original, &cliped_image, &cell_size);

    println!("Field-------");
    for y in 0..CELL_HEIGHT {
        for x in 0..CELL_WIDTH {
            if field[x][y] == Field::EXIST {
                print!("o");
            } else if field[x][y] == Field::NONE {
                print!(".");
            } else {
                print!("?");
            }
        }
        println!("");
    }

    // ネクストを取得する
    let nexts = get_nexts(&image);

    println!("Nexts-------");
    println!("{:?}", nexts);
    imshow("test", &image);

}

// ネクストを取得する
fn get_nexts(image: &Mat) -> [Mino; 5] {

    let next1 = cut_rect(image, NEXT_1);
    let next2 = cut_rect(image, NEXT_2);
    let next3 = cut_rect(image, NEXT_3);
    let next4 = cut_rect(image, NEXT_4);
    let next5 = cut_rect(image, NEXT_5);

    let n1 = estimate_block(&next1);
    let n2 = estimate_block(&next2);
    let n3 = estimate_block(&next3);
    let n4 = estimate_block(&next4);
    let n5 = estimate_block(&next5);

    return [n1, n2, n3, n4, n5];
}

// ミノ判別する
// 正直 kmeans でわざわざやるほどでもない気はするが一旦このまま
fn estimate_block(image: &Mat) -> Mino {
    const CLUSTER_NUM: i32 = 3;
    let mino_hsv_vec = vec![S_MINO_HSV, Z_MINO_HSV, L_MINO_HSV, J_MINO_HSV, I_MINO_HSV, T_MINO_HSV, O_MINO_HSV];

    // HSVに変換する
    let mut image_hsv = Mat::default();
    cvt_color(image, &mut image_hsv, COLOR_BGR2HSV, image.channels());

    let mut label = Mat::default();
    let mut center = Mat::default();

    let criteria = TermCriteria {
        typ: 100,
        max_count: 100,
        epsilon: 0.5
    };

    let mut image_f = Mat::default();
    image_hsv.convert_to(&mut image_f, CV_32F, 1.0, 0.0);

    let image_reshaped = image_f.reshape(3, image_f.rows() * image_f.cols()).unwrap();

    kmeans(
        &image_reshaped,
        CLUSTER_NUM,
        &mut label, 
        criteria,
        10,
        KMEANS_RANDOM_CENTERS,
        &mut center
    );

    // HSVだと8bitじゃ足りなそうという予想のもと,ここ以降16で扱ってみている
    let mut center_u = Mat::default();
    center.convert_to(&mut center_u, CV_16UC3, 1.0, 0.0);

    let cands_serial = center_u.data_typed::<u16>().unwrap();

    for i in 0..CLUSTER_NUM {
        let first = (i * 3) as usize;
        let candidate: [u16; 3] = [cands_serial[first]*2, cands_serial[first+1], cands_serial[first+2]];

        // 黒は切り捨てておく
        if candidate[2] <= 100 {
            continue;
        }

        for mino in &mino_hsv_vec {
            // u16だと負の値になるかもしれないので対策しておく
            let h_range = (std::cmp::max(HS_GAP, mino.1) - HS_GAP, mino.1 + HS_GAP);
            let s_range = (std::cmp::max(V_GAP, mino.2) - V_GAP, mino.2 + V_GAP);
            let v_range = (std::cmp::max(V_GAP, mino.3) - V_GAP, mino.3 + V_GAP);
            if candidate[0] >= h_range.0 && candidate[0] <= h_range.1 {
                if candidate[1] >= s_range.0 && candidate[1] <= s_range.1 {
                    if candidate[2] >= v_range.0 && candidate[2] <= v_range.1 {
                        match mino.0 {
                            Mino::S => return Mino::S,
                            Mino::Z => return Mino::Z,
                            Mino::L => return Mino::L,
                            Mino::J => return Mino::J,
                            Mino::I => return Mino::I,
                            Mino::T => return Mino::T,
                            Mino::O => return Mino::O,
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    return Mino::UNKNOWN;
}

// 指定された領域を切り出す
fn cut_rect(image: &Mat, pos: (usize, usize, usize, usize)) -> Mat {
    let x = pos.0 as i32;
    let y = pos.2 as i32;
    let width = pos.1 as i32 - pos.0 as i32;
    let height = pos.3 as i32 - pos.2 as i32;
    
    let rect = Mat::roi(
        image,
        Rect_{x: x, y: y, width: width, height: height}
    ).unwrap();

    return rect;
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

fn get_field_info(original: &Mat, image: &Mat, cell_size: &(usize, usize)) -> [[Field; CELL_HEIGHT]; CELL_WIDTH] {
    let mut field = [[Field::UNKNOWN; CELL_HEIGHT]; CELL_WIDTH];

    // 背景差分を計算する
    let mut diff_rgb = Mat::default();
    absdiff(original, image, &mut diff_rgb);

    // 差分をHSVに変換する
    let mut diff_hsv = Mat::default();
    cvt_color(&diff_rgb, &mut diff_hsv,COLOR_BGR2HSV, diff_rgb.channels());

    // 各マスごとに明度でブロックの有無を判定する
    for y in 0..CELL_HEIGHT {
        for x in 0..CELL_WIDTH {

            let cell_hsv = Mat::roi(
                &diff_hsv,
                Rect_ { x: (x * cell_size.0) as i32, y: (y * cell_size.1) as i32, width: (cell_size.0) as i32, height: (cell_size.1) as i32 }
            ).unwrap();

            // なんかこれをやるとデータが連続になって取れるようになるのでやる
            let mut cell_c = Mat::default();
            cell_hsv.convert_to(&mut cell_c, CV_8UC3, 1.0, 0.0);

            let cell_data = cell_c.data_typed::<Vec3b>().unwrap();
            let mean_hsv = mean_vec(cell_data);

            println!("{:?}", mean_hsv);

            if mean_hsv.2 >= 100 {
                field[x][y] = Field::EXIST;
            } else {
                field[x][y] = Field::NONE;
            };
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

fn get_ss() -> Mat {

    let screens = Screen::all();
    let main_screen = screens.unwrap()[1];
    let capture = main_screen.capture().unwrap();
    let buffer = capture.buffer();

    let mat = imdecode(&VectorOfu8::from_iter(buffer.clone()), 1).unwrap();

    return mat;
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