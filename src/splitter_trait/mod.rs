use opencv::core::MatTrait;
use crate::image_container::CutDirection;
/// trait that can represent different splitting strategies
pub trait SplitterTrait{
    fn split(image: &impl MatTrait) -> Option<(CutDirection, i32)>;
}
