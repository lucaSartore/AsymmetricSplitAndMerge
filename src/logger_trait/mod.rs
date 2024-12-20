use crate::prelude::*;

mod null_logger;
pub use null_logger::NullLogger;

/// trait used to log the behaviour of the split and merge algorithm, there can be different
/// implementations including one that show the progress in rial time, one that save the result on
/// a small video, an option that dose nothing and dose not impact performances eccetera
pub trait LoggerTrait{
    fn log_split(&mut self, direction: CutDirection, split_at: i32) -> Result<()>;
    fn log_merge(&mut self) -> Result<()>;
    fn finalize_log(&mut self);
}
