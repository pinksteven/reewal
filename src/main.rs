mod color;
mod data;
mod quantize;
mod yaml;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        panic!("Not enough arguments: cq <image> <depth>");
    }

    let image = &args[1];
    let depth: i8 = args[2].parse().unwrap();
    let yaml = &args[3];
    let threshold = 20;

    let current_dir = env::current_dir().unwrap().display().to_string();
    let path = format!("{}/{}", current_dir, image);
    println!("Reading image: {}", path);
    let img = image::open(path).unwrap();
    println!("Opened");

    let scheme = yaml::get_scheme(&format!("{}/{}", current_dir, yaml));
    let mut colors = quantize::quantize(&img, depth);
    let accent = data::get_accent(&mut colors, threshold);
    let mut color_map = data::map_colors(&scheme, colors, threshold as u16);
    let base16 = data::create_palette(&mut color_map, &scheme, accent, threshold as u16);

    println!("Generated scheme: ");
    for color in base16.iter().enumerate() {
        if let Some(c) = color.1 {
            println!("Color rgb #{} #{:02X}{:02X}{:02X}", color.0, c.0, c.1, c.2);
        } else {
            println!("No color number {}", color.0);
        }
    }

    let _test = color::compare_colors(&(0, 0, 0), &(255, 255, 255), 1.0, 1.0, 1.0);
}
