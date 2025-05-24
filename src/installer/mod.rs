use std::io::{Read, Write};

use crate::registry::Registry;

const PACKAGE_DIRECTORY: &str = "fx_modules";
const MODULES_FILE: &str = "modules.json";

pub struct PackageInstall {
    package_name: String,
    version: String,
}
impl PackageInstall {
    pub fn new() -> Self {
        PackageInstall {
            package_name: String::new(),
            version: String::new(),
        }
    }

    pub async fn install(&mut self, working_dir: &str, package: &str) {
        if !modules_file_exists(working_dir) {
            println!("No modules.json file found in the working directory.");
            return;
        }

        self.package_name = parse_package_name(package);
        self.version = parse_version(package);
        if self.version.is_empty() {
            self.version = "latest".to_string();
        }

        match Registry::download_package(&self.package_name, &self.version).await {
            Ok(_) => println!("Package downloaded successfully"),
            Err(_) => eprintln!("Failed to download package"),
        }
    }
}

pub fn init_fxpkg(working_dir: &str) {
    let modules_file_path = format!("{}/{}", working_dir, MODULES_FILE);
    if !std::path::Path::new(&modules_file_path).exists() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");

        let stub_path = format!("{}/{}", manifest_dir, "stubs/modules.json");
        let mut stub_file =
            std::fs::File::open(stub_path).expect("Failed to open stub modules.json file");

        let mut stub_content = String::new();
        stub_file
            .read_to_string(&mut stub_content)
            .expect("Failed to read stub modules.json file");

        let file =
            std::fs::File::create(&modules_file_path).expect("Failed to create modules.json file");

        let mut file = std::io::BufWriter::new(file);
        file.write_all(stub_content.as_bytes())
            .expect("Failed to write to modules.json file");
    }
}

fn modules_file_exists(working_dir: &str) -> bool {
    let path = format!("{}/modules.json", working_dir);
    std::fs::metadata(path).is_ok()
}

fn parse_package_name(package: &str) -> String {
    let parts: Vec<&str> = package.split('@').collect();
    if parts.len() > 1 {
        parts[0].to_string()
    } else {
        package.to_string()
    }
}

fn parse_version(package: &str) -> String {
    let parts: Vec<&str> = package.split('@').collect();
    if parts.len() > 1 {
        parts[1].to_string()
    } else {
        String::new()
    }
}
