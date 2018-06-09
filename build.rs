#[cfg(windows)]
extern crate winres;

use std::process::Command;

#[cfg(windows)]
fn main() {
    let version = git_semver();
    set_env(&version);

    let mut res = winres::WindowsResource::new();
    res.set_icon("app.ico")
        .set_language(0x0409)
        .set("FileDescription", &format!("bincrypt {}", version));
    res.compile().unwrap();
}

#[cfg(not(windows))]
fn main() {
    set_env(&git_semver());
}

fn git_semver() -> String {
    let cmd = Command::new("git")
        .args(&["describe", "--always", "--dirty=-dirty"])
        .output()
        .unwrap();
    assert!(cmd.status.success());
    std::str::from_utf8(&cmd.stdout[..])
        .unwrap()
        .trim()
        .to_string()
}

fn set_env(version: &str) {
    println!("cargo:rustc-env=GIT_PKG_VERSION_SEMVER={}", version);
    println!("cargo:rerun-if-changed=(nonexistentfile)");
}
