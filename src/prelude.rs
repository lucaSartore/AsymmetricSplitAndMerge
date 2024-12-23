#![allow(unused_imports)]
#![allow(unused_braces)]
pub use crate::image_container::*;

pub use crate::logger_trait::LoggerTrait;
pub mod logger_traits {
    pub use crate::logger_trait::{
        NullLogger,
        ImageLogger,
        OnScreenLogger,
        OnDiskLogger
    };
}

pub use crate::merger_trait::MergerTrait;
pub mod merger_traits {
    pub use crate::merger_trait::{
        BlindMerger,
        ColorBasedMerger,
        StdMerger
    };
}

pub use crate::splitter_trait::SplitterTrait;
pub mod splitter_traits {
    pub use crate::splitter_trait::{
        BlindSplitter,
        HueStdSplitter,
        StdSplitter,
        MaxDeltaSplitter
    };
}

pub use crate::main_logic::*;

pub use opencv::prelude::*;
pub use anyhow::{anyhow,Result};

pub use log::{
    info,
    warn,
    error
};
