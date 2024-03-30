# P.A.C.T. - Project Automation, Configuration, and Terminal Multiplexing 
A tool for project creation/ cloning. Organising them by project type
(personal/work/shool/test) => language (py, rust, c, arduino, etc) => project
name. initilises a git repo. Then def files are created e.g. MIT LICENSE, simple
README and .gitignore. Then tmux session is started with session-name as project
name (for use with TMS-Tmux sessionizer). Then if lang has specific env to start
like py venv, it will be started for you in tmux.  

## Installation
### Ensure rust PATH is setup
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```
Then install it user-wide
```bash
cargo install --path .
```
