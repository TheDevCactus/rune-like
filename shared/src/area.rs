use crate::{collision_bitmask::CollisionBitMask, entity::Entity};

pub type AreaSize = u8;

pub struct Area {
    pub board: Vec<Vec<CollisionBitMask>>,
    pub entities: Vec<Entity>,
    pub items: Vec<Entity>,
}

impl Area {
    pub fn new() -> Self {
        let mut board = Vec::new();
        for y in 0..AreaSize::MAX {
            let mut row = Vec::new();
            for x in 0..AreaSize::MAX {
                row.push(CollisionBitMask::None);
            }
            board.push(row);
        }
        Self {
            board: board,
            entities: vec![],
            items: vec![],
        }
    }
}
