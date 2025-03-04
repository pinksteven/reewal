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
    let mut grays = colors.clone();
    grays.retain(|color| !colors::is_colorful(color, 15));
    colors.retain(|color| colors::is_colorful(color, 15));
    // remove too similar colors
    // bc they can't be that close to each other if it's gonna work
    let mut i = 0;
    let mut len = colors.len();
    while i < len {
        let mut j = i + 1;
        while j < len {
            if colors::compare_colors(&colors[i], &colors[j], 1.0, 1.0, 1.0) < threshold {
                colors.remove(j);
                j -= 1;
                len -= 1;
            }
            j += 1;
        }
        i += 1;
    }
    colors.reverse();

    let mut base16: [Option<(u8, u8, u8)>; 16] = [None; 16];
    let mut accent = scheme[13];
    if !colors.is_empty() {
        accent = colors.pop().unwrap();
        base16[13] = Some(accent);
    } else {
        base16[13] = Some(accent);
    };

    // Get greyscale first 8 colors
    for i in 0..8 {
        let mut min: u16 = threshold + 1;
        let mut candidate: Option<(u8, u8, u8)> = None;
        for color in grays.iter() {
            let diff = colors::compare_colors(&scheme[i], color, 1.0, 1.0, 1.0);
            if diff < min {
                min = diff;
                candidate = Some(*color);
            }
        }
        if candidate.is_none() && accent != scheme[13] {
            candidate = Some(colors::mix_colors(&scheme[i], &accent, 100, 0, 0));
        } else if candidate.is_none() {
            candidate = Some(scheme[i]);
        };
        base16[i] = candidate;
    }

    // Get the scheme colors, aka the second half of it
    let mut candidates: Vec<Vec<(u8, u8, u8, u16)>> = vec![Vec::new(); 8];
    for i in 8..16 {
        //13 is accent color which we already have
        if i != 13 {
            for color in colors.iter().rev() {
                let distance = colors::compare_colors(&scheme[i], color, 1.0, 1.0, 1.0);
                if distance <= threshold {
                    candidates[i - 8].push((color.0, color.1, color.2, distance));
                }
            }
            if candidates[i - 8].is_empty() && accent != scheme[13] {
                let temp = colors::mix_colors(&scheme[i], &accent, 10, 100, 100);
                candidates[i - 8].push((
                    temp.0,
                    temp.1,
                    temp.2,
                    colors::compare_colors(&scheme[i], &temp, 1.0, 1.0, 1.0),
                ));
            } else if candidates[i - 8].is_empty() {
                candidates[i - 8].push((scheme[i].0, scheme[i].1, scheme[i].2, 0));
            };
            base16[i] = Some((
                candidates[i - 8][0].0,
                candidates[i - 8][0].1,
                candidates[i - 8][0].2,
            ));
            candidates[i - 8].reverse();
        } else {
            candidates[i - 8].push((accent.0, accent.1, accent.2, 0));
        }
    }

    // Remove to close colors for the theme to be viable
    let mut i = 8;
    while i < 16 {
        for j in 8..16 {
            if i != j
                && colors::compare_colors(&base16[i].unwrap(), &base16[j].unwrap(), 1.0, 1.0, 1.0)
                    < threshold / 2
                && base16[i].unwrap() != scheme[i]
                && base16[j].unwrap() != scheme[j]
                && !candidates[i - 8].is_empty()
                && !candidates[j - 8].is_empty()
            {
                println!(
                    "i {} len {}, j {} len {}",
                    i,
                    candidates[i - 8].len(),
                    j,
                    candidates[j - 8].len()
                );
                let change =
                    if candidates[i - 8].last().unwrap().3 >= candidates[j - 8].last().unwrap().3 {
                        i
                    } else {
                        j
                    };
                candidates[change - 8].pop();
                base16[change] = if candidates[change - 8].is_empty() && accent != scheme[13] {
                    let mut temp = colors::mix_colors(&scheme[change], &accent, 10, 100, 100);
                    let mut distance = colors::compare_colors(
                        &base16[i + j - change].unwrap(),
                        &temp,
                        1.0,
                        1.0,
                        1.0,
                    );
                    while distance <= threshold {
                        temp = colors::tweak_color(&temp, 0, 1, -1);
                        distance = colors::compare_colors(
                            &base16[i + j - change].unwrap(),
                            &temp,
                            1.0,
                            1.0,
                            1.0,
                        )
                    }
                    // candidates[change - 8].push((temp.0, temp.1, temp.2, 0));
                    Some(temp)
                } else if candidates[change - 8].is_empty() {
                    Some(scheme[change])
                } else {
                    let temp = candidates[change - 8].last().unwrap();
                    Some((temp.0, temp.1, temp.2))
                };
                if j <= i && change == j {
                    i = j - 1;
                    break;
                } else if change == i {
                    i -= 1;
                    break;
                }
            }
        }
        i += 1;
    }

    println!("Generated scheme: ");
    for color in base16.iter().flatten() {
        println!("Color rgb #{:02X}{:02X}{:02X}", color.0, color.1, color.2);
    }
    let _test = colors::compare_colors(&(0, 0, 0), &(255, 255, 255), 1.0, 1.0, 1.0);
}
