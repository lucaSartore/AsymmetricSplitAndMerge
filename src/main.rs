mod image_container;
mod logger_trait;
mod main_logic;
mod merger_trait;
mod splitter_trait;
mod prelude;
use opencv::highgui::{imshow, wait_key};
use prelude::*;

use env_logger;

fn main() {
    
    // env_logger::Builder::new()
    //     .filter_level(log::LevelFilter::Trace)
    //     .init();


    let i = ImageContainer::new_from_file_color("./test_images/200x100_split.jpg").expect("test file must be present");

    let splitter = splitter_traits::BlindSplitter::new(50);
    let merger = merger_traits::BlindMerger::new();
    let logger = logger_traits::ImageLogger::new(i.image.clone());

    let logic = MainLogic::new(splitter, merger, logger, &i);

    let _ = logic.execute_split(4);
}
