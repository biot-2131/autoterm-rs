// add some descriptive and help full comments!

#![allow(unused)]

mod print_screen;
use crate::print_screen::print_screen;

fn main() {

    let filename: String = print_screen(); 
    println!("filename = {}", filename);

}
