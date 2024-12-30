/*
 * PACT CLI - Project Automation and Cloning Tool
 * LICENSE: MIT
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "colors.h"
#include "types.h"
#include "../config.h"
#include "utils.h"

#define MAX_PATH_LEN 1024 

static const size_t LANGUAGES_COUNT = sizeof(LANGUAGES) / sizeof(Language);
static const size_t PROJECTTYPES_COUNT = sizeof(PROJECTTYPES) / sizeof(ProjectType);

const Language *find_language(const char *flag);
const ProjectType *find_project_type(const char *flag);

/* Print-Helpers */
void print_short_usage(void);
void print_options(void);
void print_usage(void);
void print_new_usage();
void print_clone_usage();
void print_usage() ;
void print_project(Project *project);

/* Helper */
void get_root_dir_path(Project *project, char *root_dir);

/* Parsers */
// Project parse_project(int argc, char *argv[], Command cmd) ;
// void parse_new(int argc, char *argv[]);
// void parse_clone(int argc, char *argv[]) ;

/* Handlers */
void handle_new_project(Project *project);
void handle_clone_project(Project *project);

int parse_args(int argc, char *argv[], Command cmd, Project *project); 
Command process_command(const char *cmd_str);
int validate_project(const Project *project, Command cmd); 
int execute_command(Command cmd, Project *project); 
int process_arguments(int argc, char *argv[], Command cmd, Project *project); 

const Language *find_language(const char *flag) {
    for (size_t i = 0; i < LANGUAGES_COUNT; i++) {
        if (strcmp(flag, LANGUAGES[i].alias) == 0) {
            return &LANGUAGES[i];
        }
    }
    return NULL;
}

const ProjectType *find_project_type(const char *flag) {
    for (size_t i = 0; i < PROJECTTYPES_COUNT; i++) {
        if (strcmp(flag, PROJECTTYPES[i].alias) == 0) {
            return &PROJECTTYPES[i];
        }
    }
    return NULL;
}

void print_options(){
    printf("Options:\n");
    printf("  -t <type_alias> ");

    printf("[");
    for (size_t i = 0; i < PROJECTTYPES_COUNT; i++){
        if (i + 1 == PROJECTTYPES_COUNT){
            printf(GREEN "'%s'" RESET, PROJECTTYPES[i].alias);
            continue;
        }
        printf(GREEN "'%s'" RESET ", " , PROJECTTYPES[i].alias);
    }
    printf("]\n");

    printf("  -l <language> ");
    printf("[");
    for (size_t i = 0; i < LANGUAGES_COUNT; i++){
        if (i + 1 ==  LANGUAGES_COUNT){
            printf(GREEN "'%s'" RESET, LANGUAGES[i].alias);
            continue;
        }
        printf(GREEN "'%s'" RESET ", " , LANGUAGES[i].alias);
    }
    printf("]\n");
}

void print_new_usage(){
    printf("Usage: pact new -t <type> -l <lang> <project_name>\n");
    print_options();
    printf("  <project_name>  Name of the new project dir\n");
    printf("\nExample:\n");
    printf("  pact new  -l %s -t %s my_project_name\n", LANGUAGES[0].alias, PROJECTTYPES[0].alias );
}

void print_clone_usage(){
    printf("Usage: pact clone -t <type> -l <language> <git_url> <new_name>\n");
    print_options();
    printf("  <git_url>       URL of the repository to clone\n");
    printf("  <new_name>       optional rename of the repo\n");
    printf("\nExample:\n");
    printf("  pact clone  -l %s -t %s https://github.com/example/repo.git\n"
            , LANGUAGES[0].alias
            , PROJECTTYPES[0].alias );
}

void print_usage() {
    printf(CYAN "=============\n" RESET);
    printf(MAGENTA "  PACT CLI\n" RESET);
    printf(CYAN "=============\n\n" RESET);
    printf("Usage:\n");
    printf("  pact <command> [options]\n\n");
    printf("Commands:\n");
    printf("  new    Create a new project\n");
    printf("  clone  Clone a repository\n\n");
    print_options();
}

void print_project(Project *project)
{
    printf("Creating new project:\n");
    printf("  Type: %s (%s)\n", project->type->fulleName,
                                project->type->alias);

    printf("  Language: %s (%s)\n", project->lang->full_name,
                                    project->lang->alias);
    printf("  ProjectName: %s\n", project->name);
    printf("  Project URL: %s\n", project->url);

}

/* Handlers */
void handle_new_project(Project *project) {
    // 1. build root dir path 
    // 2. Make all dirs
    // 3. do initialisation
    // 4. init git, returns if its already git repo.
    // 5. Start/switch to tmux
    char root_dir[1024];
    build_root_dir(
            BASEDIR,
            project->type->fulleName,
            project->lang->full_name,
            project->name,
            root_dir,
            sizeof(root_dir)
            );

    printf("Root directory: %s\n", root_dir);
    print_project(project);

    int path_exist = create_root_dir(root_dir);
    if (!path_exist){
        run_project_init(project->lang, project->name, root_dir, PACT_TEMPLATES);
        init_git(root_dir);
    }
    start_tmux(project->name, root_dir);
}

void handle_clone_project(Project *project){
    // Set name to urls repo name
    // otherwise use the passed in name
    char repo_name[256];
    if(!project->name && project->url){
        get_repo_name(project->url, repo_name, sizeof(repo_name));
        printf("extracted name: %s\n", repo_name);
        project->name = strdup(repo_name);
    }

    char root_dir[1024];
    build_root_dir(
            BASEDIR,
            project->type->fulleName,
            project->lang->full_name,
            project->name,
            root_dir,
            sizeof(root_dir)
            );

    printf("Root directory: %s\n", root_dir);

    git_clone(project->url, project->name, root_dir);
    start_tmux(project->name, root_dir);

}

int parse_args(int argc, char *argv[], Command cmd, Project *project) {
    int opt;

    while ((opt = getopt(argc, argv, "l:t:")) != -1) {
        switch (opt) {
            case 'l':
                project->lang = find_language(optarg);
                if (!project->lang) {
                    fprintf(stderr, "Error: Invalid language '-l %s'.\n", optarg);
                    return 0;
                }
                break;
            case 't':
                project->type = find_project_type(optarg);
                if (!project->type) {
                    fprintf(stderr, "Error: Invalid project type '-t %s'.\n", optarg);
                    return 0;
                }
                break;
            default:
                fprintf(stderr, "Error: Unknown option '-%c'.\n", opt);
                return 0;
        }
    }


/*   Handle positional arguments */
    if (cmd == CMD_NEW && optind < argc) {
        project->name = argv[optind];
    } else if (cmd == CMD_CLONE) {
        if (optind < argc) {
            project->url = argv[optind++];
        }
        if (optind < argc) {
            project->name = argv[optind];
        }
    }

    return 1;  /* Successfully parsed */
}


Command process_command(const char *cmd_str){
    if (strcmp(cmd_str, "new") == 0){
        return CMD_NEW;
    } else if (strcmp(cmd_str, "clone") == 0) {
        return CMD_CLONE;
    } 
    return CMD_UNKNOWN;
}

int validate_project(const Project *project, Command cmd) {
    if (!project->lang) {
        fprintf(stderr, "Error: Language is required.\n");
        return 0;
    }
    if (!project->type) {
        fprintf(stderr, "Error: Project type is required.\n");
        return 0;
    }
    if (cmd == CMD_NEW && !project->name) {
        fprintf(stderr, "Error: Project name is required for 'new'.\n");
        return 0;
    }
    if (cmd == CMD_CLONE && !project->url) {
        fprintf(stderr, "Error: URL is required for 'clone'.\n");
        return 0;
    }
    return 1; // All inputs valid
}

int execute_command(Command cmd, Project *project) {
    switch (cmd) {
    case CMD_NEW:
         printf("Processing project clone...\n");
         handle_new_project(project);
         break;

    case CMD_CLONE:
         printf("Processing project clone...\n");
         handle_clone_project(project);
         break;

    default:
        /*  should not happen, as it is already validated */
        fprintf(stderr, "Error: Unknown command.\n");
        return 0;
    }

    return 1; // Successful execution
}

int process_arguments(int argc, char *argv[], Command cmd, Project *project) {
    if (!parse_args(argc, argv, cmd, project)) {
        if (cmd == CMD_NEW) {
            print_new_usage();
        } else {
            print_clone_usage();
        }
        // print_options();    /* Hint with valid options */
        return 0;           /* Error already printed in parse_args */
    }


    if (!validate_project(project, cmd)) {
        return 0;           /* Error already printed in validate_project */
    }

    return 1;               /* Success */
}

int main(int argc, char *argv[])
{
    if (argc < 3) {
        print_usage();
        return 1;
    }

    Command cmd = process_command(argv[1]);
    if (cmd == CMD_UNKNOWN) {
        fprintf(stderr, "Error: Unknown command '%s'.\n", argv[1]);
        print_usage();
        return 1;
    }

    Project project = {0}; // Init to zero or NULL
    if (!process_arguments(argc - 1, &argv[1], cmd, &project)) {
        return 1; // Argument parsing or validation failed
    }

    if (!execute_command(cmd, &project)) {
        // print_project(&project);
        return 1; // Execution failed
    }
    return 0;
}

