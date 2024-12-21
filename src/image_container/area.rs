use opencv::core::{Mat, Rect, Scalar};
use opencv::imgproc::rectangle;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum AreaMarker {
    SquaredArea {
        height: i32,
        width: i32,
        y_start: i32,
        x_start: i32,
    },
    MaskedArea(Mat),
}

impl AreaMarker {
    fn get_mat_area(&mut self, reference_mat: &Mat) -> &Mat {
        self.to_maks(reference_mat);
        match &(*self) {
            AreaMarker::MaskedArea(v) => &v,
            _ => panic!("error in code... to_mask hasn't workd"),
        }
    }
    fn to_maks(&mut self, reference_mat: &Mat) {
        let (height, width, y, x) = match self {
            Self::SquaredArea {
                height,
                width,
                y_start,
                x_start,
            } => (*height, *width, *y_start, *x_start),
            _ => return,
        };

        let new_mask = Mat::zeros(
            reference_mat.rows(),
            reference_mat.cols(),
            opencv::core::CV_8U,
        )
        .expect("error in creating mat");
        let mut new_mask = new_mask.to_mat().expect("error in creating mat");

        rectangle(
            &mut new_mask,
            Rect::new(x, y, width, height),
            Scalar::new(255., 255., 255., 0.),
            -1,
            opencv::imgproc::LINE_8,
            0,
        )
        .expect("generation of rectangle has failed");

        *self = Self::MaskedArea(new_mask)
    }

    pub fn merge(area_1: &Mat, area_2: &Mat) -> Result<Self>{
        let mut result = Mat::default();
        opencv::core::bitwise_or(&area_1, &area_2, &mut result, &Mat::default())?;
        return Ok(Self::MaskedArea(result))
    }
}

impl From<&ImageContainerSplit<'_>> for AreaMarker {
    fn from(value: &ImageContainerSplit<'_>) -> Self {
        Self::SquaredArea {
            height: value.height,
            width: value.width,
            y_start: value.y_start,
            x_start: value.x_start,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Area {
    pub id: usize,
    pub marker: AreaMarker,
}

impl Area {
    pub fn new(id: usize, height: i32, width: i32) -> Self {
        return Self {
            id,
            marker: AreaMarker::SquaredArea {
                height,
                width,
                y_start: 0,
                x_start: 0,
            },
        };
    }
    
    pub fn new_from_id_and_marker(id: usize, marker: AreaMarker) -> Self{
        return Self{id,marker}
    }

    pub fn new_from_split(id: usize,split: &ImageContainerSplit<'_>) -> Self {
        return Self {
            id,
            marker: split.into()
        };
    }

    pub fn get_mat_area(&mut self, reference_mat: &Mat) -> &Mat {
        self.marker.get_mat_area(reference_mat)
    }
}
