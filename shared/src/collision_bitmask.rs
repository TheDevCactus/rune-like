const TOP: u8 = 0b00000001;
const BOTTOM: u8 = 0b00000010;
const LEFT: u8 = 0b00000100;
const RIGHT: u8 = 0b00001000;
const SQUARE: u8 = 0b00001111;
const NONE: u8 = 0b00000000;
const DIAGONAL_LEFT: u8 = 0b00010000;
const DIAGONAL_RIGHT: u8 = 0b00100000;

pub enum CollisionBitMask {
    Top,
    Bottom,
    Left,
    Right,
    Square,
    None,
    DiagonalLeft,
    DiagonalRight,
}

impl From<u8> for CollisionBitMask {
    fn from(value: u8) -> Self {
        match value {
            TOP => CollisionBitMask::Top,
            BOTTOM => CollisionBitMask::Bottom,
            LEFT => CollisionBitMask::Left,
            RIGHT => CollisionBitMask::Right,
            SQUARE => CollisionBitMask::Square,
            NONE => CollisionBitMask::None,
            DIAGONAL_LEFT => CollisionBitMask::DiagonalLeft,
            DIAGONAL_RIGHT => CollisionBitMask::DiagonalRight,
            _ => panic!("Invalid CollisionBitMask"),
        }
    }
}

impl Into<u8> for CollisionBitMask {
    fn into(self) -> u8 {
        match self {
            CollisionBitMask::Top => TOP,
            CollisionBitMask::Bottom => BOTTOM,
            CollisionBitMask::Left => LEFT,
            CollisionBitMask::Right => RIGHT,
            CollisionBitMask::Square => SQUARE,
            CollisionBitMask::None => NONE,
            CollisionBitMask::DiagonalLeft => DIAGONAL_LEFT,
            CollisionBitMask::DiagonalRight => DIAGONAL_RIGHT,
        }
    }
} 