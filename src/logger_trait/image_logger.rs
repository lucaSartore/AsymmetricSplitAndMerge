use std::{collections::HashMap, error::Error};
use opencv::{core::{Scalar, BORDER_CONSTANT, CV_8U}, highgui::{imshow, wait_key}, imgproc::morphology_default_border_value};

use crate::prelude::*;

type Color = Scalar;

#[derive(Debug,Clone)]
pub struct ColoredArea {
    pub color: Color,
    pub area: Area,
}

impl From<Area> for ColoredArea {
   fn from(value: Area) -> Self {
       Self{
            area: value,
            color: Self::get_random_color()
        }
   } 
}

impl ColoredArea {
    pub fn new(color: Color, area: Area) -> Self {
        Self { color, area }
    }
    pub fn new_random_color(area: Area) -> Self {
        Self {
            color: Self::get_random_color(),
            area,
        }
    }
    fn get_random_color() -> Color {
        Scalar::new(255.,0.,0.,0.)
    }
    fn get_mat_area(&mut self, reference_mat: &Mat) -> &Mat {
        self.area.get_mat_area(reference_mat)
    }
}

pub struct ImageLogger {
    input_image: Mat,
    output_image: Mat,
    areas: HashMap<usize, ColoredArea>,
}

impl ImageLogger {
    pub fn new(image: Mat) -> Self {
        let size = image.size().expect("image must be valied");
        let mut areas = HashMap::new();
        areas.insert(
            0,
            ColoredArea::new_random_color(
                Area::new(0, size.height, size.width)
            ),
        );
        return ImageLogger {
            input_image: image.clone(),
            output_image: image,
            areas,
        };
    }


    fn color_area(&mut self, id: usize){
        let area = self.areas.get_mut(&id).expect("index not found");
        let color = area.color.clone();
        let mask = area.get_mat_area(&self.input_image);

        // border color
        let color = Mat::new_rows_cols_with_default(self.input_image.rows(), self.input_image.cols(), self.input_image.typ(), color).expect("matrix_creation_has_failed");
        color.copy_to_masked(&mut self.output_image, mask).expect("coloring has failed");
        let mut second_mask = Mat::default();
        
        // real image
        opencv::imgproc::erode(
            mask,
            &mut second_mask,
            & Mat::ones(4, 4, CV_8U).expect("matrix_creation_fail"),
            opencv::core::Point_ { x: -1, y: -1 },
            1,
            BORDER_CONSTANT,
            morphology_default_border_value().expect("unable to create border value")
        ).expect("erosion has failed");

        self.input_image.copy_to_masked(&mut self.output_image, &second_mask).expect("matrix copy has failed");

    }
}

impl LoggerTrait for ImageLogger {
    fn log_split(&mut self, area_to_split_id: usize, splits: [Area;2]) -> Result<()> {
        let old_area = self.areas.remove(&area_to_split_id)
            .ok_or(anyhow!("item with id{area_to_split_id} not found"))?;

        let [a1, a2] = splits;
        let [mut a1, a2]: [ColoredArea; 2] = [a1.into(), a2.into()];

        
        let a1_id = a1.area.id;
        let a2_id = a2.area.id;

        // color consistency
        a1.color = old_area.color;

        if self.areas.get(&a1_id).is_some() || self.areas.get(&a2_id).is_some(){
            return Err(anyhow!("item with specified id is already presetn"));
        }


        self.areas.insert(a1_id, a1);
        self.areas.insert(a2_id, a2);

        self.color_area(a1_id);
        self.color_area(a2_id);

        imshow("output_image", &self.output_image).unwrap();
        wait_key(0).unwrap();

        Ok(())
    }
    fn log_merge(&mut self, to_merge: [usize;2]) -> Result<()> {
        
        Ok(())
    }
    fn finalize_log(&mut self) {}
}
    