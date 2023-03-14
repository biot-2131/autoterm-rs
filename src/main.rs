// add some descriptive and help full comments!

#![allow(unused)]

mod print_screen;
use crate::print_screen::print_screen;

mod opencv_module;
use crate::opencv_module::*;

type CvImage = opencv::prelude::Mat;
type CvMat = opencv::core::Mat;

fn main() {

    // let image_path: String = print_screen(); 
    // println!("image_path = {}", image_path);

    let templ_path = String::from("assets/terminal_top_left.png");
    println!("templ_path = {}", templ_path);
    let templ = load_image_path(&templ_path);
    println!("templ = {:?}", templ);
    // show_image(&templ);
    let templ_gray = convert_to_gray(&templ);
    // show_image(&templ_gray);

    let image_path = String::from("assets/desktop.png");
    println!("image_path = {}", image_path);
    let image = load_image_path(&image_path);
    println!("image = {:?}", image);
    // show_image(&image);
    let image_gray = convert_to_gray(&image);
    // show_image(&image_gray);

    find_waldo();

}
