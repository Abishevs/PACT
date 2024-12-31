#include "utils.h"
#include <stdio.h>
#include <stdlib.h>
#include <sys/stat.h>
#include <string.h>

void normalize_path(char *path) {
    char *src = path, *dest = path;

    while (*src) {
        *dest = *src++;

        // If a slash is followed by another slash, skip the second one
        if (*dest == '/' && *src == '/') {
            continue;
        }
        dest++;
    }
    *dest = '\0';
}

void run_project_init(const Language *lang, const char *project_name, const char *root_dir, const char *templates_dir) {
    // Run init_command if defined
    // if (lang->init_command) {
    //     printf("Running init command for language: %s\n", lang->full_name);
    //     char command[1024];
    //     snprintf(command, sizeof(command), "cd %s && %s", root_dir, lang->init_command);
    //     if (system(command) != 0) {
    //         fprintf(stderr, "Error: Failed to execute init command for '%s'.\n", lang->full_name);
    //         exit(1);
    //     }
    // }

    if (lang->init_command) {
        printf("Running init command for language: %s\n", lang->full_name);

        char command[1024];
        snprintf(command, sizeof(command), "cd %s && %s", root_dir, lang->init_command);

        // Get the user's shell
        const char *user_shell = getenv("SHELL");
        if (!user_shell) {
            user_shell = "/bin/sh"; // Default to bash if SHELL is not set
        }

        // Run the command in the user's shell
        char shell_command[2048];
        snprintf(shell_command, sizeof(shell_command), "%s -lc '%s'", user_shell, command);
        if (system(shell_command) != 0) {
            fprintf(stderr, "Error: Failed to execute init command for '%s'.\n", lang->full_name);
            exit(1);
        }
    }

    char shared_dir[1024];

    snprintf(shared_dir, sizeof(shared_dir), "%s/templates/shared", templates_dir);
    normalize_path(shared_dir);
    cp_templates(root_dir, shared_dir); // Copy shared files like README, LICENSE


    // If no init_command, copy language-specific templates
    if (lang->template_dir) {
        char lang_templates[1024];

        snprintf(lang_templates, sizeof(lang_templates), "/%s/%s", templates_dir, lang->template_dir);
        normalize_path(lang_templates);
        cp_templates(root_dir, lang_templates); // Cpy project specific
                                               // templates
        printf("Copying language-specific templates for: %s\n", lang->full_name);
        printf("From : '%s' \nto root dir '%s'\n", lang_templates, root_dir);
    } else {
        printf("No language-specific templates to copy for: %s\n", lang->full_name);
    }
}


void build_root_dir(const char *base_path, const char *type, const char *lang, const char *project_name, char *result, size_t result_size)
{
    // Combine all parts into the result string
    snprintf(result, result_size, "%s/%s/%s/%s", base_path, type, lang, project_name);
    normalize_path(result);
}

int dir_exists(const char *path) {
    struct stat info;
    return stat(path, &info) == 0 && S_ISDIR(info.st_mode);
}

void get_repo_name(const char *url, char *repo_name, size_t max_length) {
    // Find the last slash in the URL
    const char *last_slash = strrchr(url, '/');
    if (!last_slash) {
        fprintf(stderr, "Error: Invalid URL format (no slash found).\n");
        exit(1);
    }

    // Move past the last slash
    const char *name_start = last_slash + 1;

    // Check for trailing ".git" and remove it
    const char *git_suffix = ".git";
    size_t name_length = strlen(name_start);
    if (name_length >= 4 && strcmp(name_start + name_length - 4, git_suffix) == 0) {
        name_length -= 4; // Exclude ".git"
    }

    // Check if the name is empty or too long
    if (name_length == 0 || name_length >= max_length) {
        fprintf(stderr, "Error: Invalid or too long repository name.\n");
        exit(1);
    }

    // Copy the repository name into the output buffer
    strncpy(repo_name, name_start, name_length);
    repo_name[name_length] = '\0'; // Null-terminate the string
}

void git_clone(char *repo_url, char *new_path) {
    char command[1024];
    snprintf(command, sizeof(command), "git clone %s %s", repo_url, new_path);
    if (system(command) != 0) {
        exit(1); // Exit and show errors from git command 
    }

}

int create_root_dir(const char *path) {
    // Returns 1 if path exist
    // Returns 0 if all good
    // sysexit if fails to create.
    //
    if (dir_exists(path)) {
        printf("Directory already exists: %s\n", path);
        return 1; // Skip creation and exit early
    }

    char command[1024];

    // Format the system command
    snprintf(command, sizeof(command), "mkdir -p %s", path);

    // Execute the command
    if (system(command) != 0) {
        fprintf(stderr, "Error: Failed to create directory '%s' using mkdir -p.\n", path);
        exit(1);
    }

    printf("Created directory: %s\n", path);
    return 0;
}

void cp_templates(const char *root_dir, const char *source_dir) {
    char command[1024];

    snprintf(command, sizeof(command), "cp -r %s/* %s", source_dir, root_dir);
    if (system(command) != 0) {
        fprintf(stderr, "Error: Failed to copy templates from '%s' to '%s'.\n", source_dir, root_dir);
        return;
    } else {
        printf("Copied templates from '%s' to '%s'\n", source_dir, root_dir);
    }
}

void init_git(const char *path) {
    char command[1024];

    // Check if the directory is already a Git repository
    snprintf(command, sizeof(command), "git -C %s rev-parse --is-inside-work-tree >/dev/null 2>&1", path);
    if (system(command) == 0) {
        printf("Directory '%s' is already a Git repository. Skipping git init.\n", path);
        return; // Exit early since it's already a repo
    }

    // Initialize a new Git repository
    snprintf(command, sizeof(command), "git -C %s init", path);
    if (system(command) != 0) {
        fprintf(stderr, "Error: Failed to initialize Git repository in '%s'.\n", path);
        exit(1); // Exit on failure
    }

    printf("Initialized Git repository in: %s\n", path);
}

void start_tmux(const char *project_name, const char *root_dir) {
    char command[1024];

    // Check if tmux is already running
    if (system("tmux info >/dev/null 2>&1") != 0) {
        // tmux is not running; start it with the new session
        snprintf(command, sizeof(command), "tmux new-session -s %s -c %s", project_name, root_dir);
        if (system(command) != 0) {
            fprintf(stderr, "Error: Failed to create and start tmux session '%s'.\n", project_name);
            exit(1);
        }
    } else {
        // tmux is running; create a new detached session
        snprintf(command, sizeof(command), "tmux new-session -d -s %s -c %s 2>/dev/null", project_name, root_dir);
        if (system(command) != 0) {
            fprintf(stderr, "Error: Failed to create tmux session '%s'.\n", project_name);
            exit(1);
        }

        // Attach to the session
        snprintf(command, sizeof(command), "tmux switch -t %s", project_name);
        if (system(command) != 0) {
            fprintf(stderr, "Error: Failed to attach to tmux session '%s'.\n", project_name);
            exit(1);
        }
    }

    printf("Switched to tmux session: %s\n", project_name);
}
