use screenshots::{Screen};
use opencv::{
    imgproc::{cvt_color, COLOR_BGR2HSV},
    imgcodecs::imdecode,
    prelude::*, core::*, types::VectorOfu8, 
};

use crate::screen::experimental::*;
use crate::screen::values::*;

pub fn get_board_state(screen_number: usize) -> BoardState {
    // 画面取得
    let ss = get_ss(screen_number);
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
    let mut field = [[CellState::UNKNOWN; CELL_HEIGHT]; CELL_WIDTH];
    field = get_field_info(&cliped_original, &cliped_image, &cell_size);

    // ネクストを取得する
    let next_ratios = [NEXT1_POS_RASIO, NEXT2_POS_RASIO, NEXT3_POS_RASIO, NEXT4_POS_RASIO, NEXT5_POS_RASIO];
    let mut nexts = [Mino::UNKNOWN; 5];
    for i in 0..next_ratios.len() {
        let next_pos = ratio_to_position(display_width, display_height, next_ratios[i]);
        let img = cut_rect(&image, next_pos);
        nexts[i] = estimate_block(&img);
    }

    return BoardState{ field: field, nexts: nexts };
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

fn get_field_info(original: &Mat, image: &Mat, cell_size: &(usize, usize)) -> [[CellState; CELL_HEIGHT]; CELL_WIDTH] {
    let mut field = [[CellState::UNKNOWN; CELL_HEIGHT]; CELL_WIDTH];

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
                field[x][y] = CellState::EXIST;
            } else {
                field[x][y] = CellState::NONE;
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
