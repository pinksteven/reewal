use image::{GenericImageView, Pixel};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(PartialEq, Eq, Clone)]
pub struct ColorCount {
    pub rgb: (u8, u8, u8),
    pub count: usize,
}

impl Ord for ColorCount {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.count.cmp(&other.count) {
            Ordering::Equal => self.rgb.cmp(&other.rgb),
            ordering => ordering,
        }
    }
}

impl PartialOrd for ColorCount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Get a unique identifier for a color at a given depth
fn get_pixel_hash(rgb: &[u8], depth: u8) -> String {
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

pub fn quantize(img: &image::DynamicImage, depth: u8) -> BinaryHeap<ColorCount> {
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

    let mut output: BinaryHeap<ColorCount> = BinaryHeap::new();
    for leaf in tree.into_values() {
        let color = (
            (leaf.1 / leaf.0) as u8,
            (leaf.2 / leaf.0) as u8,
            (leaf.3 / leaf.0) as u8,
        );
        output.push(ColorCount {
            rgb: color,
            count: leaf.0 as usize,
        });
    }
    output
}
