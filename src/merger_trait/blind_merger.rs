use super::MergerTrait;

/// blind merger always choose to merge (used for testing)
pub struct BlindMerger{}
impl BlindMerger {
    pub fn new()->Self{
        return Self{}
    }
}

impl MergerTrait for BlindMerger {
    fn merge(&self) -> bool {
        return true
    }
}
