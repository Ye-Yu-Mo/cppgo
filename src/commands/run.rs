use std::process::Command;
use std::path::Path;

pub fn run_project() {
    // 运行主程序
    let target_dir = Path::new("target/debug/main");
    if target_dir.exists() {
        println!("🚀 Running main program...");
        let run_cmd = Command::new("./target/debug/main")
            .spawn();

        match run_cmd {
            Ok(mut child) => {
                child.wait().expect("Failed to run main");
                println!("✅ Program ran successfully.");
            }
            Err(e) => {
                eprintln!("Error running program: {}", e);
            }
        }
    } else {
        eprintln!("Error: Executable 'main' not found in target/debug/");
    }
}
