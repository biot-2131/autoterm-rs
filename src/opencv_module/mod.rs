use anyhow::Result;
// use opencv::prelude::*;
// use opencv::imgcodecs;
//use opencv:: {prelude::*, highgui, imgcodecs, imgproc};

use opencv::{
    core::*,
	highgui,
    imgproc,
    imgproc::{match_template, rectangle},
    imgcodecs::{imread, IMREAD_UNCHANGED},
};

type CvImage = opencv::prelude::Mat;
type CvMat = opencv::core::Mat;

pub fn convert_to_gray(image: &CvImage) -> CvImage {
    let mut img_gray = Mat::default();
    imgproc::cvt_color(&image, &mut img_gray, imgproc::COLOR_BGR2GRAY, 0);
    return img_gray;
}

pub fn show_image(image: &CvImage) -> Result<()> {
    // println!("image = {:?}", &image);
    loop {
        highgui::imshow("Image", &image)?;
        let key = highgui::wait_key(1)?;
        if key == 113 { // 113 = 'q'
            break;
        }
    }
    Ok(())
}

pub fn load_image_path(image_path: &str) -> CvImage {
	let image = opencv::imgcodecs::imread(
		image_path,
		opencv::imgcodecs::IMREAD_COLOR, // https://docs.rs/opencv/0.53.1/opencv/imgcodecs/enum.ImreadModes.html
	)
	.expect("Should error here but doesn't");

    // println!("image = {:?}", &image);
	// If the image cannot be read (because of missing file, improper permissions,
	// unsupported or invalid format), the function returns an empty matrix.
	if image.cols() == 0 && image.rows() == 0 {
		panic!("Image loaded but zero valued columns and rows mean error reading");
	}
	return image;
}

pub fn find_waldo() {
    // a_04_find_waldo_template_search
    let window_name = "find waldo";
    let template = imread("assets/waldo.jpg", IMREAD_UNCHANGED).unwrap();
    let mut img = imread("assets/WaldoBeach.jpg", IMREAD_UNCHANGED).unwrap();
    let mut res = Mat::default();
    match_template(&img, &template, &mut res, 0, &no_array()).unwrap();
    let mut max_loc = Point2i::new(0, 0);
    min_max_loc(&res, None, None, Some(&mut max_loc), None, &no_array()).unwrap();
    println!("{:?}", max_loc);
    rectangle(&mut img, Rect2i::new(max_loc.x, max_loc.y, template.rows(), template.cols()), VecN::<f64, 4>::new(0.0, 0.0, 0.0, 255.0), 4, 0, 0).unwrap(); 
    highgui::named_window(window_name, 0).unwrap();
    highgui::imshow(window_name, &mut img).unwrap();
    highgui::wait_key(0).unwrap();
}

