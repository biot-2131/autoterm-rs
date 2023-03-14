// add some descriptive and help full comments!

#![allow(unused)]

mod print_screen;
use crate::print_screen::print_screen;

mod opencv_module;
use crate::opencv_module::*;

type CvImage = opencv::prelude::Mat;
type CvMat = opencv::core::Mat;

fn main() {
    println!("");

    // let image_path: String = print_screen(); 
    // println!("image_path = {}", image_path);

    let templ_path = String::from("assets/terminal_top_left.png");
    println!("templ_path = {}", templ_path);
    let templ = load_color_image(&templ_path);
    // println!("templ = {:?}", templ);
    // show_image(&templ);
    let templ_gray = convert_to_gray(&templ);
    // show_image(&templ_gray);

    let image_path = String::from("assets/desktop.png");
    println!("image_path = {}", image_path);
    let image = load_color_image(&image_path);
    // println!("image = {:?}", image);
    // show_image(&image);
    let image_gray = convert_to_gray(&image);
    // show_image(&image_gray);

    // my_match_template(templ, image);
    let max_loc = my_match_template(templ_gray, image_gray, false);
    println!("max_loc = {:?}", max_loc);
    println!("max_loc = {:?}", max_loc.x);

    // terminal 736x483 -> offset = remove window bar 1x48 -> i/o panel 734x434 
    const TERMINAL: [i32; 2] = [736, 483];
    const OFFSET: [i32; 2] = [1, 48];
    const PANEL: [i32; 2] = [735, 482];

    let xmin = max_loc.x + OFFSET[0];   // top left width : 0
    let ymin = max_loc.y + OFFSET[1];   // top left height : 0
    let xmax = max_loc.x + PANEL[0];    // bottom right width  : 1920
    let ymax = max_loc.y + PANEL[1];    // bottom right height : 1080

    let crop_array = [xmin, ymin, xmax, ymax];

    let croped_image = crop_image(&image, &crop_array, true);

}
