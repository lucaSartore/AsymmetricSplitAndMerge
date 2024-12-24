mod image_container;
mod logger_trait;
mod main_logic;
mod merger_trait;
mod prelude;
mod splitter_trait;
use prelude::*;

use env_logger;

fn main() {
    // env_logger::Builder::new()
    //     .filter_level(log::LevelFilter::Trace)
    //     .init();

    // let i = ImageContainer::new_from_file_color("./test_images/stuff.jpg")
    //     .expect("test file must be present");
    let i = ImageContainer::new_from_file_color("./test_images/200x100_rectangle.jpg")
        .expect("test file must be present");

    // let splitter = splitter_traits::BlindSplitter::new(50);
    // let splitter = splitter_traits::MaxDeltaSplitter::new(3,50.);
    let splitter = splitter_traits::HeuristicAsymmetricSplitter::new(
        splitter_traits::MaxDeltaSplitter::new(3,50.)
    );
    // let splitter = splitter_traits::BlindSplitter::new(10);
    // let splitter = splitter_traits::StdSplitter::new(10, 30.);
    // let merger = merger_traits::StdMerger::new(40.);
    let merger = merger_traits::ColorBasedMerger::new(70.,250.);
    // let logger = logger_traits::OnDiskLogger::new(i.image.clone(), "./out.mp4")
    //     .expect("can't create logger trait");
    let logger = logger_traits::OnScreenLogger::new(i.image.clone(), "log".into());
    // let logger = logger_traits::NullLogger::new();

     
    let logic = MainLogic::new(splitter, merger, logger, &i);

    let logic = logic.execute_split(1);
    let _ = logic.execute_merge(10);
}
