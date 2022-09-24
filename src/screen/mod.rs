
use screenshots::{Screen};
use opencv::{
    highgui::{self, WINDOW_AUTOSIZE},
    imgproc::{cvt_color, COLOR_BGR2HSV},
    imgcodecs::{self, imread, imdecode, imdecode_to},
    prelude::*, core::*, sys::cv_imdecode_const__InputArrayR_int_MatX, types::VectorOfu8, 
};

const ORIGINAL_FILENAME: &str = "tetris/original.jpg";
const TET_FILENAME: &str = "tetris/sprint.jpg";

// マス数
const CELL_WIDTH: usize = 10;
const CELL_HEIGHT: usize = 20;

// 画面サイズをこの数値で割ると盤面の端の座標になる値。 (x_low, x_high, y_low, y_high)
const FIELD_RATIO: (f32, f32, f32, f32) = (6.2, 2.87, 6.75, 1.22); // ぷよテト1 Sprint

// 画面サイズをこの数値で割るとネクストの領域の端の座標になる値。 (x_low, x_high, y_low, y_high)
// ぷよテト1
const NEXT1_POS_RASIO: (f32, f32, f32, f32) = (2.75, 2.34, 6.35, 4.4);
const NEXT2_POS_RASIO: (f32, f32, f32, f32) = (2.75, 2.4, 3.85, 3.13);
const NEXT3_POS_RASIO: (f32, f32, f32, f32) = (2.75, 2.4, 2.8, 2.45);
const NEXT4_POS_RASIO: (f32, f32, f32, f32) = (2.75, 2.4, 2.25, 2.0);
const NEXT5_POS_RASIO: (f32, f32, f32, f32) = (2.75, 2.4, 1.86, 1.68);

fn ratio_to_position(display_width: u32, display_height: u32, ratio: (f32, f32, f32, f32)) -> (usize, usize, usize, usize) {
    let lower_x = display_width as f32 / ratio.0;
    let higher_x = display_width as f32 / ratio.1;
    let lower_y = display_height as f32 / ratio.2;
    let higher_y = display_height as f32 / ratio.3;

    return (lower_x as usize, higher_x as usize, lower_y as usize, higher_y as usize);
}

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
#[derive(Clone)]
#[derive(Copy)]
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

    while true {
        // 画面取得
        let ss = get_ss(1);
        let image = ss.0;
        let display_width = ss.1;
        let display_height = ss.2;
        let field_pos = ratio_to_position(display_width, display_height, FIELD_RATIO);
        let cliped_image = cut_rect(&image, field_pos);

        // 画像読み込み
        let original_image = read_image_file(ORIGINAL_FILENAME);
        let cliped_original = cut_rect(&original_image, field_pos);


        // 座標の区切りサイズを計算する
        let width = field_pos.1 - field_pos.0;
        let height = field_pos.3 - field_pos.2;
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
        let next_ratios = [NEXT1_POS_RASIO, NEXT2_POS_RASIO, NEXT3_POS_RASIO, NEXT4_POS_RASIO, NEXT5_POS_RASIO];
        let mut nexts = [Mino::UNKNOWN; 5];
        for i in 0..next_ratios.len() {
            let next_pos = ratio_to_position(display_width, display_height, next_ratios[i]);
            let img = cut_rect(&image, next_pos);
            nexts[i] = estimate_block(&img);
        }

        println!("Nexts-------");
        println!("{:?}", nexts);
        // imshow("test", &image);
    }

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

// 指定された領域を切り出して返す
fn cut_rect(image: &Mat, pos: (usize, usize, usize, usize)) -> Mat {
    let width = pos.1 - pos.0;
    let height = pos.3 - pos.2;
    let cliped = Mat::roi(
        &image,
        Rect_ { x: (pos.0) as i32, y: (pos.2) as i32, width: (width) as i32, height: (height) as i32 }
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

fn get_ss(screen_number: usize) -> (Mat, u32, u32) {

    let screens = Screen::all();
    let target_screen = screens.unwrap()[screen_number];
    let capture = target_screen.capture().unwrap();
    let buffer = capture.buffer();

    let mat = imdecode(&VectorOfu8::from_iter(buffer.clone()), 1).unwrap();

    return (mat, capture.width(), capture.height());
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