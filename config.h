//ABS path to basedir eg dev or projects
static const char *BASEDIR = "/home/frap/dev";
static const char *PACT_TEMPLATES = "/home/frap/dev/personal/c/PACT_templates";
static const char *SHELL = "/usr/bin/zsh";

static const Language LANGUAGES[] = {
    // "CLI Alias", "Dir name", "templates folder to cpy", "init command"
    { "c", "c", "templates/c", NULL },
    { "cpp", "cpp", "templates/cpp", NULL },
    { "raylib", "cpp", "templates/raylibcpp", NULL },
    { "py", "python", "templates/python", NULL },
    { "rs", "rust", NULL, "cargo init" },
    { "tex", "latex", "templates/latex", NULL },
    { "esp", "esp", NULL, "cp -r /home/frap/esp/esp-idf/examples/get-started/hello_world ." },
    { "go", "go", NULL, NULL},
    { "misc", "mix", NULL, NULL}
};

static const ProjectType PROJECTTYPES[] = {
    // "CLI Alias" "dir name"
	{ "p", "personal" },
	{ "w", "work" },
	{ "s", "school" },
	{ "t", "test" }
};

