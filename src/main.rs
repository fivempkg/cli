// fxpkg add test_module@latest
// fxpkg add test_module

pub mod installer;
pub mod registry;

use clap::{Args, Parser, Subcommand};
use std::env;

#[derive(Debug, Parser)]
#[command(name = "fxpkg")]
#[command(about = "A package manager for FiveM")]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Install {},
}

#[derive(Debug, Args)]
struct InstallArgs {
    package: String,
}

fn main() {
    let args = Cli::parse();

    match args.commands {
        Commands::Install {} => {
            println!("Installing package...");
        }
    }
}

/*#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let package = env::args().nth(2).expect("missing package");
    let mut pkg = installer::PackageInstall::new();

    pkg.install(package.as_str());

    match registry::Registry::download_package("module.lua").await {
        Ok(_) => println!("Package downloaded successfully"),
        Err(_) => eprintln!("Failed to download package"),
    }
} */
