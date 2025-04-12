use std::fs;
use std::path::Path;

pub fn clean_project() {
    // 清理依赖
    let deps_dir = Path::new(".cppgo/deps");
    if deps_dir.exists() {
        fs::remove_dir_all(deps_dir).expect("Failed to remove dependencies");
        println!("✅ Cleaned dependencies.");
    } else {
        println!("No dependencies to clean.");
    }

    // 清理编译产物
    let target_dir = Path::new("target/debug");
    if target_dir.exists() {
        fs::remove_dir_all(target_dir).expect("Failed to remove target/debug");
        println!("✅ Cleaned build artifacts.");
    } else {
        println!("No build artifacts to clean.");
    }
}
