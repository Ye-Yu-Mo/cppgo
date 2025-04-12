use std::fs;
use std::path::Path;
use toml_edit::Document;

pub fn remove_dependency(package: &str) {
    let config_path = Path::new("cppgo.toml");
    if !config_path.exists() {
        eprintln!("Error: cppgo.toml not found. Are you in a cppgo project directory?");
        return;
    }

    let content = fs::read_to_string(config_path).expect("Failed to read cppgo.toml");
    let mut doc = content.parse::<Document>().expect("Invalid TOML format");

    // 移除依赖
    if doc["dependencies"].as_table_mut().unwrap().remove(package).is_none() {
        eprintln!("Warning: dependency '{}' not found.", package);
        return;
    }

    fs::write(config_path, doc.to_string()).expect("Failed to write cppgo.toml");

    println!("✅ Removed dependency '{}'", package);
}
