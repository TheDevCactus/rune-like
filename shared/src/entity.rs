use crate::area::AreaSize;

pub struct Entity {
  pub x: AreaSize,
  pub y: AreaSize,
}

impl Entity {
  pub fn new() -> Self {
    Self { x: 0, y: 0 }
  }
}