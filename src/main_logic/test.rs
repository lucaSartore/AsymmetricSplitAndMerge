use crate::prelude::*;
#[test]
fn test_split_simple(){
let i = ImageContainer::new_from_file_color("./test_images/200x100_split.jpg").expect("test file must be present");
    let i = i.to_image_container_split();

    let _ = i.split(CutDirection::CutParallelToY, 200).expect_err("this split should fail");
    let _ = i.split(CutDirection::CutParallelToY, 199).expect("this split should not fail");

    let _ = i.split(CutDirection::CutParallelToX, 100).expect_err("this split should fail");
    let _ = i.split(CutDirection::CutParallelToX, 99).expect("this split should not fail");
}


