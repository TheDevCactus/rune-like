use crate::{collision_bitmask::CollisionBitMask, entity::Entity};

pub type AreaSize = u8;

pub struct Area {
    pub board: Vec<Vec<CollisionBitMask>>,
    pub entities: Vec<Entity>,
    pub items: Vec<Entity>,
}
