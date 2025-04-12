use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::Command;

use serde::Deserialize;

// 结构体：映射 cppgo.toml 配置
#[derive(Debug, Deserialize)]
struct TomlConfig {
    dependencies: HashMap<String, String>,
}

// 安装依赖函数
pub fn install_dependencies() {
    // 加载 cppgo.toml
    let config_path = Path::new("cppgo.toml");
    if !config_path.exists() {
        eprintln!("Error: cppgo.toml not found.");
        return;
    }

    let config_content = fs::read_to_string(config_path).expect("Failed to read cppgo.toml");
    let config: TomlConfig = toml::from_str(&config_content).expect("Failed to parse cppgo.toml");

    // 创建 .cppgo/deps 目录
    let deps_dir = Path::new(".cppgo/deps");
    if !deps_dir.exists() {
        fs::create_dir_all(deps_dir).expect("Failed to create .cppgo/deps directory");
    }

    // 遍历 dependencies 安装
    for (package, version) in config.dependencies.iter() {
        let package_dir = deps_dir.join(package);

        // 检查依赖是否已存在
        if package_dir.exists() {
            let version_file = package_dir.join("VERSION");
            let current_version = if version_file.exists() {
                fs::read_to_string(&version_file).unwrap_or_else(|_| "0.0.0".to_string())
            } else {
                "0.0.0".to_string()
            };

            // 版本不一致时，删除旧依赖
            if current_version != *version {
                println!(
                    "🔄 Updating '{}' from version '{}' to '{}'",
                    package, current_version, version
                );
                fs::remove_dir_all(&package_dir).expect("Failed to remove old dependency");
                install_package(package, version, &package_dir);
            } else {
                println!("✅ '{}' version '{}' already installed.", package, version);
            }
        } else {
            println!("📦 Installing '{}' version '{}'...", package, version);
            install_package(package, version, &package_dir);
        }
    }
}

// 安装单个依赖（简单模拟）
fn install_package(package: &str, version: &str, package_dir: &Path) {
    // 模拟克隆或下载依赖包
    fs::create_dir_all(package_dir).expect("Failed to create package directory");

    // 写入 VERSION 文件记录当前版本
    let version_file_path = package_dir.join("VERSION");
    let mut version_file = File::create(&version_file_path).expect("Failed to create VERSION file");
    version_file
        .write_all(version.as_bytes())
        .expect("Failed to write version info");

    // 模拟构建步骤
    println!("🔧 Building '{}'", package);
    let build_result = Command::new("echo")
        .arg(format!("Building package '{}'", package))
        .current_dir(&package_dir)
        .output();

    match build_result {
        Ok(output) => {
            println!("{}", String::from_utf8_lossy(&output.stdout));
            println!("✅ Installed '{}'", package);
        }
        Err(e) => {
            eprintln!("❌ Build failed for '{}': {}", package, e);
        }
    }
}
