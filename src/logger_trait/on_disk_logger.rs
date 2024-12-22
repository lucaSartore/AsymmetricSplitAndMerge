use super::*;
use opencv::videoio::{VideoWriter, VideoWriterProperties};

pub struct OnDiskLogger {
    video_writer: VideoWriter,
    image_logger: ImageLogger,
}


impl OnDiskLogger {
    pub fn new(image: Mat, output_path: &str) -> Result<Self> {
        // Define the codec and create VideoWriter object
        let fourcc = VideoWriter::fourcc('M', 'J', 'P', 'G')?;

        let mut video_writer = VideoWriter::new(
            output_path,
            fourcc,
            15.,
            image.size()?,
            true,
        )?;

        // Set the quality (0-100)
        video_writer.set(VideoWriterProperties::VIDEOWRITER_PROP_QUALITY as i32, 95.0)?;

        return Ok(Self {
            image_logger: ImageLogger::new(image),
            video_writer,
        });
    }
}

impl LoggerTrait for OnDiskLogger{
    fn log_split(&mut self, area_to_split_id: usize, splits: [Area;2]) -> Result<()> {
        self.image_logger.log_split(area_to_split_id, splits)?;
        self.video_writer.write(&self.image_logger.get_mat_ref())?;
        return Ok(());
    }

    fn log_merge(&mut self, new_item_id: usize, to_merge: [usize;2]) -> Result<()> {
        self.image_logger.log_merge(new_item_id, to_merge)?;
        self.video_writer.write(&self.image_logger.get_mat_ref())?;
        return Ok(());
    }

    fn finalize_log(&mut self) -> Result<()>{
        // leave the last frame on screen for a bit
        for _ in 0..100{
            self.video_writer.write(&self.image_logger.get_mat_ref())?;
        }
        self.video_writer.release()?;
        return Ok(())
    }
}

