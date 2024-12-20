use crate::prelude::*;

/// BlindSplitter always decide to split up until a certain size
pub struct BlindSplitter{
    min_split_size: i32
}
impl BlindSplitter {
    pub fn new(min_split_size: i32) -> Self{
        assert!(min_split_size >= 2,"an image must be at least a 2x2 picture for split to take effect");
        Self{min_split_size}
    }
}

impl SplitterTrait for BlindSplitter {
    fn split(&self, image: &impl opencv::prelude::MatTrait) -> Option<(crate::image_container::CutDirection, i32)> {
        let size = image.size().expect("valid image must be passed");
        if size.height > size.width{
            // split the height
            if size.height < self.min_split_size {
                return None;
            }
            return Some((CutDirection::CutParallelToX, size.height/2));
        }else{
            // split the width
            if size.width < self.min_split_size {
                return None;
            }
            return Some((CutDirection::CutParallelToY, size.width/2));
        }
    } 
}
