use opencv::core::Scalar;
use super::*;

pub struct ColorBasedMerger {
    color_distance_threshold: f64,
    std_distance_threshold: f64,
}

impl ColorBasedMerger {
    pub fn new(color_distance_threshold: f64, std_distance_threshold: f64) -> Self {
        return Self {
            color_distance_threshold,
            std_distance_threshold,
        };
    }
}

fn calculate_masked_average(matrix: &Mat, mask: &Mat) -> Result<(Scalar, Scalar)> {
    // Verify input dimensions match
    
    if matrix.size()? != mask.size()? {
        return Err(anyhow!("size missmatch"));
    }

    // Calculate mean only for pixels where mask is non-zero
    let mut mean = Scalar::default();
    let mut std_dev = Scalar::default();

    for _ in 0..1000{
        opencv::core::mean_std_dev(&matrix, &mut mean, &mut std_dev, &mask)?;
    }

    Ok((mean, std_dev))
}

fn eucledian_distance(items: &[f64]) -> f64{
    let mut sum = 0.;
    for e in items{
        sum += e.powi(2);
    }
    return sum.sqrt()
}

impl MergerTrait for ColorBasedMerger {
    fn merge(&self, mask_a: &Mat, mask_b: &Mat, image: &Mat) -> bool {
        let (color_a,std_a) = calculate_masked_average(image, mask_a)
            .expect("matrix calculation went wrong");
        let (color_b,std_b) = calculate_masked_average(image, mask_b)
            .expect("matrix calculation went wrong");
        let delta_color = color_a - color_b;
        let delta_std = std_a - std_b;
        let delta_color = eucledian_distance(delta_color.as_slice());
        let delta_std = eucledian_distance(delta_std.as_slice());
        return delta_color < self.color_distance_threshold &&
               delta_std < self.std_distance_threshold;
    }
}
