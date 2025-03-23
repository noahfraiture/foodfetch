use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("meal_cache.json");
    fs::copy("src/data/meal_cache.json", &dest_path).unwrap();
    println!("cargo:rerun-if-changed=src/data/meal_cache.json");
} 