use image::{GenericImageView, Pixel};
use std::{collections::HashMap, env};

// Get a unique identifier for a color at a given depth
pub fn get_pixel_hash(rgb: &[u8], depth: i8) -> String {
    let mut pixel_hash = String::new();
    for i in 0..depth {
        let r_bit = (rgb[0] >> (7 - i)) & 1;
        let g_bit = (rgb[1] >> (7 - i)) & 1;
        let b_bit = (rgb[2] >> (7 - i)) & 1;

        // Combine these bits into a single value
        let combined_bits = (r_bit << 2) | (g_bit << 1) | b_bit;

        pixel_hash.push_str(&combined_bits.to_string());
    }

    pixel_hash
}

pub fn quantize(img: &image::DynamicImage, depth: i8) -> Vec<(u64, u64, u64, u64)> {
    let mut tree: HashMap<String, (u64, u64, u64, u64)> = HashMap::new();

    for (_x, _y, pixel) in img.pixels() {
        let pixel_rgb = pixel.to_rgb();
        let rgb = pixel_rgb.channels();
        let pixel_hash = get_pixel_hash(rgb, depth);

        if tree.contains_key(&pixel_hash) {
            let v = tree.get(&pixel_hash).unwrap();
            tree.insert(
                pixel_hash,
                (
                    v.0 + 1,
                    v.1 + u64::from(pixel_rgb[0]),
                    v.2 + u64::from(pixel_rgb[1]),
                    v.3 + u64::from(pixel_rgb[2]),
                ),
            );
        } else {
            tree.insert(
                pixel_hash,
                (
                    1,
                    pixel_rgb[0].into(),
                    pixel_rgb[1].into(),
                    pixel_rgb[2].into(),
                ),
            );
        }
    }
    let mut colors: Vec<(u64, u64, u64, u64)> = tree
        .into_values()
        .map(|(c, r, g, b)| (c, r / c, g / c, b / c))
        .collect();
    colors.sort_by(|a, b| b.0.cmp(&a.0));
    colors
}

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

    let colors = quantize(&img, depth);

    println!("Palette colors: {}", colors.len());
    for color in colors {
        println!(
            "rgb({}, {}, {}) appears {} times",
            color.1, color.2, color.3, color.0
        );
    }
}
