use opencv::{
    core::{add, no_array, reduce, Point, ReduceTypes, BORDER_DEFAULT, CV_32F},
    imgproc::{cvt_color, filter_2d, COLOR_BGR2GRAY},
};

use super::*;

pub struct HeuristicAsymmetricSplitter<T: SplitterTrait> {
    decision_splitter: T,
}


impl<T: SplitterTrait> HeuristicAsymmetricSplitter<T> {
    pub fn new(decision_splitter: T) -> Self {
        Self { decision_splitter }
    }

    fn is_split_too_asymetric(&self, cut_direction: CutDirection, cut_at: i32, image: &Mat) -> bool {

        let mut height_after_split = image.rows();
        let mut width_after_split = image.cols();

        if cut_at == 0 {
            return true
        }

        if cut_direction == CutDirection::CutParallelToX {
            if cut_at == image.rows() {
                return true
            }
            height_after_split = cut_at.min(height_after_split-cut_at)
        } else {
            if cut_at == image.cols() {
                return true
            }
            width_after_split = cut_at.min(width_after_split-cut_at)
        }

        return height_after_split > width_after_split * 7 ||
               width_after_split > height_after_split * 7
    }
}

impl<T: SplitterTrait> SplitterTrait for HeuristicAsymmetricSplitter<T> {
    fn split(&self, image: &Mat) -> Option<(CutDirection, i32)> {
        if image.rows() <= 5 || image.cols() <= 5
        {
            return self.decision_splitter.split(image);
        }

        let (original_direction, original_cut) = self.decision_splitter.split(image)?;

        let kernel_x = Mat::from_slice_2d(&[[-1], [1]]).expect("error in kernel creation");
        let kernel_x_flip = Mat::from_slice_2d(&[[1], [-1]]).expect("error in kernel creation");
        let kernel_y = Mat::from_slice_2d(&[[-1, 1]]).expect("error in kernel creation");
        let kernel_y_flip = Mat::from_slice_2d(&[[1, -1]]).expect("error in kernel creation");

        let mut derivate_mat_x_straight = Mat::default();
        let mut derivate_mat_y_straight = Mat::default();
        let mut derivate_mat_x_flip = Mat::default();
        let mut derivate_mat_y_flip = Mat::default();

        filter_2d( image, &mut derivate_mat_x_straight, 0, &kernel_x, Point::new(0, 0), 0., BORDER_DEFAULT,)
        .expect("error in partial derivate");
        filter_2d( image, &mut derivate_mat_x_flip, 0, &kernel_x_flip, Point::new(0, 0), 0., BORDER_DEFAULT,)
        .expect("error in partial derivate");

        filter_2d( image, &mut derivate_mat_y_straight, 0, &kernel_y, Point::new(0, 0), 0., BORDER_DEFAULT,)
        .expect("error in partial derivate");
        filter_2d( image, &mut derivate_mat_y_flip, 0, &kernel_y_flip, Point::new(0, 0), 0., BORDER_DEFAULT,)
        .expect("error in partial derivate");

        let mut derifate_mat_x = Mat::default();
        let mut derifate_mat_y = Mat::default();

        add(
            &derivate_mat_x_flip,
            &derivate_mat_x_straight,
            &mut derifate_mat_x,
            &no_array(),
            -1,
        )
        .expect("error in partial derivate add");
        add(
            &derivate_mat_y_flip,
            &derivate_mat_y_straight,
            &mut derifate_mat_y,
            &no_array(),
            -1,
        )
        .expect("error in partial derivate add");

        let mut derivate_mat_gray_x = Mat::default();
        let mut derivate_mat_gray_y = Mat::default();

        cvt_color(&derifate_mat_x, &mut derivate_mat_gray_x, COLOR_BGR2GRAY, 0)
            .expect("error in color converstion function");
        cvt_color(&derifate_mat_y, &mut derivate_mat_gray_y, COLOR_BGR2GRAY, 0)
            .expect("error in color converstion function");

        let len_y = image.cols();
        let len_x = image.rows();

        let mut output_x = Mat::default();
        let mut output_y = Mat::default();

        reduce(
            &derivate_mat_gray_x,
            &mut output_x,
            1,
            ReduceTypes::REDUCE_SUM.into(),
            CV_32F,
        )
        .expect("error while calculating the max value");

        reduce(
            &derivate_mat_gray_y,
            &mut output_y,
            0,
            ReduceTypes::REDUCE_SUM.into(),
            CV_32F,
        )
        .expect("error while calculating the max value");

        let output_x = (0..len_x)
            .map(|i| *output_x.at::<f32>(i).expect("error in output creation"))
            .collect::<Vec<_>>();
        let output_y: Vec<f32> = (0..len_y)
            .map(|i| *output_y.at::<f32>(i).expect("error in output creation"))
            .collect::<Vec<_>>();



        let (mut max_index_x, mut max_x) = output_x
            .into_iter()
            .enumerate()
            .max_by(|a, b| a.1.total_cmp(&b.1))
            .expect("matrix must have at least a size");
        max_x *= len_x as f32;
        max_index_x += 1;

        let (mut max_index_y,mut max_y) = output_y
            .into_iter()
            .enumerate()
            .max_by(|a, b| a.1.total_cmp(&b.1))
            .expect("matrix must have at least a size");
        max_y *= len_y as f32;
        max_index_y += 1;

        if max_x == 0. && max_y == 0. {
            return Some((original_direction, original_cut));
        }

        let (cut_direction, cut_at) = if max_x > max_y {
            (CutDirection::CutParallelToX, max_index_x as i32)
        } else {
            (CutDirection::CutParallelToY, max_index_y as i32)
        };

        if self.is_split_too_asymetric(cut_direction, cut_at, image) {
            return Some((original_direction, original_cut));
        }
        return Some((cut_direction, cut_at));
    }
}
