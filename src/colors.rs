use std::cmp::max;

fn rgb_to_hsl(rgb: &(u8, u8, u8)) -> (f64, f64, f64) {
    let r: f64 = rgb.0 as f64 / 255.0;
    let g: f64 = rgb.1 as f64 / 255.0;
    let b: f64 = rgb.2 as f64 / 255.0;
    let cmax: f64 = r.max(g.max(b));
    let cmin: f64 = r.min(g.min(b));
    let delta = cmax - cmin;

    let l = (cmax + cmin) / 2.0;
    let mut h = 0.0;
    let mut s = 0.0;
    if delta != 0.0 {
        s = if l < 0.5 {
            delta / (cmax + cmin)
        } else {
            delta / (2.0 - cmax - cmin)
        };

        let del_r = (((cmax - r) / 6.0) + (delta / 2.0)) / delta;
        let del_g = (((cmax - g) / 6.0) + (delta / 2.0)) / delta;
        let del_b = (((cmax - b) / 6.0) + (delta / 2.0)) / delta;

        h = if r == cmax {
            del_b - del_g
        } else if g == cmax {
            (1.0 / 3.0) + del_r - del_b
        } else {
            (2.0 / 3.0) + del_g - del_r
        };

        if h < 0.0 {
            h += 1.0
        }
        if h > 1.0 {
            h -= 1.0
        }
    }
    (h, s, l)
}

fn hue_to_rgb(v1: f64, v2: f64, v3: f64) -> f64 {
    let mut vh = v3;
    if vh < 0.0 {
        vh += 1.0
    }
    if vh > 1.0 {
        vh -= 1.0
    }
    if (6.0 * vh) < 1.0 {
        return v1 + ((v2 - v1) * 6.0 * vh);
    }
    if (2.0 * vh) < 1.0 {
        return v2;
    }
    if (3.0 * vh) < 2.0 {
        return v1 + ((v2 - v1) * ((2.0 / 3.0) - vh) * 6.0);
    }
    v1
}

fn hsl_to_rgb(hsl: &(f64, f64, f64)) -> (u8, u8, u8) {
    let h = hsl.0;
    let s = hsl.1;
    let l = hsl.2;
    let mut r = (l * 255.0) as u8;
    let mut g = (l * 255.0) as u8;
    let mut b = (l * 255.0) as u8;

    if s != 0.0 {
        let var_2 = if l < 0.5 {
            l * (1.0 + s)
        } else {
            (l + s) - (s * l)
        };
        let var_1 = 2.0 * l - var_2;
        r = (255.0 * hue_to_rgb(var_1, var_2, h + (1.0 / 3.0))) as u8;
        g = (255.0 * hue_to_rgb(var_1, var_2, h)) as u8;
        b = (255.0 * hue_to_rgb(var_1, var_2, h - (1.0 / 3.0))) as u8;
    }
    (r, g, b)
}

// Compare max color distance against a threshold to remove greyscale colors
pub fn is_colorful(rgb: &(u8, u8, u8), threshold: u8) -> bool {
    let diff = (
        rgb.0.abs_diff(rgb.1),
        rgb.0.abs_diff(rgb.2),
        rgb.1.abs_diff(rgb.2),
    );
    let max_diff = max(diff.0, max(diff.1, diff.2));
    // adjust to 0-100 scale for simplicity in tweaking numbers
    let adjusted = (max_diff as f64 / 2.55) as u8;
    adjusted >= threshold
}

// Calculate a simplified color distance (https://www.compuphase.com/cmetric.htm)
pub fn compare_colors(rgb1: &(u8, u8, u8), rgb2: &(u8, u8, u8)) -> u16 {
    // println!(
    //     "Comparing: rgb({}, {}, {}) and rgb({}, {}, {})",
    //     rgb1.0, rgb1.1, rgb1.2, rgb2.0, rgb2.1, rgb2.2
    // );
    let rmean: f64 = (rgb1.0 as f64 + rgb2.0 as f64) / 2.0;
    let r: f64 = rgb1.0.abs_diff(rgb2.0).into();
    let g: f64 = rgb1.1.abs_diff(rgb2.1).into();
    let b: f64 = rgb1.2.abs_diff(rgb2.2).into();
    let pure_distance = f64::sqrt(
        ((2.0 + rmean / 256.0) * r * r) + (4.0 * g * g) + ((2.0 + (255.0 - rmean) / 256.0) * b * b),
    );
    // adjust to 0-100 scale for simplicity in tweaking numbers
    let adjusted: u16 = (pure_distance / 7.64) as u16;
    // println!("Distance: {}", adjusted);
    adjusted
}
