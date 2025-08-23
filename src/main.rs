use clap::{Arg, Command as ClapCommand};
use serde::Deserialize;
use std::{collections::HashMap, fs, path::Path, sync::Arc};

#[derive(Debug, Deserialize)]
struct Config {
    types: Vec<TypeConfig>,
    new: Option<HashMap<String, CommandBlock>>,
    clone: Option<HashMap<String, CommandBlock>>,
}

#[derive(Debug, Deserialize)]
struct TypeConfig {
    t: String,
    alias: String,
}

#[derive(Debug, Deserialize)]
struct CommandBlock {
    dir: Option<String>,
    pre: Option<Vec<Step>>,
    main: Option<Vec<Step>>,
    post: Option<Vec<Step>>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Step {
    Run { run: Cmd },
    Clone { clone: Option<()> },
}

#[derive(Debug, Deserialize)]
struct Cmd {
    cmd: String,
    args: Vec<String>,
}

fn main() {
    let config_path = Path::new("pact_templates/config.yaml");
    let yaml = fs::read_to_string(config_path).expect("Failed to read config");
    let config: Config = serde_yaml::from_str(&yaml).expect("Invalid YAML");

    let mut lang_choices: Vec<String> = vec![];
    if let Some(new_map) = &config.new {
        lang_choices.extend(new_map.keys().cloned());
    }
    if let Some(clone_map) = &config.clone {
        lang_choices.extend(clone_map.keys().cloned());
    }
    lang_choices.sort();
    lang_choices.dedup();
    let lang_choices = Arc::new(lang_choices);

    let mut type_choices: Vec<TypeConfig> = vec![];
    for t in &config.types {
        type_choices.push(TypeConfig { t: t.t.clone(), alias: t.alias.clone() });
    }

    let type_choices = Arc::new(type_choices);

    let app = ClapCommand::new("pact")
        .about("Test CLI")
        .subcommand(
            ClapCommand::new("new")
            .about("Create a new project")
            .after_help(&format!(
                    "Available languages: {}\nAvailable types: {}",
                    lang_choices.join(", "),
                    type_choices
                    .iter()
                    .map(|tc| format!("({}, {})", tc.alias, tc.t))
                    .collect::<Vec<_>>()
                    .join(", ")
            ))
            .arg(
                Arg::new("lang")
                .required(true)
                .help("Project language")
                .value_parser({
                    let lang_choices = Arc::clone(&lang_choices);
                    move |s: &str| {
                        if lang_choices.contains(&s.to_string()) {
                            Ok(s.to_string())
                        } else {
                            Err(format!("Invalid language: {s}"))
                        }
                    }
                }),
            )
            .arg(
                Arg::new("project_name")
                .required(true)
                .help("Name of the project"),
            )
            .arg(
                Arg::new("type")
                .required(true)
                .help("Project type (t or alias)")
                .value_parser({
                    let type_choices = Arc::clone(&type_choices);
                    move |s: &str| {
                        if type_choices.iter().any(|tc| tc.t == s || tc.alias == s) {
                            Ok(s.to_string())
                        } else {
                            Err(format!("Invalid type: {s}"))
                        }
                    }
                }),
            ),
    )
        .subcommand(
            ClapCommand::new("clone")
            .about("Clone a project")
            .after_help(&format!(
                    "Available languages: {}\nAvailable types: {}",
                    lang_choices.join(", "),
                    type_choices
                    .iter()
                    .map(|tc| format!("({}, {})", tc.t, tc.alias))
                    .collect::<Vec<_>>()
                    .join(", ")
            ))
            .arg(
                Arg::new("lang")
                .required(true)
                .help("Project language")
                .value_parser({
                    let lang_choices = Arc::clone(&lang_choices);
                    move |s: &str| {
                        if lang_choices.contains(&s.to_string()) {
                            Ok(s.to_string())
                        } else {
                            Err(format!("Invalid language: {s}"))
                        }
                    }
                }),
            )
            .arg(
                Arg::new("project_name")
                .required(true)
                .help("Name of the project"),
            )
            .arg(
                Arg::new("type")
                .required(true)
                .help("Project type (t or alias)")
                .value_parser({
                    let type_choices = Arc::clone(&type_choices);
                    move |s: &str| {
                        if type_choices.iter().any(|tc| tc.t == s || tc.alias == s) {
                            Ok(s.to_string())
                        } else {
                            Err(format!("Invalid type: {s}"))
                        }
                    }
                }),
            )
            );

    let matches = app.clone().get_matches();

    match matches.subcommand() {
        Some(("new", sub_m)) => {
            let lang = sub_m.get_one::<String>("lang").unwrap();
            let project_name = sub_m.get_one::<String>("project_name").unwrap();
            let project_type = sub_m.get_one::<String>("type").unwrap();

            println!("CLI Input (new):");
            println!("  lang: {}", lang);
            println!("  project_name: {}", project_name);
            println!("  type: {}", project_type);

            if let Some(block) = config.new.as_ref().and_then(|m| m.get(lang)) {
                println!("Found new block for {}: {:?}", lang, block);
            }
        }
        Some(("clone", sub_m)) => {
            let lang = sub_m.get_one::<String>("lang").unwrap();
            let project_name = sub_m.get_one::<String>("project_name").unwrap();
            let project_type = sub_m.get_one::<String>("type").unwrap();

            println!("CLI Input (clone):");
            println!("  lang: {}", lang);
            println!("  project_name: {}", project_name);
            println!("  type: {}", project_type);

            if let Some(block) = config.clone.as_ref().and_then(|m| m.get(lang)) {
                println!("Found clone block for {}: {:?}", lang, block);
            }
        }
        _ => println!("Unknown or no subcommand"),
    }
}

