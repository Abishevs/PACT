# PACT (Project Automation and Cloning Tool)

PACT (Project Automation and Cloning Tool) is a unified itnerface for
project scaffolding. A wrapper around other project scafolliding cli commands.
With support configurable templates and support for Git and tmux,
PACT provides a to extent customizable workflow for developers.

The tool runs in phases were u can define pre-, main- and post-phases.
My general workflow is to invoke pact to create a new project dir in the
Intialise boiler plate of the project, initialise git, and start a headless
tmux session to which it will attach after the program is done.

Now it will also expose all these phases as a yaml configuration such that user
can defin very different workflows (eg tmux to anther multiplexer, or simply cd
into the project dir).

Yet it's still my vision of how I like to work meaning it won't work for
everyone and that's not the point of this tool either. The point it's flexible 
and you can define it once and then have a unified interface to create new
projects. Mostly suitable for people who start many small projects to learn or
play around.

## Reasoning
Idea mainly came from python projects as those are often small but require some
boilerplate anyways. With this tool u simply provide it and it will output you
a dir with boilerplate python code + start the tmux session and switch to it.
In the post phase u can also start the virtual env, such that when u are
attached to the tmux session it will be already ready to use.

Am using (tmux-sessionizer)[https://crates.io/crates/tmux-sessionizer] which expects a config of dirs of git repos
For easy managment they are all placed in the $HOME/dev dir. So this tool
integrates nicely into this workflow. New projects are not started that often,
but cloning is done often. This makes it trivial to do with a minimal mental
overhead. No more thinking to where to place the repos and no cd and chaning
git clone name etc.

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
1. Install from crates.io
   ```bash
    cargo install pact-cli
    ```

### Or compile from source
   ```bash
   git clone git@github.com:Abishevs/PACT.git
   cd pact
   cargo install --path .
    ```

