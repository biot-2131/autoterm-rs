use opencv::prelude::*;
use opencv::imgcodecs;

pub fn hello_world() {
    println!("Hello, World!");
}

pub fn load_color_image(filename: String) {
    let img = imgcodecs::imread(&filename, imgcodecs::IMREAD_COLOR)?;
    println!("img = {:?}", img);
}
