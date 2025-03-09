use std::f64::consts::PI;

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
    //threshold is a distance from 0 saturation at 0.5 lightness and 0.8*threshold are
    //the top/bottom values of lighness at max saturation, treating it as a quadratic curve,
    //gives this function given the threshold and lightness this return the minimum value for saturation
    let color_point = ((1.0 - 0.8 * t) / (0.8 * t - 0.5).powi(2)) * (hsl.2 - 0.5).powi(2) + t;
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

// Calculate the delta E of colors using CIE2000
// with ability to fine tune weight factors
pub fn compare_colors(
    rgb1: &(u8, u8, u8),
    rgb2: &(u8, u8, u8),
    hue_weight: f64,
    chroma_weight: f64,
    light_weight: f64,
) -> u16 {
    let lab1 = rgb_to_lab(rgb1);
    let lab2 = rgb_to_lab(rgb2);

    let delta_l = lab2.0 - lab1.0;
    let xl = (lab1.0 + lab2.0) / 2.0;
    let mut c1 = (lab1.1.powi(2) + lab1.2.powi(2)).sqrt();
    let mut c2 = (lab2.1.powi(2) + lab2.2.powi(2)).sqrt();
    let mut xc = (c1 + c2) / 2.0;
    let a1 = lab1.1 + (lab1.1 / 2.0) * (1.0 - (xc.powi(7) / (xc.powi(7) + 6103515625.0)).sqrt());
    let a2 = lab2.1 + (lab2.1 / 2.0) * (1.0 - (xc.powi(7) / (xc.powi(7) + 6103515625.0)).sqrt());

    c1 = (a1.powi(2) + lab1.2.powi(2)).sqrt();
    c2 = (a2.powi(2) + lab2.2.powi(2)).sqrt();
    xc = (c1 + c2) / 2.0;
    let delta_c = c2 - c1;

    let h1 = (lab1.2.to_radians().atan2(a1.to_radians()) + PI) % (2.0 * PI);
    let h2 = (lab2.2.to_radians().atan2(a2.to_radians()) + PI) % (2.0 * PI);
    let delta_h = if c1 * c2 == 0.0 {
        0.0
    } else if (h1 - h2).abs() <= PI {
        2.0 * (c1 * c2).sqrt() * ((h2 - h1) / 2.0).sin()
    } else if h2 <= h1 {
        2.0 * (c1 * c2).sqrt() * ((h2 - h1 + 2.0 * PI) / 2.0).sin()
    } else {
        2.0 * (c1 * c2).sqrt() * ((h2 - h1 - 2.0 * PI) / 2.0).sin()
    };
    let xh = if (h1 - h2).abs() <= PI {
        (h1 + h2) / 2.0
    } else if h1 + h2 < 2.0 * PI {
        (h1 + h2 + 2.0 * PI) / 2.0
    } else {
        (h1 + h2 - 2.0 * PI) / 2.0
    };
    let xt = 1.0 - 0.17 * (xh - PI / 6.0).cos()
        + 0.24 * (2.0 * xh).cos()
        + 0.32 * (3.0 * xh + PI / 30.0).cos()
        - 0.2 * (4.0 * xh - 0.35 * PI).cos();
    let sl = 1.0 + 0.015 * (xl - 50.0).powi(2) / (20.0 + (xl - 50.0).powi(2)).sqrt();
    let sc = 1.0 + 0.045 * xc;
    let sh = 1.0 + 0.015 * xc * xt;
    let rt = -2.0
        * (xc.powi(7) / (xc.powi(7) + 6103515625.0)).sqrt()
        * (60.0 * (-((xh.to_degrees() - 275.0) / 25.0).powi(2)).exp())
            .to_radians()
            .sin();
    let l_part = delta_l / (sl * light_weight);
    let c_part = delta_c / (sc * chroma_weight);
    let h_part = delta_h / (sh * hue_weight);
    let rt_part = rt * c_part * h_part;
    let result = (l_part.powi(2) + c_part.powi(2) + h_part.powi(2) + rt_part).sqrt();
    result.round() as u16
}

pub fn mix_colors(
    main: &(u8, u8, u8),
    accent: &(u8, u8, u8),
    hue_factor: i8,
    sat_factor: i8,
    light_factor: i8,
) -> (u8, u8, u8) {
    let main_hsl = rgb_to_hsl(main);
    let accent_hsl = rgb_to_hsl(accent);
    //multiply diff by scaled factors
    let hue_change = (main_hsl.0 - accent_hsl.0) * (hue_factor as f64 / 100.0);
    let sat_change = (main_hsl.1 - accent_hsl.1) * (sat_factor as f64 / 100.0);
    let light_change = (main_hsl.2 - accent_hsl.2) * (light_factor as f64 / 100.0);
    //move the values
    let result: (f64, f64, f64) = (
        (main_hsl.0 - hue_change).clamp(0.0, 1.0),
        (main_hsl.1 - sat_change).clamp(0.0, 1.0),
        (main_hsl.2 - light_change).clamp(0.0, 1.0),
    );
    hsl_to_rgb(&result)
}

pub fn tweak_color(rgb: &mut (u8, u8, u8), hue_factor: i8, sat_factor: i8, light_factor: i8) {
    let hsl = rgb_to_hsl(rgb);
    let h_change = hsl.0 * (hue_factor as f64 / 100.0);
    let s_change = hsl.1 * (sat_factor as f64 / 100.0);
    let l_change = hsl.2 * (light_factor as f64 / 100.0);

    let result: (f64, f64, f64) = (
        (hsl.0 + h_change).clamp(0.0, 1.0),
        (hsl.1 + s_change).clamp(0.0, 1.0),
        (hsl.2 + l_change).clamp(0.0, 1.0),
    );
    *rgb = hsl_to_rgb(&result);
}
