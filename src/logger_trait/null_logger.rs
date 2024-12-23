#![allow(dead_code)]
use super::LoggerTrait;
use anyhow::Result;

/// a logger that dose nothing (and also don't impact performance..)
pub struct NullLogger{}
impl NullLogger {
    pub fn new() -> Self{
        Self{}
    }
}
impl LoggerTrait for NullLogger {
    fn log_split(&mut self, _area_to_split_id: usize, _splits: [super::Area;2]) -> anyhow::Result<()> {
        Ok(())
    }

    fn log_merge(&mut self,_new_item_id: usize, _to_merge: [usize;2]) -> anyhow::Result<()> {
        Ok(())
    }

    fn finalize_log(&mut self) -> Result<()>  {
        Ok(())
    }
}

