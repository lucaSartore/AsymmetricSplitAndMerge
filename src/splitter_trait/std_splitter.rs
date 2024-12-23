
use opencv::core::{mean_std_dev, no_array, Scalar};

use super::*;

pub struct StdSplitter {
    std_threshold: f64,
    blind_splitter: BlindSplitter
}

impl StdSplitter {
    pub fn new(min_split_size: i32, std_threshold: f64) -> Self{
        Self{
            std_threshold,
            blind_splitter: BlindSplitter::new(min_split_size)
        }
    }
}

impl SplitterTrait for StdSplitter {
    fn split(&self, image: &Mat) -> Option<(CutDirection, i32)> {

        let mut mean = Scalar::default();
        let mut std = Scalar::default();
        mean_std_dev(
            &image,
            &mut mean,
            &mut std,
            &no_array(),
        ).expect("eror in huestd splitter");


        dbg!(std);

        let distance = std.as_slice()
            .iter().map(|x| x.powi(2))
            .sum::<f64>()
            .sqrt();

        dbg!(distance);

        dbg!(distance > self.std_threshold);

        if distance > self.std_threshold{
            return self.blind_splitter.split(image)
        } else {
            return None
        }
    }
}
