use crate::prelude::*;

#[test]
fn test_split_simple(){
    let i = ImageContainer::new_from_file_color("./test_images/200x100_split.jpg").expect("test file must be present");

    let splitter = splitter_traits::BlindSplitter::new(10);
    let merger = merger_traits::BlindMerger::new();
    let logger = logger_traits::NullLogger::new();

    let logic = MainLogic::new(splitter, merger, logger, &i);

    let _ = logic.execute_split(4);
}


#[test]
fn test_split_image_logger(){
    let i = ImageContainer::new_from_file_color("./test_images/200x100_split.jpg").expect("test file must be present");

    let splitter = splitter_traits::BlindSplitter::new(50);
    let merger = merger_traits::BlindMerger::new();
    let logger = logger_traits::ImageLogger::new(i.image.clone());

    let logic = MainLogic::new(splitter, merger, logger, &i);

    let _ = logic.execute_split(4);
}
