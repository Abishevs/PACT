use clap::{builder::PossibleValuesParser, Arg, Command};
use pact_cli2::{load_config, run_clone_project, run_new_project};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config("pact_templates/config.yaml")?;
    let type_choices_static: Vec<&'static str> = config
        .types
        .keys()
        .cloned()
        .map(|s| Box::leak(s.into_boxed_str()) as &'static str)
        .collect();

    let new_choices_static: Vec<&'static str> = config
        .new
        .as_ref()
        .unwrap()
        .keys()
        .cloned()
        .map(|s| Box::leak(s.into_boxed_str()) as &'static str)
        .collect();

    let matches = Command::new("pact")
        .about("P.A.C.T. - Project Automation, Configuration, and Terminal Multiplexing")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("new")
                .about("Create a new project")
                .arg(
                    Arg::new("type")
                        .help("Project type")
                        .value_parser(PossibleValuesParser::new(type_choices_static.clone()))
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("lang")
                        .help("Project language")
                        .value_parser(PossibleValuesParser::new(new_choices_static.clone()))
                        .required(true)
                        .index(2),
                )
                .arg(Arg::new("project_name").required(true).index(3))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("clone")
                .about("Clone a project")
                .arg(
                    Arg::new("type")
                        .help("Project type")
                        .value_parser(PossibleValuesParser::new(type_choices_static))
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("lang")
                        .help("Project language")
                        .value_parser(PossibleValuesParser::new(new_choices_static))
                        .required(true)
                        .index(2)
                )
                .arg(Arg::new("url").required(true).index(3))
                .arg(Arg::new("new_name").required(false).index(4))
                .arg_required_else_help(true),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("new", sub)) => {
            let t = sub.get_one::<String>("type").unwrap();
            let l = sub.get_one::<String>("lang").unwrap();
            let name = sub.get_one::<String>("project_name").unwrap();
            run_new_project(&config, l, t, name);
        }
        Some(("clone", sub)) => {
            let t = sub.get_one::<String>("type").unwrap();
            let l = sub.get_one::<String>("lang").unwrap();
            let url = sub.get_one::<String>("url").unwrap();
            let new_name = sub.get_one::<String>("new_name");
            run_clone_project(&config, l, t, url, new_name);
        }
        _ => {}
    }

    Ok(())
}
