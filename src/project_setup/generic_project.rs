use std::path::Path;

use crate::{
    cli::{initialize_git_repo, NewProject},
    utils::{create_license, create_readme},
};

pub fn generic_project_setup(project_dir: &str, new_project: &NewProject) {
    let project_path = Path::new(&project_dir);
    let license_path = project_path.join("LICENSE");
    create_license(&license_path);

    let readme_path = project_path.join("README.md");
    create_readme(&readme_path, &new_project.project_name);

    initialize_git_repo(&project_dir).unwrap();
}
