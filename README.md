# PACT (Project Automation and Cloning Tool)

PACT (Project Automation and Cloning Tool) is a lightweight command-line
utility written in C. It simplifies the process of creating and managing
project directories. With configurable templates and support for Git and tmux,
PACT provides a to extent customizable workflow for developers.
Yet it's still my vision of how I like to work meaning it won't work for
everyone and that's not the point of this tool either.


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
    cargo install
    ```
3. Edit templates to your liking
