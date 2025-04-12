use std::fs;
use std::path::Path;
use toml_edit::{Document, value};

pub fn add_dependency(package: &str) {
    let config_path = Path::new("cppgo.toml");
    if !config_path.exists() {
        eprintln!("Error: cppgo.toml not found. Are you in a cppgo project directory?");
        return;
    }

    let content = fs::read_to_string(config_path).expect("Failed to read cppgo.toml");
    let mut doc = content.parse::<Document>().expect("Invalid TOML format");

    // 插入依赖
    doc["dependencies"][package] = value("*");

    fs::write(config_path, doc.to_string()).expect("Failed to write cppgo.toml");

    println!("✅ Added dependency '{}'", package);
}
