use clap::{Args, Parser, Subcommand};

use crate::constants::BASE_DIR;
use crate::project_setup::arduino_setup::setup_arduino_project;
use crate::project_setup::generic_project::generic_project_setup;
use crate::project_setup::python_setup::setup_python_project;
use crate::project_setup::rust_setup::setup_rust_project;
use crate::types::{Language, ProjectType};
use crate::utils::{expand_home_directory, extract_repo_name_from_url};
use std::process::Command;
// use std::fs::{self, File, create_dir_all, create_dir};
use std::{env, fs, io, process};
// use std::io::{self, Write};
use std::path::Path;

#[derive(Parser)]
#[command(version,
          about = "P.A.C.T. - Project Automation, Configuration, and Terminal Multiplexing",
          long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    New(NewProject),
    Clone(CloneProject),
}

#[derive(Args, Debug)]
pub struct NewProject {
        #[arg(help = "Project Name e.g. pact")]
        pub project_name: String, 
        pub language: Language,
        pub project_type: ProjectType,
        #[arg(short, long, action = clap::ArgAction::SetTrue, help = "Simple project structure")]
        pub advanced: bool,

}

#[derive(Args, Debug)]
pub struct CloneProject {


    pub language: Language,
    pub project_type: ProjectType,

    #[arg(help = "Git URL to clone")]
    pub url: String,
    #[arg(help = "Clone into dir with new name")]
    pub new_name: Option<String>,
}


pub fn process_new_project(new_project: &NewProject) {
    let project_dir = match create_project_dir(&new_project) {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("Failed to create project directory: {}", e);
            process::exit(1);
        },
    };

    match new_project.language {
        Language::Python | Language::Py => {
            // println!("Creating a Python project: {:?}", new_project.project_type);
            setup_python_project(&project_dir, &new_project);
        },
        Language::Rust | Language::Rs => {
            if let Err(e) = setup_rust_project(&project_dir, &new_project) {
                eprintln!("Failed to set up Rust project: {}", e);
                process::exit(1);
            }
        },

        Language::Arduino => {
            setup_arduino_project(&project_dir, &new_project);
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

pub fn create_project_dir(new_project: &NewProject) -> Result<String, io::Error>{
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
    // println!("Project to be created: {}", project_dir);
    initialize_git_repo(&project_dir)?;

    Ok(project_dir)

}


pub fn get_language_setup_command(language: &Language, project_dir: &str) -> Option<String> {
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

pub fn process_project_clone(new_clone: &CloneProject) -> Result<(), io::Error> {
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
                                 &format!("{}/{}",project_dir, project_name),
                                 &new_clone.language)?;
    Ok(())

}

pub fn initialize_git_repo(project_dir: &str) -> Result<(), std::io::Error> {
    Command::new("git")
        .args(["init", project_dir])
        .status()?;
    // println!("Initialized a new git repository in {}", project_dir);
    Ok(())
}


pub fn start_or_attach_tmux_session(project_name: &str, project_dir: &str, language: &Language) -> Result<(), io::Error> {
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
    } 
    else {
        // Not inside TMUX, create and attach to new session
        let new_session = Command::new("tmux")
            .args(["new-session", "-d", "-s", project_name, "-c", project_dir])
            .output()?;
        
        if !new_session.status.success() {
            let error_message = String::from_utf8_lossy(&new_session.stderr);
            return Err(io::Error::new(io::ErrorKind::Other, error_message.to_string()));

        } 

        Command::new("tmux")
            .args(["attach-session", "-t", project_name])
            .output()?;

    }

    if let Some(setup_command) = get_language_setup_command(&language, &project_dir) {
        // Send the setup command to the tmux session
        Command::new("tmux")
            .args(["send-keys", "-t", project_name, &format!("{}\n", setup_command)])
            .output().unwrap();
    }

    // println!("Attaching to tmux session: {}", project_name);
    Ok(())
}
