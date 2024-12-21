use super::*;

/// blind merger always choose to merge (used for testing)
pub struct BlindMerger{}
impl BlindMerger {
    pub fn new()->Self{
        return Self{}
    }
}

impl MergerTrait for BlindMerger {
    fn merge(&self, _mask_a: &UnmanagedMat, _mask_b: &UnmanagedMat, _image: &Mat) -> bool {
        return true
    }
}
