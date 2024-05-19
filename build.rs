use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = PathBuf::from(out_dir).parent().unwrap().parent().unwrap().to_path_buf();
    let script_path = "scripts/clean.js";

    fs::copy(script_path, target_dir.join("clean.js")).expect("Failed to copy clean.js");
}
