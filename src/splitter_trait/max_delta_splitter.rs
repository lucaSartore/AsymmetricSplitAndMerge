use opencv::{
    core::{absdiff, mean_std_dev, min_max_loc, no_array, pow, reduce, Scalar, Size, BORDER_DEFAULT}, highgui::{imshow, wait_key}, imgproc::gaussian_blur
};

use super::*;

pub struct MaxDeltaSplitter {
    delta_threshold: f64,
    blind_splitter: BlindSplitter
}

impl MaxDeltaSplitter {
    pub fn new(min_split_size: i32, delta_threshold: f64) -> Self{
        Self{
            delta_threshold,
            blind_splitter: BlindSplitter::new(min_split_size)
        }
    }
}

impl SplitterTrait for MaxDeltaSplitter {
    fn split(&self, image: &Mat) -> Option<(CutDirection, i32)> {
        
        let mut blur = Mat::default();

        // note: the image cloning here is because, if the image is a slice the
        // function will pick up items from the bordering when applying the kernel for the blur
        gaussian_blur(&image.clone(), &mut blur,Size::new(9, 9), 3., 0., BORDER_DEFAULT)
            .expect("error in gaussian_blur");

        let mut mean = Scalar::default();
        let mut std = Scalar::default();
        mean_std_dev(
            &blur,
            &mut mean,
            &mut std,
            &no_array(),
        ).expect("eror in huestd splitter");

        let average_color_mat = Mat::new_rows_cols_with_default(
            image.rows(),
            image.cols(),
            image.typ(),
            mean
        ).expect("error im mat creation");

        let mut abs_diff_mat = Mat::default();
        absdiff(&blur, &average_color_mat, &mut abs_diff_mat).expect("error in mat distance");

        
        // imshow("absdif", &abs_diff_mat).unwrap();
        // wait_key(0).unwrap();


        // Square the differences for each channel
        let mut squared_diff = Mat::default();
        pow(&abs_diff_mat, 2., &mut squared_diff)
            .expect("error in power of two");


        // imshow("pow", &squared_diff).unwrap();
        
        let mut distance_mat = Mat::default();
        reduce(
            &squared_diff,
            &mut distance_mat,
            2, // Reduce along the color channels (axis 2)
            opencv::core::REDUCE_SUM,
            opencv::core::CV_64F,
        ).expect("error in reduce");

        // Find the maximum value in the resulting matrix
        let mut min_val = 0.0;
        let mut max_val = 0.0;
        min_max_loc(&distance_mat, Some(&mut min_val), Some(&mut max_val), None, None, &no_array())
        .expect("error in min max loc");
        
        let max_distance = max_val.sqrt();
        // dbg!(max_distance);
        // dbg!(self.delta_threshold);
        // dbg!(max_distance > self.delta_threshold);

        if max_distance > self.delta_threshold{
            return self.blind_splitter.split(image)
        } else {
            return None
        }
    }
}
