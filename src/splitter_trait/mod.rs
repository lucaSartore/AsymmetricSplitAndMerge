use crate::prelude::*;

mod blind_splitter;
pub use blind_splitter::BlindSplitter;

/// trait that can represent different splitting strategies
pub trait SplitterTrait: Sync + 'static{
    fn split(&self, image: &impl MatTrait) -> Option<(CutDirection, i32)>;
}
