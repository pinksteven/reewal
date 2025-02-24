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
            if colors::compare_colors(&colors[i], &colors[j]) < threshold {
                colors.remove(j);
                j -= 1;
                len -= 1;
            }
            j += 1;
        }
        i += 1;
    }
    colors.reverse();

    let mut base16: Vec<String> = vec![String::new(); 16];
    let accent = colors.pop().unwrap();
    base16[13] = format!("#{:X}{:X}{:X}", accent.0, accent.1, accent.2);
    for i in 0..8 {
        let mut found = false;
        let mut min: u16 = threshold + 1;
        let mut candidate: (u8, u8, u8) = (255, 0, 0);
        for color in grays.iter() {
            let diff = colors::compare_colors(&scheme[i], color);
            if diff < min {
                min = diff;
                candidate = *color;
                found = true;
            }
        }
        if !found {
            candidate = colors::mix_colors(&scheme[i], &accent, 100, 0, 0);
        };
        base16[i] = format!("#{:02X}{:02X}{:02X}", candidate.0, candidate.1, candidate.2);
    }
    let mut candidates: Vec<Vec<(String, u16)>> = vec![Vec::new(); 8];
    for i in 8..16 {
        //13 is accent color which we already have
        if i != 13 {
            for color in colors.iter().rev() {
                let distance = colors::compare_colors(&scheme[i], color);
                if distance <= threshold {
                    candidates[i - 8].push((
                        format!("#{:02X}{:02X}{:02X}", color.0, color.1, color.2),
                        distance,
                    ));
                }
            }
            base16[i] = if candidates[i - 8].is_empty() {
                let temp = colors::mix_colors(&scheme[i], &accent, 10, 100, 100);
                format!("#{:02X}{:02X}{:02X}", temp.0, temp.1, temp.2)
            } else {
                candidates[i - 8][0].0.clone()
            };
            candidates[i - 8].reverse();
        }
    }

    let mut i = 8;
    while i < 16 {
        if !base16[i].is_empty() {
            for j in 8..16 {
                if i != j && base16[i] == base16[j] {
                    let change = if candidates[i - 8].last().unwrap().1
                        >= candidates[j - 8].last().unwrap().1
                    {
                        i
                    } else {
                        j
                    };
                    candidates[change - 8].pop();
                    base16[change] = if candidates[change - 8].is_empty() {
                        let temp = colors::mix_colors(&scheme[change], &accent, 0, 100, 100);
                        format!("#{:02X}{:02X}{:02X}", temp.0, temp.1, temp.2)
                    } else {
                        candidates[change - 8].last().unwrap().0.clone()
                    };
                    if j < i && change == j {
                        i = j;
                        break;
                    } else if change == i {
                        i -= 1;
                        break;
                    }
                }
            }
        }
        i += 1;
    }

    println!("Generated scheme: ");
    for color in &base16 {
        println!("Color rgb {}", color);
    }
}
