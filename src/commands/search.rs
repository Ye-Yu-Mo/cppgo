use std::collections::HashMap;

/// 模拟可用的包列表
fn get_available_packages() -> HashMap<&'static str, &'static str> {
    let mut packages = HashMap::new();
    packages.insert("spdlog", "1.0.0");
    packages.insert("fmtlib", "2.2.0");
    packages.insert("nlohmann_json", "3.11.2");
    packages.insert("gtest", "1.14.0");
    packages
}

/// 搜索包
pub fn search_package(query: &str) {
    let packages = get_available_packages();
    let mut found = false;

    for (name, version) in packages.iter() {
        if name.contains(query) {
            println!("📦 {} v{}", name, version);
            found = true;
        }
    }

    if !found {
        println!("❌ No package found matching '{}'", query);
    }
}
