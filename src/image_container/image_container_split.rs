use anyhow::{anyhow, Result};
use opencv::boxed_ref::BoxedRef;
use opencv::prelude::*;
use super::*;

/// Structure that contains a rectangular split that starts from an ImageContainer
#[derive(Debug)]
pub struct ImageContainerSplit<'a> {
    pub image: BoxedRef<'a, Mat>,
    pub x_start: i32,
    pub y_start: i32,
    pub height: i32,
    pub width: i32,
}

impl<'b> ImageContainerSplit<'b> {
    pub fn split(
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
