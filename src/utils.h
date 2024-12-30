#ifndef UTILS_H
#define UTILS_H 
#include <stdio.h>
#include "types.h"

void run_project_init(const Language *lang, const char *project_name, const char *root_dir, const char *templates_dir);
void build_root_dir(const char *base_path, const char *type, const char *lang, const char *project_name, char *result, size_t result_size);
int create_root_dir(const char *root_dir);
void cp_templates(const char *root_dir, const char *path_templates); 
void init_git(const char *path);
void get_repo_name(const char *url, char *repo_name, size_t max_length);
void git_clone(char *repo_url, char *project_name, char *root_dir);
// Starts tmux session with name of project in root dir
void start_tmux(const char *project_name,const char *root_dir);

#endif
