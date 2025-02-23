use saphyr::Yaml;
use std::{fs::File, io::Read};

fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
    let offset = match hex.len() {
        6 => 0,
        7 => 1,
        _ => panic!("Not expected hex length {}, expected 6 or 7", hex.len()),
    };
    let r = u8::from_str_radix(&hex[offset..2 + offset], 16).unwrap();
    let g = u8::from_str_radix(&hex[2 + offset..4 + offset], 16).unwrap();
    let b = u8::from_str_radix(&hex[4 + offset..], 16).unwrap();
    (r, g, b)
}

pub fn get_scheme(path: &String) -> Vec<(u8, u8, u8)> {
    let mut content = String::new();
    let mut file = File::open(path).unwrap();
    file.read_to_string(&mut content).unwrap();
    let docs = Yaml::load_from_str(&content).unwrap();
    let out: Vec<(u8, u8, u8)> = docs[0]["palette"]
        .as_hash()
        .unwrap()
        .iter()
        .map(|(_k, v)| hex_to_rgb(v.as_str().unwrap()))
        .collect();

    if out.len() != 16 {
        panic!("Not a base16 paltte, expected 16 colors, got {}", out.len());
    }
    out
}
