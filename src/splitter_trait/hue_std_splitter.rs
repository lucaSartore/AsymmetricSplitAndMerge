use opencv::{
    core::{mean_std_dev, no_array, split, Scalar},
    imgproc::{cvt_color, COLOR_BGR2HSV},
};

use super::*;

pub struct HueStdSplitter {
    std_threshold: f64,
    blind_splitter: BlindSplitter
}

impl HueStdSplitter {
    pub fn new(min_split_size: i32, std_threshold: f64) -> Self{
        Self{
            std_threshold,
            blind_splitter: BlindSplitter::new(min_split_size)
        }
    }
}

impl SplitterTrait for HueStdSplitter {
    fn split(&self, image: &Mat) -> Option<(CutDirection, i32)> {
        let mut hsv = Mat::default();
        cvt_color(image, &mut hsv, COLOR_BGR2HSV, 0).expect("error in splitter trait");

        let mut hsv_split = opencv::core::Vector::<Mat>::new();
        split(&hsv, &mut hsv_split).expect("error in hue std splitter");

        let mut mean = Scalar::default();
        let mut std = Scalar::default();
        mean_std_dev(
            &hsv_split.get(0).expect("error in huestd splitter"),
            &mut mean,
            &mut std,
            &no_array(),
        ).expect("eror in huestd splitter");

        if std.as_slice()[0] > self.std_threshold {
            return self.blind_splitter.split(image)
        } else {
            return None
        }
    }
}
