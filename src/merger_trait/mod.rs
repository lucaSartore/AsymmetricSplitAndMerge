mod blind_merger;
pub use blind_merger::BlindMerger;

use crate::prelude::*;
/// trait that can represent different merging strategies
pub trait MergerTrait: Sync + 'static{
    fn merge(&self, mask_a: &UnmanagedMat, mask_b: &UnmanagedMat, image: &Mat) -> bool;
}
