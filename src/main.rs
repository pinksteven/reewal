mod color;
mod config;
mod data;
mod quantize;
mod yaml;
use std::env;

fn handle_path(path: String) -> String {
    if path.starts_with("/") || path.starts_with("\\") {
        path
    } else {
        let current_dir = std::env::current_dir().unwrap().display().to_string();
        format!("{}/{}", current_dir, path)
    }
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Insufficient arguments provided. An image and save file locations required");
    }
    let mut save = args.pop().expect("No save location provided");
    let image = args.pop().expect("No image provided");

    let config = config::parse_config(args);

    let path = handle_path(image);
    println!("Reading image: {}", path);
    let img = image::open(path).expect("Could not open image");
    println!("Opened");

    println!("Parsing image data");
    let mut colors = quantize::quantize(&img, config.depth);
    println!("Generating palette");
    let accent = data::get_accent(&mut colors, config.vibrancy);
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
    save = handle_path(save);
    println!("Saving scheme to {}", save);
    yaml::export_scheme(&save, &base16);
}
