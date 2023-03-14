use anyhow::Result;
// use opencv::prelude::*;
// use opencv::imgcodecs;
//use opencv:: {prelude::*, highgui, imgcodecs, imgproc};
// use anyhow::Result;
// use opencv:: {prelude::*, core, highgui, imgproc, imgcodecs};


use opencv::{
    core::*,
    prelude::*,
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

pub fn load_color_image(image_path: &str) -> CvImage {
    // should check if file exists and is readable!
	let image = opencv::imgcodecs::imread(
		image_path,
		opencv::imgcodecs::IMREAD_COLOR,
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

pub fn my_match_template(template: CvImage, mut image: CvImage, show: bool) -> Point_<i32> {

    let mut result = Mat::default();
    match_template(&image, &template, &mut result, 0, &no_array()).unwrap();
    let mut max_loc = Point2i::new(0, 0);
    min_max_loc(&result, None, None, Some(&mut max_loc), None, &no_array()).unwrap();
    // println!("\nresult = {:?}", result);
    // println!("max_loc = {:?}", max_loc);

    if show {
        rectangle(&mut image, Rect2i::new(max_loc.x, max_loc.y, 
            template.rows(), template.cols()),
            VecN::<f64, 4>::new(0.0, 0.0, 0.0, 0.0), 
            1, 0, 0).unwrap(); 
    
    // 16:9 640:360 848:480 854:480 960:540 1024:576 1280:720 1366:768 1600:900 1920:1080
        let window_name = "Window Name";
        highgui::named_window(window_name, 0).unwrap();
        highgui::resize_window(window_name, 960, 540);
        highgui::imshow(window_name, &mut image).unwrap();
        highgui::wait_key(0).unwrap();    
    }
    
    return max_loc;
}

pub fn crop_image(image: &CvImage, crop_array: &[i32], show: bool) -> CvImage {

    let crop_image = Mat::roi(&image, opencv::core::Rect {
        x: crop_array[0],
        y: crop_array[1],
        width: crop_array[2] - crop_array[0],
        height: crop_array[3] - crop_array[1],
    }).unwrap();   

    if show {
        show_image(&crop_image);
    }

    return crop_image;
}
