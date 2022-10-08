pub const ORIGINAL_FILENAME: &str = "tetris/original.jpg";
pub const TET_FILENAME: &str = "tetris/sprint.jpg";

// マス数
pub const CELL_WIDTH: usize = 10;
pub const CELL_HEIGHT: usize = 20;

pub struct PositionRatio {
    pub lower_x_ratio: f32,
    pub upper_x_ratio: f32,
    pub lower_y_ratio: f32,
    pub upper_y_ratio: f32
}

// 画面サイズをこの数値で割るとテトリス盤面の矩形の端を示す比率
pub const FIELD_RATIO: PositionRatio = PositionRatio {
    lower_x_ratio: 6.2, 
    upper_x_ratio: 2.87, 
    lower_y_ratio: 6.75, 
    upper_y_ratio: 1.22
}; 

// 画面サイズをこの数値で割るとネクスト1〜5の領域の端を示す比率
pub const NEXT1_POS_RATIO: PositionRatio = PositionRatio {
    lower_x_ratio: 2.75, 
    upper_x_ratio: 2.34, 
    lower_y_ratio: 6.35, 
    upper_y_ratio: 4.4
};
pub const NEXT2_POS_RATIO: PositionRatio = PositionRatio {
    lower_x_ratio: 2.75, 
    upper_x_ratio: 2.4, 
    lower_y_ratio: 3.85, 
    upper_y_ratio: 3.13
};
pub const NEXT3_POS_RATIO: PositionRatio = PositionRatio {
    lower_x_ratio: 2.75, 
    upper_x_ratio: 2.4, 
    lower_y_ratio: 2.8, 
    upper_y_ratio: 2.45
};
pub const NEXT4_POS_RATIO: PositionRatio = PositionRatio {
    lower_x_ratio: 2.75, 
    upper_x_ratio: 2.4, 
    lower_y_ratio: 2.25, 
    upper_y_ratio: 2.0
};
pub const NEXT5_POS_RATIO: PositionRatio = PositionRatio {
    lower_x_ratio: 2.75, 
    upper_x_ratio: 2.4, 
    lower_y_ratio: 1.86, 
    upper_y_ratio: 1.68
};

pub const NEXT_RATIOS: [PositionRatio; 5] = [
    NEXT1_POS_RATIO, 
    NEXT2_POS_RATIO, 
    NEXT3_POS_RATIO, 
    NEXT4_POS_RATIO, 
    NEXT5_POS_RATIO
];

// ミノの基準 HSV
pub struct MinoHSV {
    pub kind: Mino,
    pub h: u16,
    pub s: u16,
    pub v: u16
}
pub const MINO_HSV_LIST: [MinoHSV; 7] = [
    MinoHSV { kind: Mino::S, h: 100, s: 190, v: 180 },
    MinoHSV { kind: Mino::Z, h: 0, s: 220, v: 130 },
    MinoHSV { kind: Mino::L, h: 25, s: 250, v: 240 },
    MinoHSV { kind: Mino::J, h: 210, s: 250, v: 170 },
    MinoHSV { kind: Mino::I, h: 200, s: 240, v: 200 },
    MinoHSV { kind: Mino::T, h: 290, s: 175, v: 150 },
    MinoHSV { kind: Mino::O, h: 45, s: 215, v: 200 },
];
// 許容する HSV の誤差範囲
pub const ALLOW_H_GAP: u16 = 10;
pub const ALLOW_S_GAP: u16 = 50;
pub const ALLOW_V_GAP: u16 = 50;

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
pub enum CellState {
    EXIST, NONE, UNKNOWN
}

