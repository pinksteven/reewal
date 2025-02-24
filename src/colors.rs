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
    let hsl = rgb_to_hsl(rgb);
    // scale threshold from 0-100 to 0-1
    let t: f64 = threshold as f64 / 100.0;

    //a bunch of funky math on hsl,
    //threshold is a distance from 0 saturation at 0.5 lightness and the top/bottom values of
    //lighness at max saturation, treating it as a quadratic curve, gives this function
    //given the threshold and lightness this return the minimum value for saturation
    let color_point = ((1.0 - t) / (t - 0.5).powi(2)) * (hsl.2 - 0.5).powi(2) + t;
    hsl.1 >= color_point
}

// Needed for color comparison
fn rgb_to_lab(rgb: &(u8, u8, u8)) -> (f64, f64, f64) {
    // Convert to XYZ, as rgb can't really be converted straight to lab
    let mut r: f64 = rgb.0 as f64 / 255.0;
    let mut g: f64 = rgb.1 as f64 / 255.0;
    let mut b: f64 = rgb.2 as f64 / 255.0;

    r = if r > 0.04045 {
        ((r + 0.055) / 1.055).powf(2.4) * 100.0
    } else {
        (r / 12.92) * 100.0
    };
    g = if g > 0.04045 {
        ((g + 0.055) / 1.055).powf(2.4) * 100.0
    } else {
        (g / 12.92) * 100.0
    };
    b = if b > 0.04045 {
        ((b + 0.055) / 1.055).powf(2.4) * 100.0
    } else {
        (b / 12.92) * 100.0
    };
    // Dividing but D65/2 reference in this stepb
    let mut x: f64 = (r * 0.4124 + g * 0.3576 + b * 0.1805) / 95.047;
    let mut y: f64 = (r * 0.2126 + g * 0.7152 + b * 0.0722) / 100.0;
    let mut z: f64 = (r * 0.0193 + g * 0.1192 + b * 0.9505) / 108.883;

    // Conversion to L*ab
    x = if x > 0.008856 {
        x.powf(1.0 / 3.0)
    } else {
        (7.787 * x) + (16.0 / 116.0)
    };
    y = if y > 0.008856 {
        y.powf(1.0 / 3.0)
    } else {
        (7.787 * y) + (16.0 / 116.0)
    };
    z = if z > 0.008856 {
        z.powf(1.0 / 3.0)
    } else {
        (7.787 * z) + (16.0 / 116.0)
    };
    let l: f64 = (116.0 * y) - 16.0;
    let a: f64 = 500.0 * (x - y);
    let b: f64 = 200.0 * (y - z);

    (l, a, b)
}

// Calculate the delta E of colors
// using the very outdated version due to simplicity in calculation
// and lack of need for that high levels of precision
pub fn compare_colors(rgb1: &(u8, u8, u8), rgb2: &(u8, u8, u8)) -> u16 {
    let lab1 = rgb_to_lab(rgb1);
    let lab2 = rgb_to_lab(rgb2);
    let delta =
        ((lab1.0 - lab2.0).powi(2) + (lab1.1 - lab2.1).powi(2) + (lab1.2 - lab2.2).powi(2)).sqrt();
    delta.round() as u16
}

pub fn mix_colors(
    main: &(u8, u8, u8),
    accent: &(u8, u8, u8),
    hue_factor: u8,
    sat_factor: u8,
    light_factor: u8,
) -> (u8, u8, u8) {
    let main_hsl = rgb_to_hsl(main);
    let accent_hsl = rgb_to_hsl(accent);
    //multiply diff by scaled factors
    let hue_change = (main_hsl.0 - accent_hsl.0) * (hue_factor as f64 / 100.0);
    let sat_change = (main_hsl.1 - accent_hsl.1) * (sat_factor as f64 / 100.0);
    let light_change = (main_hsl.2 - accent_hsl.2) * (light_factor as f64 / 100.0);
    //move the values
    let result: (f64, f64, f64) = (
        (main_hsl.0 - hue_change),
        (main_hsl.1 - sat_change),
        (main_hsl.2 - light_change),
    );
    hsl_to_rgb(&result)
}
