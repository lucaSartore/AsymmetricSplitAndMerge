use crate::prelude::*;

mod blind_splitter;
pub use blind_splitter::BlindSplitter;

mod hue_std_splitter;
pub use hue_std_splitter::HueStdSplitter;

mod std_splitter;
pub use std_splitter::StdSplitter;

mod max_delta_splitter;
pub use max_delta_splitter::MaxDeltaSplitter;

/// trait that can represent different splitting strategies
pub trait SplitterTrait: Sync + 'static{
    fn split(&self, image: &Mat) -> Option<(CutDirection, i32)>;
}
