use opencv::highgui::{imshow, wait_key};

use super::*;
pub struct OnStreenLogger{
    image_logger: ImageLogger,
    log_window_name: String
}

impl OnStreenLogger {
    pub fn new(image: Mat, log_window_name: String) -> Self {
        Self{
            image_logger: ImageLogger::new(image),
            log_window_name
        }
    }
}

impl LoggerTrait for OnStreenLogger {
    fn log_split(&mut self, area_to_split_id: usize, splits: [Area;2]) -> Result<()> {
        self.image_logger.log_split(area_to_split_id, splits)?;
        imshow(&self.log_window_name, self.image_logger.get_mat_ref())?;
        wait_key(0)?;
        Ok(())
    }

    fn log_merge(&mut self, to_merge: [usize;2]) -> Result<()> {
        self.image_logger.log_merge(to_merge)?;
        Ok(())
    }

    fn finalize_log(&mut self) {
    }
}