// fxpkg add test_module@latest
// fxpkg add test_module

/**
 *
 * do we create a catetory or just a directory for modules?
 * do we ensure [package] to the server.cfg?
 *
 * [fx_modules]/pure_lua_resource
 *  metadata.json
 *      - contains version number
 *  fx_manifest.lua
 *  main.lua
 *
 *  ----- [some_resource/fxmanifest.lua]
 *  shared_scripts "fx_modules/pure_lua_resource"
 *
 *
 *
*/
pub mod installer;
pub mod registry;

use clap::{Args, Parser, Subcommand};
use std::env;
use std::path::Path;

#[derive(Debug, Parser)]
#[command(name = "fxpkg")]
#[command(about = "A package manager for FiveM")]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Install(InstallArgs),
    Init,
}

#[derive(Debug, Args)]
struct InstallArgs {
    /// The package to install
    name: String,
}

fn find_resources_directory(current_path: &Path) -> Result<String, String> {
    let mut path = current_path.to_path_buf();
    
    // Check if current directory is already "resources"
    if path.file_name().and_then(|n| n.to_str()) == Some("resources") {
        return Ok(path.to_string_lossy().to_string());
    }
    
    // Traverse upwards to find "resources" directory
    loop {
        let resources_path = path.join("resources");
        if resources_path.exists() && resources_path.is_dir() {
            return Ok(resources_path.to_string_lossy().to_string());
        }
        
        // Move to parent directory
        if let Some(parent) = path.parent() {
            path = parent.to_path_buf();
        } else {
            break;
        }
    }
    
    Err("Could not find 'resources' directory. Please run 'fxpkg init' from within or above a 'resources' directory.".to_string())
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let args = Cli::parse();

    let pwd = env::current_dir().unwrap();

    match args.commands {
        Commands::Install(package) => {
            match find_resources_directory(&pwd) {
                Ok(resources_path) => {
                    let mut pkg_installer = installer::PackageInstall::new();
                    pkg_installer
                        .install(&resources_path, &package.name)
                        .await;
                }
                Err(error_msg) => {
                    eprintln!("Error: {}", error_msg);
                    std::process::exit(1);
                }
            }
        }
        Commands::Init => {
            match find_resources_directory(&pwd) {
                Ok(resources_path) => {
                    installer::init_fxpkg(&resources_path);
                    println!("Initialized fxpkg in: {}", resources_path);
                }
                Err(error_msg) => {
                    eprintln!("Error: {}", error_msg);
                    std::process::exit(1);
                }
            }
        }
    }
}
