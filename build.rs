use std::env;
use std::fs::{self, File};
use std::hash::Hasher;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use const_gen::{CompileConst, const_declaration};
use xz2::read::XzEncoder;

fn main() {
    println!("cargo:rerun-if-changed=keyboard.toml");
    println!("cargo:rerun-if-changed=vial.json");

    generate_vial_config();
    install_memory_x();

    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rustc-link-arg=--nmagic");
    println!("cargo:rustc-link-arg=-Tlink.x");
    println!("cargo:rustc-link-arg=-Tdefmt.x");
}

fn generate_vial_config() {
    let out_file = Path::new(&env::var_os("OUT_DIR").unwrap()).join("config_generated.rs");
    let mut content = String::new();
    File::open("vial.json")
        .expect("vial.json is required for RMK Vial metadata")
        .read_to_string(&mut content)
        .expect("failed to read vial.json");

    let vial_json = json::stringify(json::parse(&content).expect("invalid vial.json"));
    let mut keyboard_def_compressed = Vec::new();
    XzEncoder::new(vial_json.as_bytes(), 6)
        .read_to_end(&mut keyboard_def_compressed)
        .expect("failed to compress vial.json");

    // Vial keyboard UID 16882930253541522617 (0xEA4C379DB209BCB9), encoded little-endian.
    let keyboard_id = vec![0xB9u8, 0xBC, 0x09, 0xB2, 0x9D, 0x37, 0x4C, 0xEA];
    let declarations = [
        const_declaration!(pub VIAL_KEYBOARD_DEF = keyboard_def_compressed),
        const_declaration!(pub VIAL_KEYBOARD_ID = keyboard_id),
    ]
    .map(|s| "#[allow(clippy::redundant_static_lifetimes)]\n".to_owned() + s.as_str())
    .join("\n");

    let keyboard_toml = fs::read("keyboard.toml").expect("keyboard.toml is required for RMK config");
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    hasher.write(&keyboard_toml);
    let keyboard_toml_fingerprint = hasher.finish();
    let declarations = format!(
        "{declarations}\n#[allow(dead_code)]\nconst _KEYBOARD_TOML_FINGERPRINT: u64 = {keyboard_toml_fingerprint};\n"
    );

    fs::write(out_file, declarations).expect("failed to write generated Vial config");
}

fn install_memory_x() {
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .expect("failed to create memory.x in OUT_DIR")
        .write_all(include_bytes!("memory.x"))
        .expect("failed to write memory.x in OUT_DIR");
    println!("cargo:rustc-link-search={}", out.display());
}
