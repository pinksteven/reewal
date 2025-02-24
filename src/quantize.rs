use image::{GenericImageView, Pixel};
use std::collections::HashMap;

// Get a unique identifier for a color at a given depth
fn get_pixel_hash(rgb: &[u8], depth: i8) -> String {
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

pub fn quantize(img: &image::DynamicImage, depth: i8) -> Vec<(u8, u8, u8)> {
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
    let mut occurrences: Vec<(u64, u8, u8, u8)> = tree
        .into_values()
        .map(|(c, r, g, b)| {
            (
                c,
                r.div_euclid(c).try_into().unwrap(),
                g.div_euclid(c).try_into().unwrap(),
                b.div_euclid(c).try_into().unwrap(),
            )
        })
        .collect();
    occurrences.sort_by(|a, b| b.0.cmp(&a.0));
    let colors: Vec<(u8, u8, u8)> = occurrences
        .iter()
        .map(|(_c, r, g, b)| (*r, *g, *b))
        .collect();
    colors
}
