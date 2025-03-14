use saphyr::Yaml;
use std::{
    fs::File,
    io::{Read, Write},
};

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
    println!("Opening scheme file: {}", path);
    let mut content = String::new();
    let mut file = File::open(path).expect("Could not open scheme file");
    file.read_to_string(&mut content)
        .expect("Could not read scheme file");
    println!("Opened");
    let docs = Yaml::load_from_str(&content).expect("Could not load scheme file as yaml");
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

pub fn export_scheme(path: &String, scheme: &Vec<Option<(u8, u8, u8)>>) {
    let mut file = File::create(path).expect("Could not save file");
    let mut output: String = r#"system: "base16"
name: "reewal-generated"
author: "reewal"
variant: "dark"
palette:
"#
    .to_string();
    for color in scheme.iter().flatten().enumerate() {
        output += format!(
            "  base{:02X}: \"#{:02X}{:02X}{:02X}\"\n",
            color.0, color.1 .0, color.1 .1, color.1 .2
        )
        .as_str();
    }
    let _ = file.write(output.as_bytes());
}
