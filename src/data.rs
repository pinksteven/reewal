use std::collections::{BinaryHeap, HashMap};

use super::color;
use super::config::Config;
use super::quantize::ColorCount;

pub fn get_accent(input: &mut BinaryHeap<ColorCount>, threshold: u8) -> (u8, u8, u8) {
    let mut colors = input.clone();

    colors.retain(|x| color::is_colorful(&x.rgb, threshold));
    let accent = colors.pop().unwrap().rgb;
    input.retain(|x| x.rgb.0 != accent.0 && x.rgb.1 != accent.1 && x.rgb.2 != accent.2);
    accent
}

pub fn map_colors(
    candidate_colors: BinaryHeap<ColorCount>,
    config: &Config,
) -> HashMap<(u8, u8, u8), BinaryHeap<ColorCount>> {
    let mut color_map: HashMap<(u8, u8, u8), BinaryHeap<ColorCount>> = HashMap::new();
    let template_colors = &config.template_colors;

    for color in template_colors.iter().enumerate() {
        let mut heap: BinaryHeap<ColorCount> = BinaryHeap::new();

        for candidate in &candidate_colors {
            if color::compare_colors(
                color.1,
                &candidate.rgb,
                config.hue_compare,
                config.chroma_compare,
                config.light_compare,
            ) <= config.likeness
                && (color.0 < 8 || color::is_colorful(&candidate.rgb, config.vibrancy))
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
    config: &Config,
) -> Vec<Option<(u8, u8, u8)>> {
    let mut output: Vec<Option<(u8, u8, u8)>> = vec![None; 16];
    for i in 0..8 {
        let color = &template_colors[i];
        let mut best_color = None;
        let mut best_distance = u16::MAX;

        for candidate in color_map.get(color).unwrap().iter() {
            let candidate_distance = color::compare_colors(
                &candidate.rgb,
                color,
                2.0 * config.hue_compare, // We're comparing grayscale colors, hue shouldn't matter
                config.chroma_compare,
                0.9 * config.light_compare, // and this should matter more i think
            );
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
    config: &Config,
    index: usize,
) {
    if index == 13 {
        return;
    }
    if let Some(c1) = palette[index] {
        for i in 8..16 {
            if i != index {
                if let Some(c2) = palette[i] {
                    if color::compare_colors(&c1, &c2, 1.0, 1.0, 1.0) < config.similarity {
                        // Remove the color that is less similar to the template color
                        let c1_distance = color::compare_colors(
                            &c1,
                            &template_colors[index],
                            config.hue_compare,
                            config.chroma_compare,
                            config.light_compare,
                        );
                        let c2_distance = color::compare_colors(
                            &c2,
                            &template_colors[i],
                            config.hue_compare,
                            config.chroma_compare,
                            config.light_compare,
                        );
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
                                    config,
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
                            check_and_replace(color_map, palette, template_colors, config, i);
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

fn gen_color(
    palette: &Vec<Option<(u8, u8, u8)>>,
    template_colors: &Vec<(u8, u8, u8)>,
    accent: &(u8, u8, u8),
    config: &Config,
    index: usize,
) -> (u8, u8, u8) {
    let mut generated = color::mix_colors(
        &template_colors[index],
        accent,
        config.hue_mix,
        config.saturation_mix,
        config.light_mix,
    );
    let mut distance = color::compare_colors(
        &generated,
        &template_colors[index],
        config.hue_compare,
        config.chroma_compare,
        config.light_compare,
    );
    let mut palette_distance = u16::MAX;
    for i in 8..16 {
        if let Some(c) = palette[i] {
            let temp = color::compare_colors(
                &generated,
                &c,
                config.hue_compare,
                config.chroma_compare,
                config.light_compare,
            );
            if temp < palette_distance {
                palette_distance = temp;
            }
        }
    }
    let mut best_distance = palette_distance;
    let mut best_color = generated;
    let mut i = 0;
    while distance <= config.similarity && i < u16::MAX {
        generated = color::tweak_color(
            &generated,
            config.hue_tweak,
            config.saturation_tweak,
            config.light_tweak,
        );
        distance = color::compare_colors(
            &generated,
            &template_colors[index],
            config.hue_compare,
            config.chroma_compare,
            config.light_compare,
        );
        palette_distance = u16::MAX;
        for i in 8..16 {
            if let Some(c) = palette[i] {
                let temp = color::compare_colors(
                    &generated,
                    &c,
                    config.hue_compare,
                    config.chroma_compare,
                    config.light_compare,
                );
                if temp < palette_distance {
                    palette_distance = temp;
                }
            }
        }
        if palette_distance > best_distance {
            best_distance = palette_distance;
            best_color = generated;
        }
        i += 1;
    }
    best_color
}

pub fn create_palette(
    color_map: &mut HashMap<(u8, u8, u8), BinaryHeap<ColorCount>>,
    accent_color: (u8, u8, u8),
    config: &Config,
) -> Vec<Option<(u8, u8, u8)>> {
    let template_colors = &config.template_colors;
    let mut palette = assign_grayscale_colors(color_map, template_colors, config);
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
        check_and_replace(color_map, &mut palette, template_colors, config, i);
    }

    for i in 8..16 {
        if palette[i].is_none() {
            palette[i] = Some(gen_color(
                &palette,
                template_colors,
                &accent_color,
                config,
                i,
            ));
        }
    }

    palette
}
