use screenshots::{Screen};
use opencv::{
    imgproc::{cvt_color, COLOR_BGR2HSV},
    imgcodecs::imdecode,
    prelude::*, core::*, types::VectorOfu8, 
};

use crate::screen::experimental::*;
use crate::screen::values::*;

type FieldMatrix = [[CellState; CELL_HEIGHT]; CELL_WIDTH];
pub struct BoardState {
    field: FieldMatrix,
    nexts: [Mino; 5]
}

struct ScreenShot {
    image: Mat,
    width: u32,
    height: u32
}

struct RectPosition {
    lower_x: usize,
    upper_x: usize,
    lower_y: usize,
    upper_y: usize
}

pub fn get_board_state(screen_number: usize) -> BoardState {
    // 画面取得
    let ss = get_ss(screen_number);
    let field_pos = ratio_to_position(ss.width, ss.height, &FIELD_RATIO);
    let cliped_image = cut_rect(&ss.image, &field_pos);

    // 画像読み込み
    let original_image = read_image_file(ORIGINAL_FILENAME);
    let cliped_original = cut_rect(&original_image, &field_pos);

    // 座標の区切りサイズを計算する
    let width = field_pos.upper_x - field_pos.lower_x;
    let height = field_pos.upper_y - field_pos.lower_y;
    let cell_size = ((width / CELL_WIDTH), (height / CELL_HEIGHT));

    // 盤面を取得する
    let field = get_field_state(&cliped_original, &cliped_image, &cell_size);

    // ネクストを取得する
    let mut nexts = [Mino::UNKNOWN; 5];
    for i in 0..nexts.len() {
        let next_pos = ratio_to_position(ss.width, ss.height, &NEXT_RATIOS[i]);
        let img = cut_rect(&ss.image, &next_pos);
        nexts[i] = estimate_block(&img);
    }

    return BoardState{ field: field, nexts: nexts };
}

fn ratio_to_position(display_width: u32, display_height: u32, ratio: &PositionRatio) -> RectPosition {
    let lower_x = display_width as f32 / ratio.lower_x_ratio;
    let upper_x = display_width as f32 / ratio.upper_x_ratio;
    let lower_y = display_height as f32 / ratio.lower_y_ratio;
    let upper_y = display_height as f32 / ratio.upper_y_ratio;

    return RectPosition {
        lower_x: lower_x as usize, 
        upper_x: upper_x as usize, 
        lower_y: lower_y as usize, 
        upper_y: upper_y as usize
    };
}

// ミノ判別する
// 正直 kmeans でわざわざやるほどでもない気はするが一旦このまま
fn estimate_block(image: &Mat) -> Mino {
    const CLUSTER_NUM: i32 = 3;

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

    // 候補値が3要素区切りで直列に並んだ配列 (h,s,v,h,s,v,h...)
    let cands_serial = center_u.data_typed::<u16>().unwrap();

    for i in 0..CLUSTER_NUM {
        let first = (i * 3) as usize;
        let candidate: MinoHSV  = MinoHSV {
            kind: Mino::UNKNOWN,
            h: cands_serial[first]*2, // H値を0〜360にスケールする
            s: cands_serial[first+1], 
            v: cands_serial[first+2]
        };

        // 黒っぽかったら切り捨てておく
        if candidate.v <= 100 {
            continue;
        }

        for mino in MINO_HSV_LIST{
            // u16だと負の値になるかもしれないので対策しておく
            let h_range = (std::cmp::max(ALLOW_H_GAP, mino.h) - ALLOW_H_GAP, mino.h + ALLOW_H_GAP);
            let s_range = (std::cmp::max(ALLOW_S_GAP, mino.s) - ALLOW_S_GAP, mino.s + ALLOW_S_GAP);
            let v_range = (std::cmp::max(ALLOW_V_GAP, mino.v) - ALLOW_V_GAP, mino.v + ALLOW_V_GAP);

            if candidate.h >= h_range.0 && candidate.h <= h_range.1 {
                if candidate.s >= s_range.0 && candidate.s <= s_range.1 {
                    if candidate.v >= v_range.0 && candidate.v <= v_range.1 {
                        return mino.kind
                    }
                }
            }
        }
    }
    return Mino::UNKNOWN;
}

// 指定された領域を切り出して返す
fn cut_rect(image: &Mat, pos: &RectPosition) -> Mat {
    let width = pos.upper_x - pos.lower_x;
    let height = pos.upper_y - pos.lower_y;
    let cliped = Mat::roi(
        &image,
        Rect_ { x: (pos.lower_x) as i32, y: (pos.lower_y) as i32, width: (width) as i32, height: (height) as i32 }
    ).unwrap();
    return cliped;
}

fn get_field_state(original: &Mat, image: &Mat, cell_size: &(usize, usize)) -> FieldMatrix {
    let mut field: FieldMatrix = [[CellState::UNKNOWN; CELL_HEIGHT]; CELL_WIDTH];

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

fn get_ss(screen_number: usize) -> ScreenShot {

    let screens = Screen::all();
    let target_screen = screens.unwrap()[screen_number];
    let capture = target_screen.capture().unwrap();
    let buffer = capture.buffer();

    let mat = imdecode(&VectorOfu8::from_iter(buffer.clone()), 1).unwrap();

    return ScreenShot {
        image: mat, 
        width: capture.width(), 
        height: capture.height()
    };
}
