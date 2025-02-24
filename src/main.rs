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
    let threshold = 25;

    let current_dir = env::current_dir().unwrap().display().to_string();
    let path = format!("{}/{}", current_dir, image);
    println!("Reading image: {}", path);
    let img = image::open(path).unwrap();
    println!("Opened");

    let scheme = yaml::get_scheme(&format!("{}/{}", current_dir, yaml));
    let mut colors = quantize::quantize(&img, depth);
    // remove too similar colors
    // bc they can't be that close to each other if it's gonna work
    let mut i = 0;
    let mut len = colors.len();
    while i < len {
        let mut j = i + 1;
        while j < len {
            if colors::compare_colors(&colors[i], &colors[j]) < (threshold / 2) {
                colors.remove(j);
                j -= 1;
                len -= 1;
            }
            j += 1;
        }
        i += 1;
    }
    println!("Got {} colors", colors.len());
    for color in &colors {
        println!("Color rgb({}, {}, {})", color.0, color.1, color.2);
    }
    colors.reverse();
    let mut grays = colors.clone();
    grays.retain(|color| !colors::is_colorful(color, 15));
    colors.retain(|color| colors::is_colorful(color, 15));

    let mut base16: [(u8, u8, u8); 16] = [(255, 0, 0); 16];
    base16[13] = colors.pop().unwrap();
    for i in 0..8 {
        let mut found = false;
        let mut min: u16 = threshold + 1;
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
        } else {
            base16[i] = colors::mix_colors(&scheme[i], &base16[13], 100, 0, 0);
        }
    }
    let mut candidates: Vec<Vec<(String, u16)>> = Vec::new();
    for i in 8..16 {
        if i != 13 {
            for color in colors.iter().rev() {
                let distance = colors::compare_colors(&scheme[i], color);
                if distance <= threshold {
                    candidates[i - 8]
                        .push((format!("{:X}{:X}{:X}", color.0, color.1, color.2), distance));
                }
            }
        }
    }
    println!("Generated scheme: ");
    for color in &base16 {
        println!("Color rgb({}, {}, {})", color.0, color.1, color.2);
    }
}
