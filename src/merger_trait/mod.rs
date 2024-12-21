mod blind_merger;
pub use blind_merger::BlindMerger;
mod color_based_merger;
pub use color_based_merger::ColorBasedMerger;


use crate::prelude::*;
/// trait that can represent different merging strategies
pub trait MergerTrait: Sync + 'static{
    fn merge(&self, mask_a: &Mat, mask_b: &Mat, image: &Mat) -> bool;
}
