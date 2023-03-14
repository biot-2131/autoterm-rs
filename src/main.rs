// add some descriptive and help full comments!

#![allow(unused)]

mod print_screen;
use crate::print_screen::print_screen;

mod opencv_module;
use crate::opencv_module::hello_world;

fn main() {

    let filename: String = print_screen(); 
    println!("filename = {}", filename);

    hello_world();

    let image_filename = String::from("target/2023-03-13_11:15:40.png");
    load_color_image(image_filename);
    //let templ_filename = String::from("target/2023-03-13_11:15:40.png");

}
