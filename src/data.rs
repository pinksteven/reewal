use std::collections::{BinaryHeap, HashMap};

use crate::color::{is_colorful, tweak_color};

use super::color;
use super::quantize::ColorCount;

pub fn get_accent(input: &mut BinaryHeap<ColorCount>, threshold: u8) -> (u8, u8, u8) {
    let mut colors = input.clone();

    colors.retain(|x| color::is_colorful(&x.rgb, threshold));
    let accent = colors.pop().unwrap().rgb;
    input.retain(|x| x.rgb.0 != accent.0 && x.rgb.1 != accent.1 && x.rgb.2 != accent.2);
    accent
}

pub fn map_colors(
    template_colors: &Vec<(u8, u8, u8)>,
    candidate_colors: BinaryHeap<ColorCount>,
    threshold: u16,
) -> HashMap<(u8, u8, u8), BinaryHeap<ColorCount>> {
    let mut color_map: HashMap<(u8, u8, u8), BinaryHeap<ColorCount>> = HashMap::new();

    for color in template_colors.iter().enumerate() {
        let mut heap: BinaryHeap<ColorCount> = BinaryHeap::new();

        for candidate in &candidate_colors {
            if color::compare_colors(color.1, &candidate.rgb, 1.0, 1.0, 1.0) <= threshold
                && (color.0 < 8 || color::is_colorful(&candidate.rgb, threshold as u8))
            {
                heap.push(candidate.clone());
            }
        }
        color_map.insert(*color.1, heap);
    }
    color_map
}

fn assign_grayscale_colors(
    color_map: &HashMap<(u8, u8, u8), BinaryHeap<ColorCount>>,
    template_colors: &Vec<(u8, u8, u8)>,
) -> Vec<Option<(u8, u8, u8)>> {
    let mut output: Vec<Option<(u8, u8, u8)>> = vec![None; 16];
    for i in 0..8 {
        let color = &template_colors[i];
        let mut best_color = None;
        let mut best_distance = u16::MAX;

        for candidate in color_map.get(color).unwrap().iter() {
            let candidate_distance = color::compare_colors(&candidate.rgb, color, 1.0, 1.0, 1.0);
            if candidate_distance < best_distance {
                best_distance = candidate_distance;
                best_color = Some(candidate.rgb);
            }
        }
        output[i] = best_color;
    }
    output
}

// Recursive deletion and replacement until all colors are distinct enough
fn check_and_replace(
    color_map: &mut HashMap<(u8, u8, u8), BinaryHeap<ColorCount>>,
    palette: &mut Vec<Option<(u8, u8, u8)>>,
    template_colors: &Vec<(u8, u8, u8)>,
    threshold: u16,
    index: usize,
) {
    if index == 13 {
        return;
    }
    if let Some(c1) = palette[index] {
        for i in 8..16 {
            if i != index {
                if let Some(c2) = palette[i] {
                    if color::compare_colors(&c1, &c2, 1.0, 1.0, 1.0) < threshold {
                        // Remove the color that is less similar to the template color
                        let c1_distance =
                            color::compare_colors(&c1, &template_colors[index], 1.0, 1.0, 1.0);
                        let c2_distance =
                            color::compare_colors(&c2, &template_colors[i], 1.0, 1.0, 1.0);
                        if c1_distance > c2_distance || i == 13 {
                            if let Some(next_candidate) =
                                color_map.get_mut(&template_colors[index]).unwrap().pop()
                            {
                                palette[index] = Some(next_candidate.rgb);
                                // Recheck against the entire palette
                                check_and_replace(
                                    color_map,
                                    palette,
                                    template_colors,
                                    threshold,
                                    index,
                                );
                            } else {
                                palette[index] = None; // mark as removed
                            }
                        } else if let Some(next_candidate) =
                            color_map.get_mut(&template_colors[i]).unwrap().pop()
                        {
                            palette[i] = Some(next_candidate.rgb);
                            // Recheck against the entire palette
                            check_and_replace(color_map, palette, template_colors, threshold, i);
                        } else {
                            // If no more candidate colors, leave it as is
                            palette[i] = None;
                        }
                    }
                }
            }
        }
    }
}

pub fn create_palette(
    color_map: &mut HashMap<(u8, u8, u8), BinaryHeap<ColorCount>>,
    template_colors: &Vec<(u8, u8, u8)>,
    accent_color: (u8, u8, u8),
    threshold: u16,
) -> Vec<Option<(u8, u8, u8)>> {
    let mut palette = assign_grayscale_colors(color_map, template_colors);
    palette[13] = Some(accent_color);

    // Assign most frequent candidate colors to each slot
    for i in 8..16 {
        if i != 13 {
            palette[i] = color_map
                .get_mut(&template_colors[i])
                .unwrap()
                .pop()
                .map(|x| x.rgb);
        }
    }

    // Remove colors that are too similar to each other
    for i in 8..16 {
        check_and_replace(color_map, &mut palette, template_colors, threshold, i);
    }

    palette
}
