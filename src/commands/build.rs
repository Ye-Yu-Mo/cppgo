use std::fs;
use std::path::Path;
use std::process::Command;

pub fn build_project() {
    // 获取项目根目录的 cppgo.toml 文件
    let config_path = Path::new("cppgo.toml");
    if !config_path.exists() {
        eprintln!("Error: cppgo.toml not found. Are you in a cppgo project directory?");
        return;
    }

    // 遍历依赖，假设每个依赖有一个简单的 CMakeLists.txt 或 Makefile
    let deps_dir = Path::new(".cppgo/deps");
    if deps_dir.exists() {
        for dep in fs::read_dir(deps_dir).expect("Failed to read deps directory") {
            let dep_path = dep.expect("Failed to read entry").path();
            if dep_path.is_dir() {
                println!("🔨 Building dependency from: {:?}", dep_path);
                let build_cmd = Command::new("cmake")
                    .arg(".")
                    .current_dir(&dep_path)  // 借用 dep_path
                    .spawn();

                match build_cmd {
                    Ok(mut child) => {
                        child.wait().expect("Failed to build dependency");
                        println!("✅ Built dependency: {:?}", dep_path);
                    }
                    Err(e) => {
                        eprintln!("Error building dependency {:?}: {}", dep_path, e);
                    }
                }
            }
        }
    }

    // 创建目标目录
    let target_dir = Path::new("target/debug");
    if !target_dir.exists() {
        fs::create_dir_all(target_dir).expect("Failed to create target/debug directory");
    }

    // 编译主项目 src/main.cpp
    let main_dir = Path::new("src/main.cpp");
    if main_dir.exists() {
        println!("🔨 Building main.cpp...");
        let build_cmd = Command::new("g++")
            .arg("src/main.cpp")
            .arg("-o")
            .arg("target/debug/main")
            .spawn();

        match build_cmd {
            Ok(mut child) => {
                child.wait().expect("Failed to build main.cpp");
                println!("✅ Built main.cpp");
            }
            Err(e) => {
                eprintln!("Error building main.cpp: {}", e);
            }
        }
    } else {
        eprintln!("Error: src/main.cpp not found!");
    }
}
