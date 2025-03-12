mod color;
mod config;
mod data;
mod quantize;
mod yaml;
use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let image = args.pop().expect("No image provided");

    let config = config::parse_config(args);

    let current_dir = env::current_dir().unwrap().display().to_string();
    let path = format!("{}/{}", current_dir, image);
    println!("Reading image: {}", path);
    let img = image::open(path).unwrap();
    println!("Opened");

    let mut colors = quantize::quantize(&img, config.depth);
    let accent = data::get_accent(&mut colors, config.colorful_threshold);
    let mut color_map = data::map_colors(colors, &config);
    let base16 = data::create_palette(&mut color_map, accent, &config);

    println!("Generated scheme: ");
    for color in base16.iter().enumerate() {
        if let Some(c) = color.1 {
            println!("Color rgb #{} #{:02X}{:02X}{:02X}", color.0, c.0, c.1, c.2);
        } else {
            println!("No color number {}", color.0);
        }
    }
}
