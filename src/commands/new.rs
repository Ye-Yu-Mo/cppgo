use std::fs;
use std::io::Write;
use std::path::Path;

pub fn create_new_project(name: &str) {
    let project_path = Path::new(name);
    if project_path.exists() {
        eprintln!("Error: directory '{}' already exists.", name);
        return;
    }

    // 创建项目目录和 src 目录
    fs::create_dir_all(project_path.join("src")).expect("Failed to create project directories");

    // 创建 cppgo.toml
    let toml_content = format!(
        "[package]
name = \"{}\"
version = \"0.1.0\"
edition = \"2025\"

[dependencies]
",
        name
    );

    fs::write(project_path.join("Cppgo.toml"), toml_content).expect("Failed to write Cppgo.toml");

    // 创建 main.cpp
    let main_cpp_content = format!(
        "#include <iostream>

int main() {{
    std::cout << \"Hello from {}!\" << std::endl;
    return 0;
}}
",
        name
    );

    let mut main_file =
        fs::File::create(project_path.join("src/main.cpp")).expect("Failed to create main.cpp");
    main_file
        .write_all(main_cpp_content.as_bytes())
        .expect("Failed to write main.cpp");

    println!("✅ Created new C++ project '{}'", name);
}
