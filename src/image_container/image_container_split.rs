use crate::prelude::*;
use anyhow::{anyhow, Result};
use opencv::boxed_ref::BoxedRef;


/// Structure that contains a rectangular split that starts from an ImageContainer
#[derive(Debug)]
pub struct ImageContainerSplit<'a> {
    pub image: BoxedRef<'a, Mat>,
    pub x_start: i32,
    pub y_start: i32,
    pub height: i32,
    pub width: i32,
}



impl ImageContainerSplit<'_> {

    pub fn split(
        &self,
        direction: CutDirection,
        split_at: i32,
    ) -> Result<[ImageContainerSplit<'_>; 2]> {

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

    pub fn are_neighbors(a: &Self, b: &Self) -> bool{
        
        fn overlap(a_start: i32, a_len: i32, b_start: i32, b_len: i32) -> bool{
            let a_end = a_start + a_len;
            let b_end = b_start + b_len;
            
            let no_overlap = a_end < b_start || b_end < a_start;
            return !no_overlap;
        }

        fn touch(a_start: i32, a_len: i32, b_start: i32, b_len: i32) -> bool{
            let a_end = a_start + a_len;
            let b_end = b_start + b_len;
            
            return a_start == b_end || b_start == a_end;
        }

        //      ■
        //  ■   ■
        //  ■
        let horizontal_overlap = overlap(a.y_start,a.height,b.y_start,b.height);
        let horizontal_touch = touch(a.x_start, a.width, b.x_start, b.width);
        //    ■■■
        //  ■■■■
        let vertical_overlap = overlap(a.x_start, a.width, b.x_start, b.width);
        let vertical_touch = touch(a.y_start,a.height,b.y_start,b.height);
        
        return (horizontal_touch && horizontal_overlap) ||
               (vertical_touch && vertical_overlap)
    }
}
