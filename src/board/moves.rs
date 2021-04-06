use super::piece::*;

#[derive(Debug)]
pub struct Move {
    start: i32,
    target: i32,
    promotion: Option<Kind>, //FIXME: `Kind` allows some impossible promotions
                             // is_capture: bool
}

// static DIRECTION_OFFSETS : [i32; 8] = [8, -8, -1, 1, 7, -7, 9, -9];
//

impl Move {
    pub fn new(start: i32, target: i32) -> Self {
        Move {
            start,
            target,
            promotion: None,
        }
    }

    pub fn new_with_promotion(start: i32, target: i32, promotion: Option<Kind>) -> Self {
        Move {
            start,
            target,
            promotion,
        }
    }
}
