use dirs::home_dir;
use std::{fs, io::Write, path::Path};

use crate::file_contents;

pub fn expand_home_directory(path: &str) -> String {
    if path.starts_with("~/") {
        if let Some(home) = home_dir() {
            return home.join(&path[2..]).to_string_lossy().into_owned();
        }
    }
    path.to_string()
}

pub fn write_to_file(file_path: &Path, content: &[u8]) {
    match fs::File::create(file_path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(content) {
                eprintln!("Failed to write to file: {}", e);
            }
        }
        Err(e) => eprintln!("Failed to create file: {}", e),
    }
}

pub fn create_license(file_path: &Path) {
    write_to_file(file_path, file_contents::MIT_LICENSE_CONTENT.as_bytes());
}

pub fn create_readme(file_path: &Path, extra_content: &str) {
    let content = format!(
        "\
# {}

",
        extra_content
    );
    write_to_file(file_path, content.as_bytes());
}

pub fn extract_repo_name_from_url(git_url: &str) -> String {
    git_url
        .split('/')
        .filter(|&s| !s.is_empty()) // Ensure empty segments are not considered
        .last() // Get the last segment which should be the repository name or name.git
        .map(|name| {
            name.strip_suffix(".git") // Attempt to strip '.git' if present
                .unwrap_or(name) // If '.git' is not present, return the original segment
        })
        .unwrap_or("unknown") // Fallback to "unknown" if no segments are found
        .to_string()
}

// fn get_git_config(config_key: &str) -> String {
//     let output = Command::new("git")
//         .args(["config", config_key])
//         .output()
//         .expect("failed to execute process");
//
//     if output.status.success() {
//         String::from_utf8_lossy(&output.stdout).trim().to_string()
//     } else {
//         eprintln!("Error getting git config for {}", config_key);
//         std::process::exit(1);
//     }
// }
// fn main() {
//     let user_name = get_git_config("user.name");
//     let user_email = get_git_config("user.email");
//
//     println!("Git User Name: {}", user_name);
//     println!("Git User Email: {}", user_email);
// }
