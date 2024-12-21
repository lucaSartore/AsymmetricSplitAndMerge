use crate::prelude::*;
mod test_util;
pub use test_util::*;


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


#[test]
fn test_correct_splitting(){

    let i = ImageContainer::new_from_file_color("./test_images/200x100_split.jpg").expect("test file must be present");

    struct TestLogger{image: Mat}
    impl LoggerTrait for TestLogger {
        fn log_split(&mut self, area_to_split_id: usize, splits: [Area;2]) -> Result<()> {
            let [mut a1, mut a2] = splits;
            let mask_a1 = a1.get_mat_area(&self.image);
            let mask_a2 = a2.get_mat_area(&self.image);

            let result = check_mask_correct_split(mask_a1, mask_a2).expect("check correct split has failed");
    
            assert!(result, "bad split detected!");

            return Ok(());
        }

        fn log_merge(&mut self, to_merge: [usize;2]) -> Result<()> { Ok(())}

        fn finalize_log(&mut self) { }
    }
        
    let splitter = splitter_traits::BlindSplitter::new(50);
    let merger = merger_traits::BlindMerger::new();
    let logger = TestLogger{image: i.image.clone()};

    let logic = MainLogic::new(splitter, merger, logger, &i);

    let _ = logic.execute_split(4);
}
