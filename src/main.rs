mod quantize;
// use image::{GenericImageView, Pixel};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("Not enough arguments: cq <image> <depth>");
    }

    let image = &args[1];
    let depth: i8 = args[2].parse().unwrap();

    let current_dir = env::current_dir().unwrap().display().to_string();
    let path = format!("{}/{}", current_dir, image);
    println!("Reading image: {}", path);
    let img = image::open(path).unwrap();
    println!("Opened");

    let colors = quantize::quantize(&img, depth);

    println!("Palette colors: {}", colors.len());
    for color in colors {
        println!(
            "rgb({}, {}, {}) appears {} times",
            color.1, color.2, color.3, color.0
        );
    }
}
