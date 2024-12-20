use crate::prelude::*;
use opencv::imgcodecs::{imread, ImreadModes};

/// Structure that contains the image that is been analyzed and split;
#[derive(Debug)]
pub struct ImageContainer {
    pub image: Mat,
    pub height: i32,
    pub width: i32,
}

impl ImageContainer {
    pub fn new_from_file(path: &str, mode: ImreadModes) -> Result<Self> {
        let mat = imread(path, mode.into())?;
        let size = mat.size()?;
        Ok(ImageContainer {
            image: mat,
            height: size.height,
            width: size.width,
        })
    }

    pub fn new_from_file_color(path: &str) -> Result<Self> {
        Self::new_from_file(path, ImreadModes::IMREAD_COLOR)
    }

    pub fn to_image_container_split<'a>(&'a self) -> ImageContainerSplit<'a> {
        return ImageContainerSplit {
            image: self.image.row_bounds(0, self.height).expect("height should always be small enough"),
            x_start: 0,
            y_start: 0,
            height: self.height,
            width: self.width,
        };
    }
}
