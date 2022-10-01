pub const ORIGINAL_FILENAME: &str = "tetris/original.jpg";
pub const TET_FILENAME: &str = "tetris/sprint.jpg";

// マス数
pub const CELL_WIDTH: usize = 10;
pub const CELL_HEIGHT: usize = 20;

// 画面サイズをこの数値で割ると盤面の端の座標になる値。 (x_low, x_high, y_low, y_high)
pub const FIELD_RATIO: (f32, f32, f32, f32) = (6.2, 2.87, 6.75, 1.22); // ぷよテト1 Sprint

// 画面サイズをこの数値で割るとネクストの領域の端の座標になる値。 (x_low, x_high, y_low, y_high)
// ぷよテト1
pub const NEXT1_POS_RASIO: (f32, f32, f32, f32) = (2.75, 2.34, 6.35, 4.4);
pub const NEXT2_POS_RASIO: (f32, f32, f32, f32) = (2.75, 2.4, 3.85, 3.13);
pub const NEXT3_POS_RASIO: (f32, f32, f32, f32) = (2.75, 2.4, 2.8, 2.45);
pub const NEXT4_POS_RASIO: (f32, f32, f32, f32) = (2.75, 2.4, 2.25, 2.0);
pub const NEXT5_POS_RASIO: (f32, f32, f32, f32) = (2.75, 2.4, 1.86, 1.68);

// ミノの基準 HSV
pub const S_MINO_HSV: (Mino, u16, u16, u16) =  (Mino::S, 100, 190, 180);
pub const Z_MINO_HSV: (Mino, u16, u16, u16) = (Mino::Z, 0, 220, 130);
pub const L_MINO_HSV: (Mino, u16, u16, u16) = (Mino::L, 25, 250, 240);
pub const J_MINO_HSV: (Mino, u16, u16, u16) = (Mino::J, 210, 250, 170);
pub const I_MINO_HSV: (Mino, u16, u16, u16) = (Mino::I, 200, 240, 200);
pub const T_MINO_HSV: (Mino, u16, u16, u16) = (Mino::T, 290, 175, 150);
pub const O_MINO_HSV: (Mino, u16, u16, u16) = (Mino::O, 45, 215, 200);
pub const HS_GAP: u16 = 10;
pub const V_GAP: u16 = 50;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub enum Mino {
    S, Z, L, J, I, O, T, UNKNOWN
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
pub enum Field {
    EXIST, NONE, UNKNOWN
}

pub fn ratio_to_position(display_width: u32, display_height: u32, ratio: (f32, f32, f32, f32)) -> (usize, usize, usize, usize) {
    let lower_x = display_width as f32 / ratio.0;
    let higher_x = display_width as f32 / ratio.1;
    let lower_y = display_height as f32 / ratio.2;
    let higher_y = display_height as f32 / ratio.3;

    return (lower_x as usize, higher_x as usize, lower_y as usize, higher_y as usize);
}