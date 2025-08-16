use std::{io, path::Path, process::Command};

use crate::{
    cli::NewProject,
    utils::{create_license, create_readme},
};

pub fn setup_rust_project(project_dir: &str, new_project: &NewProject) -> Result<(), io::Error> {
    let project_path = Path::new(&project_dir);
    let license_path = project_path.join("LICENSE");
    let readme_path = project_path.join("README.md");
    create_license(&license_path);
    create_readme(&readme_path, &new_project.project_name);

    // Attempt to run `cargo` commands or other setup steps
    let output = Command::new("cargo")
        .arg("init")
        .arg(project_dir)
        .output()?;

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(io::Error::new(
            io::ErrorKind::Other,
            error_message.to_string(),
        ));
    }

    // println!("Rust project setup in {}", project_dir);
    Ok(())
}
