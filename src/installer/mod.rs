const PACKAGE_DIRECTORY: &str = "modules";

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

    pub fn install(&mut self, working_dir: &str, package: &str) {
        if !modules_file_exists(working_dir) {
            println!("No modules.json file found in the working directory.");
            return;
        }

        self.package_name = parse_package_name(package);
        self.version = parse_version(package);
        if self.version.is_empty() {
            self.version = "latest".to_string();
        }
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
