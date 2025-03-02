use std::path::Path;
use std::process::{Command, ExitStatus};
use std::{fs, io};

use static_files::resource_dir;

static UI_DIR: &str = "../";
static UI_DIST_DIR: &str = "../client/dist";
static STATIC_DIR: &str = "./target/generated";

#[cfg(windows)]
static NPM_CMD: &str = "npm.cmd";
#[cfg(not(windows))]
static NPM_CMD: &str = "npm";

fn main() {
    println!("Build UI - build.rs!");

    build_ui().expect("Error while building UI");

    copy_dir_all(UI_DIST_DIR, STATIC_DIR).expect("Failed to copy UI files");
    resource_dir("./target/generated").build().unwrap();
}

fn install_ui_deps() -> io::Result<ExitStatus> {
    if !Path::new("../node_modules").exists() {
        println!("Installing node dependencies...");
        Command::new(NPM_CMD)
            .args(["ci"])
            .current_dir(UI_DIR)
            .status()
    } else {
        Ok(ExitStatus::default())
    }
}

fn build_ui() -> io::Result<ExitStatus> {
    if !Path::new(UI_DIST_DIR).exists() || Path::new(UI_DIST_DIR).read_dir()?.next().is_none() {
        install_ui_deps()?;

        println!("Building UI...");
        Command::new(NPM_CMD)
            .args(["run", "build"])
            .current_dir(UI_DIR)
            .status()
    } else {
        println!("Using previously built UI files");
        Ok(ExitStatus::default())
    }
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
