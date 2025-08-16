use clap::Parser;

mod file_contents;
mod types;
mod cli;
mod utils;
mod project_setup;
mod constants;
use cli::{process_new_project, Cli, Commands};

use crate::cli::process_project_clone;

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::New(new_project) => {
            // println!("Got: {:?}", new_project);
            process_new_project(&new_project);
        },
        Commands::Clone(clone) => {
            if let Err(e) = process_project_clone(&clone) {
                eprintln!("{}", e);
            }
        }

    }

}
