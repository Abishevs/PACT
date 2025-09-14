#!/usr/bin/env zsh

# Map short aliases to scopes
declare -A TYPES LANGS
TYPES=( p personal w work s school )
LANGS=( py python rs rust )

CONFIG_DIR="$HOME"/.config/pact
BASE_DIR="$HOME"/dev

tmux_attach() {
  local SESSION="$1"

  if [[ -n "$TMUX" ]]; then
    # already inside tmux, switch
    tmux switch-client -t "$SESSION"
  else
    # not in tmux, attach normally
    tmux attach -t "$SESSION"
  fi
}

# ------------------- #
# Subcommand function #
# ------------------- #

pact_new() {
    if [[ $# -lt 3 ]]; then
        echo "Error: missing arguments"
        echo "Usage: pact new <type> <language> [option] <project_name>"
        echo "option: like bin, lib etc defined in config langs"
	echo "without option it will run <lang>_default() functions from the config"
        return 1
    fi

    local TYPE="$1"; shift # First Arg, move to the next
    local LANG="$1"; shift # -||-

    local EXTRA=""
    local PROJECT=""

    if [[ $# -gt 1 ]]; then
        EXTRA="$1"; shift
    fi

    PROJECT="$1"; shift || true

    TYPE="${TYPES[$TYPE]:-$TYPE}"
    LANG="${LANGS[$LANG]:-$LANG}"

    # echo "Scope: $TYPE"
    # echo "Language: $LANG"
    # echo "Extra option: $EXTRA"
    # echo "Project: $PROJECT"

    local MODULE_FILE="$CONFIG_DIR/${LANG}.zsh"
    if [[ ! -f $MODULE_FILE ]]; then
        echo "Module not found: $MODULE_FILE"
        return 1
    fi

    # Check optional function passed in as arg
    local FUNC=""
    if [[ -n "$EXTRA" ]]; then
        FUNC="${LANG}_${EXTRA}"
    else
        FUNC="${LANG}_default"
    fi

    # Load module in the current shell just to check function existence
    source "$MODULE_FILE"
    if ! typeset -f "$FUNC" >/dev/null; then
	    echo "Error: function '$FUNC' not found in module $MODULE_FILE"
	    return 1
    fi

    local PROJECT_DIR="${BASE_DIR}/${TYPE}/${LANG}/${PROJECT}"

    if [[ -d $PROJECT_DIR ]]; then
        echo "Project already exists"
	return 1
    fi

    mkdir -p "$PROJECT_DIR" 
    cd "$project_DIR"

    # Start a new detached tmux session that runs the module + function
    tmux new-session -d -s "$PROJECT" -c "$PROJECT_DIR" \
	    "git init; source '$MODULE_FILE'; $FUNC '$PROJECT'; exec zsh"

    # Attach to it
    tmux_attach "$PROJECT"
}

pact_clone() {
    if [[ $# -lt 3 ]]; then
        echo "Error: missing arguments"
        echo "Usage: pact clone <type> <language> <git_url> [project_name]"
        return 1
    fi

    local TYPE="$1"; shift
    local LANG="$1"; shift

    local GIT_URL="$1"; shift
    local PROJECT=""

    if [[ $# -ge 1 ]]; then
        PROJECT="$1"; shift # Rename the cloned repo from args
    else
	PROJECT="${GIT_URL##*/}" # Strip everything before last /
	PROJECT="${PROJECT%.git}" # Strip optional .git extension
    fi
    echo "$#"

    TYPE="${TYPES[$TYPE]:-$TYPE}"
    LANG="${LANGS[$LANG]:-$LANG}"

    PROJECT_DIR="${BASE_DIR}/$TYPE/$LANG/$PROJECT"

    # echo "Type: $TYPE"
    # echo "Lang: $LANG"
    # echo "URL: $GIT_URL"
    # echo "Project_name: $PROJECT"
    # echo "CLONE_PATH: $PROJECT_DIR"
    
    tmux new-session -d -s "$PROJECT" -c "$PROJECT_DIR" \
	    "git clone $GIT_URL $PROJECT_DIR; cd $PROJECT_DIR; exec zsh"
    tmux_attach "$PROJECT"

}
pact_help() {
  cat <<EOF
P.A.C.T. - The project -caffolding final bosses assitant

Usage:
  pact <command> [options] [args]

Commands:
  new <type> <lang> [flags] <project_name>   Create a new project
  clone <type> <lang> <repo> [project_name]  Clone a project template

Examples:
  pact new personal py --lib my_project
  pact p rs clone git@github.com:user/repo.git

EOF
}

# ------------- #
# Main dispatch #
# ------------- #
COMMAND="$1"
shift || true

case "$COMMAND" in
  new)
    pact_new "$@"
    ;;
  clone)
    pact_clone "$@"
    ;;
  help|--help|-h|"")
    pact_help
    ;;
  *)
    echo "Unknown command: $COMMAND"
    pact_help
    exit 1
    ;;
esac

