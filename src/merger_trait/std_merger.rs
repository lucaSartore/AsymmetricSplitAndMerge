
use opencv::core::{bitwise_or, mean_std_dev, no_array, Scalar};

use super::*;

pub struct StdMerger {
    std_threshold: f64,
}

impl StdMerger {
    pub fn new(std_threshold: f64) -> Self{
        Self{
            std_threshold,
        }
    }
}

impl MergerTrait for StdMerger {
    fn merge(&self, mask_a: &Mat, mask_b: &Mat, image: &Mat) -> bool {
        
        let mut mask = Mat::default();
        bitwise_or(mask_a, mask_b, &mut mask, &no_array())
            .expect("error in std merger");

        let mut mean = Scalar::default();
        let mut std = Scalar::default();
        mean_std_dev(
            &image,
            &mut mean,
            &mut std,
            &mask,
        ).expect("eror in huestd splitter");


        // dbg!(std);

        let distance = std.as_slice()
            .iter().map(|x| x.powi(2))
            .sum::<f64>()
            .sqrt();

        return distance < self.std_threshold;
    }
    
}
