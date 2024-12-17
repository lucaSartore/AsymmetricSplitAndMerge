use std::marker::PhantomData;

use anyhow::Result;
use opencv::boxed_ref::BoxedRef;
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

    pub fn to_image_container_split<'a>(&'a self) -> Result<ImageContainerSplit<'a>> {
        return Ok(
            ImageContainerSplit{
                image: self.image.row_bounds(0, self.height)?,
                x_start: 0,
                y_start: 0,
                height: 0,
                width: 0
            }
        )
    }
}

/// Structure that contains a rectangular split that starts from an ImageContainer
pub struct ImageContainerSplit<'a> {
    // lifetime: PhantomData<&'a ()>,
    image: BoxedRef<'a, Mat>,
    x_start: i32,
    y_start: i32,
    height: i32,
    width: i32,
}

#[derive(Eq,PartialEq,Debug)]
pub enum SplitDirection {
    Horizontal,
    Vertical,
}


impl<'b> ImageContainerSplit<'b>{
    fn split(
        &'b self,
        direction: SplitDirection,
        split_at: i32,
    ) -> Result<[ImageContainerSplit<'b>; 2]> {
        let [a, b] = match direction {
            SplitDirection::Vertical => [
                self.image.row_bounds(0, split_at)?,
                self.image.row_bounds(split_at + 1, self.height)?,
            ],
            SplitDirection::Horizontal => [
                self.image.col_bounds(0, split_at)?,
                self.image.col_bounds(split_at + 1, self.width)?,
            ],
        };
        
        let v_split = direction == SplitDirection::Vertical;
        let h_split = !v_split;

        return Ok([
            ImageContainerSplit {
                image: a,
                x_start: self.x_start,
                y_start: self.y_start,
                height: if v_split {self.height} else {split_at},
                width: if h_split {self.width} else {split_at},
            },
            ImageContainerSplit {
                image: b,
                x_start: self.x_start + if h_split {0} else {split_at+1},
                y_start: self.x_start + if v_split {0} else {split_at+1},
                height: if v_split {self.height} else {self.height-split_at},
                width: if h_split {self.width} else {self.width-split_at},
            },
        ]);
    }
}

