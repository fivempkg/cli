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

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let args = Cli::parse();

    let pwd = env::current_dir().unwrap();

    match args.commands {
        Commands::Install(package) => {
            let mut pkg_installer = installer::PackageInstall::new();
            pkg_installer
                .install(pwd.to_str().unwrap(), &package.name)
                .await;
        }
        Commands::Init => {
            let pwd = env::current_dir().unwrap();
            let path = pwd.to_str().unwrap();

            installer::init_fxpkg(path);
        }
    }
}
