#[cfg(test)]
mod test;
use std::any;

use anyhow::{anyhow, Error, Result};
use opencv::boxed_ref::BoxedRef;
use opencv::imgcodecs::{imread, ImreadModes};
use opencv::prelude::*;

/// Structure that contains the image that is been analyzed and split;
#[derive(Debug)]
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

    pub fn new_from_file_color(path: &str) -> Result<Self> {
        Self::new_from_file(path, ImreadModes::IMREAD_COLOR)
    }

    pub fn to_image_container_split<'a>(&'a self) -> ImageContainerSplit<'a> {
        return ImageContainerSplit {
            image: self.image.row_bounds(0, self.height).expect("height should always be small enough"),
            x_start: 0,
            y_start: 0,
            height: self.height,
            width: self.width,
        };
    }
}

/// Structure that contains a rectangular split that starts from an ImageContainer
#[derive(Debug)]
pub struct ImageContainerSplit<'a> {
    // lifetime: PhantomData<&'a ()>,
    image: BoxedRef<'a, Mat>,
    x_start: i32,
    y_start: i32,
    height: i32,
    width: i32,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum CutDirection {
    CutParallelToX,
    CutParallelToY,
}

impl<'b> ImageContainerSplit<'b> {
    fn split(
        &'b self,
        direction: CutDirection,
        split_at: i32,
    ) -> Result<[ImageContainerSplit<'b>; 2]> {

        if split_at <= 0{
            return Err(anyhow!("split_at={} shall not be zero or negative",split_at))
        }

        let [a, b] = match (direction, split_at>=self.height, split_at>=self.width) {
            (CutDirection::CutParallelToX,true,_) => return Err(anyhow!("split_at={} is out of bound for Y axis of height={}",split_at,self.height)),
            (CutDirection::CutParallelToY,_,true) => return Err(anyhow!("split_at={} is out of bound for X axis of width={}",split_at,self.width)),
            (CutDirection::CutParallelToX,_,_) => [
                self.image.row_bounds(0, split_at)?,
                self.image.row_bounds(split_at, self.height)?,
            ],
            (CutDirection::CutParallelToY, _, _) => [
                self.image.col_bounds(0, split_at)?,
                self.image.col_bounds(split_at, self.width)?,
            ],
        };

        let y_split = direction == CutDirection::CutParallelToY;
        let x_split = !y_split;

        return Ok([
            ImageContainerSplit {
                image: a,
                x_start: self.x_start,
                y_start: self.y_start,
                height: if y_split { self.height } else { split_at },
                width: if x_split { self.width } else { split_at },
            },
            ImageContainerSplit {
                image: b,
                x_start: self.x_start + if x_split { 0 } else { split_at },
                y_start: self.y_start + if y_split { 0 } else { split_at },
                height: if y_split { self.height } else { self.height - split_at },
                width: if x_split { self.width } else { self.width - split_at },
            },
        ]);
    }
}
