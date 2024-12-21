use crate::prelude::*;

mod null_logger;
pub use null_logger::NullLogger;

mod image_logger;
pub use image_logger::ImageLogger;

mod on_screen_logger;
pub use on_screen_logger::OnScreenLogger;

/// trait used to log the behaviour of the split and merge algorithm, there can be different
/// implementations including one that show the progress in rial time, one that save the result on
/// a small video, an option that dose nothing and dose not impact performances eccetera
pub trait LoggerTrait{
    fn log_split(&mut self, area_to_split_id: usize, splits: [Area;2]) -> Result<()>;
    fn log_merge(&mut self, new_item_id: usize, to_merge: [usize;2]) -> Result<()>;
    fn finalize_log(&mut self);
}
