mod commands {
    pub mod add;
    pub mod build;
    pub mod install;
    pub mod new;
    pub mod remove;
    pub mod run;
    pub mod clean;
    pub mod search;
}

use clap::{Parser, Subcommand};
use commands::{add, build, clean, install, new, remove, run, search};

#[derive(Parser)]
#[command(name = "cppgo")]
#[command(version = "0.1.0")]
#[command(about = "A lightweight C++ package manager", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New { name: String },
    Add { package: String },
    Remove { package: String },
    Install,
    Build,
    Run,
    Search { package: String },
    Clean,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { name } => {
            println!("Creating new project: {}", name);
            new::create_new_project(name);
        }
        Commands::Add { package } => {
            println!("Adding dependency: {}", package);
            add::add_dependency(package);
        }
        Commands::Remove { package } => {
            println!("Removing dependency: {}", package);
            remove::remove_dependency(package);
        }
        Commands::Install => {
            println!("Installing dependencies...");
            install::install_dependencies();
        }
        Commands::Build => {
            println!("Building project...");
            build::build_project();
        }
        Commands::Run => {
            println!("Running project...");
            install::install_dependencies();
            build::build_project();
            run::run_project();
        }
        Commands::Search { package } => {
            println!("Searching for package: {}", package);
            search::search_package(package);
        }
        Commands::Clean => {
            println!("Cleaning project...");
            clean::clean_project();
        }
    }
}
