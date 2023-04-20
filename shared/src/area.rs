type AreaSize = u8;

pub enum CollisionBitMask {
  Top = 0b00000001,
  Bottom = 0b00000010,
  Left = 0b00000100,
  Right = 0b00001000,
  Square = 0b00001111,
  None = 0b00000000,
  DiagonalLeft = 0b00010000,
  DiagonalRight = 0b00100000,
}

pub struct Entity {
  pub x: AreaSize,
  pub y: AreaSize,
}

pub struct Area {
    pub board: Vec<Vec<CollisionBitMask>>,
    pub entities: Vec<Entity>,
    pub items: Vec<Entity>,
}
