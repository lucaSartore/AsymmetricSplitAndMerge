use opencv::{
    core::{self, VecN}, imgproc, prelude::*, types, Error
};
/// verify that mask 1 and mask 2 are correctly split by verifying the following two property
/// 1) num_pixel( intersection(area_1, area_2)) == 0
/// 2) num_pixel(convex_hull(union(area_1,area_2))) == num_pixel(area_1) + num_pixel(area_2)
pub fn check_mask_correct_split(mask1: &Mat, mask2: &Mat) -> Result<bool, Error> {
    // Convert masks to 8UC1 if they aren't already
    let mut mask1_8u = Mat::default();
    let mut mask2_8u = Mat::default();
    mask1.convert_to(&mut mask1_8u, core::CV_8UC1, 1.0, 0.0)?;
    mask2.convert_to(&mut mask2_8u, core::CV_8UC1, 1.0, 0.0)?;

    // Check sizes match
    if mask1_8u.size()? != mask2_8u.size()? {
        return Err(Error::new(core::StsError, "Mask sizes don't match"));
    }

    // Property 1: Check intersection is empty
    let mut intersection = Mat::default();
    core::bitwise_and(&mask1_8u, &mask2_8u, &mut intersection, &Mat::default())?;


    let intersection_pixels = core::count_non_zero(&intersection)?;
    let property1 = intersection_pixels == 0;

    // Property 2: Check convex hull area equals sum of individual areas
    // First, create union of masks
    let mut union = Mat::default();
    core::bitwise_or(&mask1_8u, &mask2_8u, &mut union, &Mat::default())?;

    // Find contours of the union
    let mut contours = types::VectorOfVectorOfPoint::new();
    imgproc::find_contours(
        &union,
        &mut contours,
        imgproc::RETR_EXTERNAL,
        imgproc::CHAIN_APPROX_SIMPLE,
        core::Point::new(0, 0),
    )?;

    assert_ne!(contours.len(), 0, "non valid images as there is no contourns");

    // Get convex hull
    let mut hull = Mat::default();
    let mut points = types::VectorOfPoint::new();
    for i in 0..contours.len() {
        points.extend(contours.get(i)?.iter());
    }
    imgproc::convex_hull(&points, &mut hull, false, true)?;

    // Draw convex hull on empty mask
    let mut hull_mask = Mat::new_rows_cols_with_default(
        mask1_8u.rows(),
        mask1_8u.cols(),
        core::CV_8UC1,
        core::Scalar::all(0.0),
    )?;

    let hull_points: Vec<core::Point> = hull.iter::<VecN<i32,2>>()?.map(|p| core::Point::new(p.1[0], p.1[1])).collect();

    let hull_contour = types::VectorOfPoint::from_iter(hull_points);

    imgproc::fill_poly(
        &mut hull_mask,
        &types::VectorOfVectorOfPoint::from_iter([hull_contour]),
        core::Scalar::new(255.0, 0.0, 0.0, 0.0),
        imgproc::LINE_8,
        0,
        core::Point::new(0, 0),
    )?;

    // Count pixels
    let hull_pixels = core::count_non_zero(&hull_mask)?;
    let mask1_pixels = core::count_non_zero(&mask1_8u)?;
    let mask2_pixels = core::count_non_zero(&mask2_8u)?;
    
    let property2 = hull_pixels == mask1_pixels + mask2_pixels;


    // imshow("mask 1", &mask1_8u)?;
    // imshow("mask_2", &mask2_8u)?;
    // imshow("intersection", &intersection)?;
    // imshow("convex_hull", &hull_mask)?;
    // imshow("union", &union)?;
    // println!("{} {}",property1, property2);
    // wait_key(0)?;

    Ok(property1 && property2)
}
