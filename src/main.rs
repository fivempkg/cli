// fxpkg add test_module@latest
// fxpkg add test_module

use std::env;
pub mod installer;

fn main() {
    let package = env::args().nth(2).expect("missing package");

    let mut pkg = installer::PackageInstall::new();
    pkg.install(package.as_str());
}
