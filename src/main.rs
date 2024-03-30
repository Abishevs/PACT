mod file_contents;
use std::fs::{self, File, create_dir_all};
use std::process;
use std::fmt;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::env;
use clap::{Args, Parser, ValueEnum, Subcommand};
use dirs::home_dir;

const BASE_DIR: &str = "~/dev";

#[derive(Parser, Debug, ValueEnum, Clone, Copy)]
enum Language {
    Py,
    Python,
    Rs,
    Rust,
    C,
    EmbedC,
    Cpp,
    Esp32,
    EmbdedRs,
    Arduino,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let variant_str = match self {
            Language::Python | Language::Py => "python",
            Language::Rs | Language::Rust => "rust",
            Language::C => "c",
            Language::Cpp => "cpp",
            Language::Esp32 | Language::EmbedC => "embedded-c", // esp-idf
            Language::EmbdedRs => "embedded-rust", 
            Language::Arduino => "embedded-cpp",
        };
        write!(f, "{}", variant_str)
    }
}

#[derive(Parser, Debug, ValueEnum, Clone, Copy)]
enum ProjectType{
    Personal,
    School,
    Work,
    Test,
}

impl fmt::Display for ProjectType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let variant_str = match self {
            ProjectType::Personal => "personal",
            ProjectType::School => "school",
            ProjectType::Work => "work",
            ProjectType::Test => "test",
        };
        write!(f, "{}", variant_str)
    }
}

#[derive(Parser)]
#[command(version,
          about = "P.A.C.T. - Project Automation, Configuration, and Terminal Multiplexing",
          long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New(NewProject),
    Clone(CloneProject),
}

#[derive(Args, Debug)]
struct NewProject {
        #[arg(help = "Project Name e.g. pact")]
        project_name: String, 
        language: Language,
        project_type: ProjectType,
        #[arg(short, long, action = clap::ArgAction::SetTrue, help = "Simple project structure")]
        advanced: bool,

}

#[derive(Args, Debug)]
struct CloneProject {


    language: Language,
    project_type: ProjectType,

    #[arg(help = "Git URL to clone")]
    url: String,
    #[arg(help = "Clone into dir with new name")]
    new_name: Option<String>,
}

fn expand_home_directory(path: &str) -> String {
    if path.starts_with("~/") {
        if let Some(home) = home_dir() {
            return home.join(&path[2..]).to_string_lossy().into_owned();
        }
    }
    path.to_string()
}

fn write_to_file(file_path: &Path, content: &[u8]) {
    match fs::File::create(file_path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(content) {
                eprintln!("Failed to write to file: {}", e);
            }
        },
        Err(e) => eprintln!("Failed to create file: {}", e),
    }
}

fn create_license(file_path: &Path) {
    write_to_file(file_path, file_contents::MIT_LICENSE_CONTENT.as_bytes());
}

fn create_readme(file_path: &Path, extra_content: &str) {
    let content = format!("\
# {} 

", extra_content);
   write_to_file(file_path, content.as_bytes()); 

}

fn create_project_dir(new_project: &NewProject) -> Result<String, io::Error>{
    let project_name = &new_project.project_name;
    let project_type = &new_project.project_type;
    let project_language = &new_project.language;
    let relative_project_dir = format!("{}/{}/{}/{}", BASE_DIR , project_type, project_language, project_name); 
    let project_dir = expand_home_directory(&relative_project_dir);
    let path = Path::new(&project_dir);

    if path.exists() {
        return Err(io::Error::new(io::ErrorKind::AlreadyExists, "Project directory already exists."));
    }

    fs::create_dir_all(&project_dir)?;  
    println!("Project to be created: {}", project_dir);
    initialize_git_repo(&project_dir)?;

    Ok(project_dir)

}

fn initialize_git_repo(project_dir: &str) -> Result<(), std::io::Error> {
    Command::new("git")
        .args(["init", project_dir])
        .status()?;
    // println!("Initialized a new git repository in {}", project_dir);
    Ok(())
}

fn get_language_setup_command(language: &Language, project_dir: &str) -> Option<String> {
    let project_dir_path = Path::new(&project_dir);
    
    match language {
        Language::Py | Language::Python => {
            let venv_path = project_dir_path.join("venv");
            if venv_path.exists() {

                // Check if venv exists if not ignore this step aka return none
                let venv_activation_cmd = format!("source {}/venv/bin/activate", project_dir);
                Some(venv_activation_cmd)
            } else {None}
        },
        _ => None,
    }
}

fn start_or_attach_tmux_session(project_name: &str, project_dir: &str, language: &Language) -> Result<(), io::Error> {
    if env::var("TMUX").is_ok() {
        // Inside TMUX, create a new session in detached mode
        let detached_session = Command::new("tmux")
            .args(["new-session", "-d", "-s", project_name, "-c", project_dir])
            .output()?;
            
        if !detached_session.status.success() {
            let error_message = String::from_utf8_lossy(&detached_session.stderr);
            return Err(io::Error::new(io::ErrorKind::Other, error_message.to_string()));
        }

        // Switch to the newly created session
        let switch_to_session = Command::new("tmux")
            .args(["switch-client", "-t", project_name])
            .output()?;

        if !switch_to_session.status.success() {
            let error_message = String::from_utf8_lossy(&switch_to_session.stderr);
            return Err(io::Error::new(io::ErrorKind::Other, error_message.to_string()));
        }
    } else {
        // Not inside TMUX, create and attach to new session
        let new_session = Command::new("tmux")
            .args(["new-session", "-s", project_name, "-c", project_dir])
            .output()?;
        
        if !new_session.status.success() {
            let error_message = String::from_utf8_lossy(&new_session.stderr);
            return Err(io::Error::new(io::ErrorKind::Other, error_message.to_string()));
        }
    }

    if let Some(setup_command) = get_language_setup_command(&language, &project_dir) {
        // Send the setup command to the tmux session
        Command::new("tmux")
            .args(["send-keys", "-t", project_name, &format!("{}\n", setup_command)])
            .output().unwrap();
    }

    println!("Attaching to tmux session: {}", project_name);
    Ok(())
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

fn setup_python_project(project_dir: &str, new_project: &NewProject) {
    let project_path = Path::new(&project_dir);
    let license_path = project_path.join("LICENSE");
    let readme_path = project_path.join("README.md");
    let gitignore_path = project_path.join(".gitignore");
    let main_py_path = project_path.join("main.py");
    let content_main_py = "\
def main():
    print(\"Hello world\")

if __name__ == \"__main__\":
    main()
";
    
    write_to_file(&main_py_path, content_main_py.as_bytes());
    write_to_file(&gitignore_path, file_contents::PY_GITIGNORE_CONTENT.as_bytes());
    create_license(&license_path);
    create_readme(&readme_path, &new_project.project_name);

    if new_project.advanced {
        let src_dir_str = format!("src/{}", &new_project.project_name);
        let src_path = project_path.join(src_dir_str);
        create_dir_all(&src_path).unwrap();
        let init_path = src_path.join("__init__.py");
        File::create(init_path).unwrap();

        // create .toml
        let toml_path = project_path.join("pyproject.toml");
        let toml_content = format!("\
[build-system]
requires = [\"setuptools>=61.0\"]
build-backend = \"setuptools.build_meta\"

[project]
name = \"{}\"
version = \"0.1.0\"
", &new_project.project_name);

        write_to_file(&toml_path, toml_content.as_bytes())

    }

    // create venv
    Command::new("python3")
        .args(["-m", "venv", &format!("{}/venv", project_dir)])
        .output().unwrap();

}

fn setup_rust_project(project_dir: &str, new_project: &NewProject) -> Result<(), io::Error> {
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
        return Err(io::Error::new(io::ErrorKind::Other, error_message.to_string()));
    }

    println!("Rust project setup in {}", project_dir);
    Ok(())
}

fn generic_project_setup(project_dir: &str, new_project: &NewProject) {
    let project_path = Path::new(&project_dir);
    let license_path = project_path.join("LICENSE");
    create_license(&license_path);

    let readme_path = project_path.join("README.md");
    create_readme(&readme_path, &new_project.project_name);

    initialize_git_repo(&project_dir).unwrap();
}

fn process_new_project(new_project: &NewProject) {
    let project_dir = match create_project_dir(&new_project) {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Failed to create project directory: {}", e);
            process::exit(1);
        },
    };

    match new_project.language {
        Language::Python | Language::Py => {
            println!("Creating a Python project: {:?}", new_project.project_type);
            setup_python_project(&project_dir, &new_project);
        },
        Language::Rust | Language::Rs => {
            if let Err(e) = setup_rust_project(&project_dir, &new_project) {
                eprintln!("Failed to set up Rust project: {}", e);
                process::exit(1);
            }
        },
        _ => {
            generic_project_setup(&project_dir, &new_project);
        }
    }

    // Start or attach to a tmux session, exiting on failure.
    if let Err(e) = start_or_attach_tmux_session(&new_project.project_name,
                                                 &project_dir,
                                                 &new_project.language) {
        eprintln!("Failed to start or attach tmux session: {}", e);
        process::exit(1);
    }

}

fn process_project_clone(new_clone: &CloneProject) -> Result<(), io::Error> {
    let relativ_path = format!("{}/{}/{}", BASE_DIR, new_clone.project_type, new_clone.language);
    let project_dir = expand_home_directory(&relativ_path);
    let project_name: String;
    fs::create_dir_all(&project_dir)?;  

    let mut command = Command::new("git");
    command.arg("-C")
           .arg(&project_dir)
           .arg("clone")
           .arg(&new_clone.url);

    // if new name is present
    if let Some(ref new_name) = new_clone.new_name {
        command.arg(new_name);
        project_name = new_name.to_string();

    } else {
        project_name = extract_repo_name_from_url(&new_clone.url);
    };

    let output = command.output()?;

    if !output.status.success() {
        let error_message = String::from_utf8_lossy(&output.stderr);
        return Err(io::Error::new(io::ErrorKind::Other, error_message.to_string()));
    }

    println!("Cloned repo: '{}' into dir: '{}/{}'", new_clone.url, project_dir, project_name);
    start_or_attach_tmux_session(&project_name,
                                 &project_dir,
                                 &new_clone.language)?;
    Ok(())

}

fn extract_repo_name_from_url(git_url: &str) -> String {
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

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::New(new_project) => {
            println!("Got: {:?}", new_project);
            process_new_project(&new_project);
        },
        Commands::Clone(clone) => {
            if let Err(e) = process_project_clone(&clone) {
                eprintln!("{}", e);
            }
        }

    }

}
