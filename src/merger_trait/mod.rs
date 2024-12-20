mod blind_merger;
pub use blind_merger::BlindMerger;
/// trait that can represent different merging strategies
pub trait MergerTrait: Sync + 'static{
    fn merge(&self) -> bool;
}
