//ABS path to basedir eg dev or projects
static const char *BASEDIR = "path/to/projects/dir";
static const char *PACT_TEMPLATES = "path/to/root/dir/of/templates";

static const Language LANGUAGES[] = {
    // "CLI Alias", "Dir name", "templates folder to cpy", "init command"
    { "c", "c", "templates/c", NULL },
    { "py", "python", "templates/python", NULL },
    { "rs", "rust", "templates/rust", "cargo init" }
};

static const ProjectType PROJECTTYPES[] = {
    // "CLI Alias" "dir name"
	{ "p", "personal" },
	{ "w", "work" },
	{ "s", "school" },
	{ "t", "test" }
};

