# PACT (Project Automation and Cloning Tool)

PACT (Project Automation and Cloning Tool) is a unified itnerface for
project scaffolding. A wrapper around other project scafolliding cli commands.
With support configurable templates and support for Git and tmux,
PACT provides a to extent customizable workflow for developers.

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

Am using [tmux-sessionizer](https://crates.io/crates/tmux-sessionizer) which expects a config of dirs of git repos
For easy managment they are all placed in the $HOME/dev dir. So this tool
integrates nicely into this workflow. New projects are not started that often,
but cloning is done often. This makes it trivial to do with a minimal mental
overhead. No more thinking to where to place the repos and no cd and chaning
git clone name etc.

## Features
### Config
My config is stored in .config/pact/<langs>.zsh
and are managed by stow in [my .dotfiles/packages/pact/.config/pact
repo](https://github.com/Abishevs/.dotfiles)
You can also define _<helpers>.zsh in the config, by sourcing those files you
can get common functions for configs, also functions that are not named
<lang>_<foo>() will be ignored for the parser use that for modulurasing your
configs.

- **New Project Creation**:
  - pact new <type> <lang> [extra] <project_name>
  1. Creates Dir struct <base>/<type>/<lang>/<project_name>, closes if exists
  2. Initializes Git repo
  3. Starts a tmux session for the project.
  4. ALL cmds from the config are run inside the tmux session. Uses <lang> to run default <lang>_default() 
  5. If [extra] is present, then it would use <lang>_<extra>() function
     instead, if exists else closes.
  6. Attaches to the tmux session (by creating new server, or using existing
     one)

- **Clone Existing Repositories**:
  - Clones a Git repository into a structured directory.
  - Optionally renames the cloned repository.
  - Starts a tmux session for the cloned project.


## Installation

```bash
git clone git@github.com:Abishevs/PACT.git
cd pact
make install
```

## removing
```bash
make uninstall
```

