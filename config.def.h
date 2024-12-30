//ABS path to basedir eg dev or projects
static const char *BASEDIR = "/home/frap/dev";
static const char *PACT_TEMPLATES = "/home/frap/dev/personal/c/PACTInC/pact_templates/";

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

