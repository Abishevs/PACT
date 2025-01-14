# PACT (Project Automation and Cloning Tool)

PACT (Project Automation and Cloning Tool) is a lightweight command-line
utility written in C. It simplifies the process of creating and managing
project directories. With configurable templates and support for Git and tmux,
PACT provides a to extent customizable workflow for developers.
Yet it's still my vision of how I like to work meaning it won't work for
everyone and that's not the point of this tool either.

Rewrite in C of originaly writen by me but in Rust project (check branch inRust if
intrested).


### Why? 
- Didn't want to write in Rust
- Wanted to learn more C
- Wanted to learn about suckless Philosophy from the insides [Suckless.org](https://suckless.org/philosophy/)
- WHY NOT? C is a simple language and this is a simple project. Thus why the need
  of dependencies?

---

## Features

- **New Project Creation**:
  - Automatically sets up a directory structure based on project type and
    programming language.
  - Copies shared and language-specific templates into the project directory.
  - Initializes Git repositories (if applicable).
  - Starts a tmux session for the project.

- **Clone Existing Repositories**:
  - Clones a Git repository into a structured directory.
  - Optionally renames the cloned repository.
  - Starts a tmux session for the cloned project.

- **Fully Configurable**:
  - All configurations, including base directories, supported languages, and
    templates, are managed in `config.h`
  - No external environment variables required.

---

## Installation
I Store my templates in another repo [PACT_templates](https://github.com/Abishevs/PACT_Templates)
But a template structure is included in pact_templates dir. You can use that

1. Clone the repository:
   ```bash
   git clone git@github.com:Abishevs/PACT.git 
   cd pact
    ```

2. Compile 
   ```bash
    make 
    ```

3. Edit `config.h` to point to correct paths. Note use ABS paths!

4. Recompile & install into `~/.local/bin/`
    ```bash
    make install 
    ```

## Configuration

All paths should be absolute, only templates path is relative to the
PACT_TEMPLATES (which is the root dir of where those are stored).

One can have both templates and or init_command set to NULL.
init command cannot yet use Users aliases, only system wide.

