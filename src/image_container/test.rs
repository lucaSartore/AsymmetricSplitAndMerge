use super::*;
use opencv::highgui::{imshow, wait_key};


fn verify_consistency(v: &ImageContainerSplit){
    assert_eq!(v.image.size().expect("should't fail").width ,v.width);
    assert_eq!(v.image.size().expect("should't fail").height ,v.height);
}


#[test]
fn test_cut_errors(){
    let i = ImageContainer::new_from_file_color("./test_images/200x100_split.jpg").expect("test file must be present");
    let i = i.to_image_container_split();

    let _ = i.split(CutDirection::CutParallelToY, 200).expect_err("this split should fail");
    let _ = i.split(CutDirection::CutParallelToY, 199).expect("this split should not fail");

    let _ = i.split(CutDirection::CutParallelToX, 100).expect_err("this split should fail");
    let _ = i.split(CutDirection::CutParallelToX, 99).expect("this split should not fail");
}

#[test]
fn test_split_parallel_to_y() {
    let i = ImageContainer::new_from_file_color("./test_images/200x100_split.jpg").expect("test file must be present");
    let i = i.to_image_container_split();

    let [c1,c2] = i.split(CutDirection::CutParallelToY, 150).expect("this split should not fail");

    verify_consistency(&c1);
    verify_consistency(&c2);

    assert_eq!(c1.height,100);
    assert_eq!(c2.height,100);

    assert_eq!(c1.width,150);
    assert_eq!(c2.width,50);

    assert_eq!(c1.y_start,0);
    assert_eq!(c2.y_start,0);

    assert_eq!(c1.x_start,0);
    assert_eq!(c2.x_start,150);

    let [c3,c4] = c2.split(CutDirection::CutParallelToY, 25).expect("this split should not fail");

    verify_consistency(&c3);
    verify_consistency(&c4);

    assert_eq!(c3.x_start,150);
    assert_eq!(c4.x_start,175);
}

#[test]
fn test_split_parallel_to_x() {
    let i = ImageContainer::new_from_file_color("./test_images/200x100_split.jpg").expect("test file must be present");
    let i = i.to_image_container_split();

    let [c1,c2] = i.split(CutDirection::CutParallelToX, 60).expect("this split should not fail");

    verify_consistency(&c1);
    verify_consistency(&c2);

    assert_eq!(c1.width,200);
    assert_eq!(c2.width,200);

    assert_eq!(c1.height,60);
    assert_eq!(c2.height,40);

    assert_eq!(c1.x_start,0);
    assert_eq!(c2.x_start,0);

    assert_eq!(c1.y_start,0);
    assert_eq!(c2.y_start,60);

    let [c3,c4] = c2.split(CutDirection::CutParallelToX, 25).expect("this split should not fail");

    verify_consistency(&c3);
    verify_consistency(&c4);

    assert_eq!(c3.y_start,60);
    assert_eq!(c4.y_start,85);
}
