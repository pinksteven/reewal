mod colors;
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
    let threshold = 10;

    let current_dir = env::current_dir().unwrap().display().to_string();
    let path = format!("{}/{}", current_dir, image);
    println!("Reading image: {}", path);
    let img = image::open(path).unwrap();
    println!("Opened");

    let scheme = yaml::get_scheme(&format!("{}/{}", current_dir, yaml));
    let mut colors = quantize::quantize(&img, depth);
    let mut grays = colors.clone();
    grays.retain(|color| !colors::is_colorful(color, 15));
    colors.retain(|color| colors::is_colorful(color, 15));

    let mut base16: [(u8, u8, u8); 16] = [(255, 0, 0); 16];
    base16[13] = colors.pop().unwrap();
    for i in 0..8 {
        let mut found = false;
        let mut min: u16 = threshold;
        let mut candidate: (u8, u8, u8) = (255, 0, 0);
        for color in grays.iter().rev() {
            let diff = colors::compare_colors(&scheme[i], color);
            if diff < min {
                min = diff;
                candidate = *color;
                found = true;
            }
        }
        if found {
            base16[i] = candidate;
        }
    }
    for i in 8..16 {
        if i != 13 {
            // let mut found = false;
            for color in colors.iter().rev() {
                if colors::compare_colors(&scheme[i], color) <= threshold {
                    base16[i] = *color;
                    // found = true;
                    break;
                }
            }
        }
    }

    for color in &base16 {
        println!("Color rgb({}, {}, {})", color.0, color.1, color.2);
    }
}
