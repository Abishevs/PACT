use std::{fs::{create_dir_all, File}, path::Path, process::Command};

use crate::{cli::NewProject, file_contents, utils::{create_license, create_readme, write_to_file}};

pub fn setup_python_project(project_dir: &str, new_project: &NewProject) {
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
