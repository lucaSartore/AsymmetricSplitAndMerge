mod blind_splitter;
pub use blind_splitter::BlindSplitter;

use opencv::core::MatTrait;
use crate::image_container::CutDirection;
/// trait that can represent different splitting strategies
pub trait SplitterTrait: Sync + 'static{
    fn split(&self, image: &impl MatTrait) -> Option<(CutDirection, i32)>;
}
