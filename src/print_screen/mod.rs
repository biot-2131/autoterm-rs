// add some comments 

use screenshots::Screen;
use std::{fs};

pub fn print_screen() -> String { //-> Result<()> {
    println!("print_screen::print_screen()");

    let screens = Screen::all().unwrap();
    // println!("screens = {:?}", screens);

    let mut filename = String::from("target/");

    // Note: it is possible to get screen shots from multiple monitors by:
    // for screen in screens { ... } we are not doing that, get just the first
    // monitor i.e. is_primary: true 

    // println!("screen {screen:?}");
    let image = screens[0].capture().unwrap();
    let buffer = image.buffer();
    //println!("buffer = {:?}", buffer);

    let date_time_utc_string = date_time_utc_string();
    //println!("date_time_utc_string = {}", date_time_utc_string);

    filename.push_str(&date_time_utc_string);
    // filename.push_str("_");
    // filename.push_str(&screen.display_info.id.to_string());
    filename.push_str(".png");
    // println!("filename = {}", filename);

    fs::write(format!("{}", filename), buffer).unwrap();

    return filename;
}

use chrono::Utc;

fn date_time_utc_string() -> String {
    let date_time_utc = Utc::now();
    let date_time_str = date_time_utc.format("%Y-%m-%d_%H:%M:%S").to_string();
    //println!{"\ndate_time_functions::date_time_str = {}",date_time_str};    
    return date_time_str;
}
