use super::yaml::get_scheme;

pub struct Config {
    pub template_colors: Vec<(u8, u8, u8)>,
    pub depth: u8,
    pub variance: u16,
    pub colorful_threshold: u8,
    pub similarity_threshold: u16,

    pub hue_compare: f64,
    pub chroma_compare: f64,
    pub light_compare: f64,

    pub hue_mix: i8,
    pub saturation_mix: i8,
    pub light_mix: i8,

    pub hue_tweak: i8,
    pub saturation_tweak: i8,
    pub light_tweak: i8,
}

pub fn parse_config(args: Vec<String>) -> Config {
    let mut config = Config {
        template_colors: vec![
            (34, 34, 34),
            (48, 48, 48),
            (85, 85, 85),
            (137, 137, 137),
            (192, 192, 192),
            (255, 255, 255),
            (255, 255, 255),
            (176, 176, 176),
            (225, 93, 103),
            (252, 128, 78),
            (242, 196, 43),
            (93, 177, 41),
            (33, 201, 146),
            (0, 163, 242),
            (180, 110, 224),
            (184, 125, 40),
        ],
        depth: 2,
        variance: 20,
        colorful_threshold: 15,
        similarity_threshold: 20,

        hue_compare: 0.75,
        chroma_compare: 1.0,
        light_compare: 1.0,

        hue_mix: 10,
        saturation_mix: 100,
        light_mix: 100,

        hue_tweak: 0,
        saturation_tweak: -1,
        light_tweak: 1,
    };

    // I feel like this is absolute shit, but it's gonna work like that
    for entry in args.iter().enumerate().skip(1).step_by(2) {
        if entry.1 == "-t" {
            let path = args[entry.0 + 1].clone();
            if path.starts_with("/") || path.starts_with("\\") {
                config.template_colors = get_scheme(&path)
            } else {
                let current_dir = std::env::current_dir().unwrap().display().to_string();
                config.template_colors = get_scheme(&format!("{}/{}", current_dir, path));
            }
        } else if entry.1 == "-d" {
            let depth: i64 = args[entry.0 + 1].parse().unwrap();
            if !(1..=8).contains(&depth) {
                panic!("-d: Incorrect depth value. Expected 1-8, got {}", depth);
            } else {
                config.depth = depth.try_into().expect("Unknown error when parsing depth");
            }
        } else if entry.1 == "-v" {
            config.variance = args[entry.0 + 1]
                .parse()
                .expect("-v: Incorrect variance value. Expected an unsigned 16bit integer");
        } else if entry.1 == "-c" {
            let input: i64 = args[entry.0 + 1].parse().unwrap();
            if !(1..=100).contains(&input) {
                panic!(
                    "-c: Incorrect color threshold value. Expected 1-100, got {}",
                    input
                );
            } else {
                config.colorful_threshold = input
                    .try_into()
                    .expect("Unknown error when parsing color threshold");
            }
        } else if entry.1 == "-s" {
            config.similarity_threshold = args[entry.0 + 1]
                .parse()
                .expect("-s: Expected an unsigned 16bit integer");
        } else if entry.1 == "--hue-compare" {
            let input: f64 = args[entry.0 + 1]
                .parse()
                .expect("--hue-compare: Expected a float");
            if input <= 0.0 {
                panic!(
                    "--hue-compare: Incorrect value. Expected a float > 0.0, got {}",
                    input
                );
            } else {
                config.hue_compare = input;
            }
        } else if entry.1 == "--chroma-compare" {
            let input: f64 = args[entry.0 + 1]
                .parse()
                .expect("--chroma-compare: Expected a float");
            if input <= 0.0 {
                panic!(
                    "--chroma-compare: Incorrect value. Expected a float > 0.0, got {}",
                    input
                );
            } else {
                config.chroma_compare = input;
            }
        } else if entry.1 == "--light-compare" {
            let input: f64 = args[entry.0 + 1]
                .parse()
                .expect("--light-compare: Expected a float");
            if input <= 0.0 {
                panic!(
                    "--light-compare: Incorrect value. Expected a float > 0.0, got {}",
                    input
                );
            } else {
                config.light_compare = input;
            }
        } else if entry.1 == "--hue-mix" {
            config.hue_mix = args[entry.0 + 1]
                .parse()
                .expect("--hue-mix: Expected an 8bit integer");
        } else if entry.1 == "--saturation-mix" {
            config.saturation_mix = args[entry.0 + 1]
                .parse()
                .expect("--saturation-mix: Expected an 8bit integer");
        } else if entry.1 == "--light-mix" {
            config.light_mix = args[entry.0 + 1]
                .parse()
                .expect("--light-mix: Expected an 8bit integer");
        } else if entry.1 == "--hue-tweak" {
            config.hue_tweak = args[entry.0 + 1]
                .parse()
                .expect("--hue-tweak: Expected an 8bit integer");
        } else if entry.1 == "--saturation-tweak" {
            config.saturation_tweak = args[entry.0 + 1]
                .parse()
                .expect("--saturation-tweak: Expected an 8bit integer");
        } else if entry.1 == "--light-tweak" {
            config.light_tweak = args[entry.0 + 1]
                .parse()
                .expect("--light-tweak: Expected an 8bit integer");
        } else {
            eprintln!("Unknown argument: {}", entry.1);
        }
    }

    config
}
