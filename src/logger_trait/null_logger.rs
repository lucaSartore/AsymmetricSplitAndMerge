use super::LoggerTrait;

/// a logger that dose nothing (and also don't impact performance..)
pub struct NullLogger{}
impl LoggerTrait for NullLogger {
    fn log_split(&mut self, _direction: super::CutDirection, _split_at: i32) -> anyhow::Result<()> {
        Ok(())
    }

    fn log_merge(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn finalize_log(&mut self) { }
}

