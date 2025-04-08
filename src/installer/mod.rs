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

    pub fn install(&mut self, package: &str) {
        self.package_name = parse_package_name(package);

        self.version = parse_version(package);
        if self.version.is_empty() {
            self.version = "latest".to_string();
        }

        println!(
            "Installing package: {} with version: {}",
            self.package_name, self.version
        );
    }
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
