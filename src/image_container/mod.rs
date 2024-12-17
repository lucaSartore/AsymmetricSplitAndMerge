use anyhow::Result;
use opencv::imgcodecs::{imread, ImreadModes};
use opencv::prelude::*;

/// Structure that contains the image that is been analyzed and split;
pub struct ImageContainer {
    image: Mat,
    height: i32,
    width: i32,
}

impl ImageContainer {
    pub fn new_from_file(path: &str, mode: ImreadModes) -> Result<Self> {
        let mat = imread(path, mode.into())?;
        let size = mat.size()?;
        Ok(ImageContainer {
            image: mat,
            height: size.height,
            width: size.width,
        })
    }
}

/// Structure that contains a rectangular split that starts from an ImageContainer
pub struct ImageContainerSplit<'a> {
    x: &'a Mat,
    x_start: i32,
    y_start: i32,
    height: i32,
    width: i32,
}

pub enum SplitDirection {
    Horizontal,
    Vertical,
}

pub trait ImageSplitter {
    fn split<'a>(
        &'a self,
        direction: SplitDirection,
        split_at: i32,
    ) -> Result<[ImageContainerSplit<'a>; 2]>;
}

impl ImageSplitter for ImageContainer {
    fn split<'a>(
        &'a self,
        direction: SplitDirection,
        split_at: i32,
    ) -> Result<[ImageContainerSplit<'a>; 2]> {
        let [a, b] = match direction {
            SplitDirection::Vertical => [
                self.image.row_bounds(0, split_at)?,
                self.image.row_bounds(split_at + 1, self.width)?
            ],
            SplitDirection::Horizontal => [
                self.image.col_bounds(0, split_at)?,
                self.image.col_bounds(split_at + 1, self.height)?
            ],
        };

        todo!();
    }
}
