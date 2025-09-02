use crate::config::{merge_phases, Config, Phases, Step};

pub fn run_new_project(config: &Config, lang: &str, project_type: &str, project_name: &str) {
    let project_steps = &config.new.as_ref().unwrap()[lang];
    let merged_phases = merge_phases(
        config.common.as_ref().map(|c| &c.phases),
        Some(&project_steps.phases),
    );
    println!("New project {project_name} of type {project_type} in {lang}");
    print_steps(&merged_phases);
}

pub fn run_clone_project(
    config: &Config,
    lang: &str,
    project_type: &str,
    url: &str,
    new_name: Option<&String>,
) {
    let project_steps = &config.clone.as_ref().unwrap()[lang];
    let merged_phases = merge_phases(
        config.common.as_ref().map(|c| &c.phases),
        Some(&project_steps.phases),
    );
    println!("Clone {url} as {new_name:?} of type {project_type} in {lang}");
    print_steps(&merged_phases);
}

fn print_steps(phases: &Phases) {
    for (phase_name, steps_opt) in [
        ("Pre", &phases.pre),
        ("Main", &phases.main),
        ("Post", &phases.post),
    ] {
        if let Some(steps) = steps_opt {
            println!("-- {phase_name}:");
            for step in steps {
                match step {
                    Step::Cmd(s) => println!("  CMD: {s}"),
                    Step::Clone { .. } => println!("  CLONE"),
                    Step::GitInit { .. } => println!("  GIT INIT"),
                }
            }
        }
    }
}
