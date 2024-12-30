#ifndef TYPES_H
#define TYPES_H

typedef struct {
    const char *alias;        // Short CLI alias (e.g., "py", "rs")
    const char *full_name;    // Full name for the language (e.g., "python", "rust")
    const char *template_dir; // Path to templates (optional)
    const char *init_command; // Custom init command (optional)
} Language;

typedef struct {
	const char *alias;
	const char *fulleName;
} ProjectType;

typedef struct {
    const Language *lang;       // Selected lang dir
    const ProjectType *type;    // Selected type of dir
    char *name;                 // selected name of dir
    char *url;                   // Git URL (Null for command new) 
} Project;

typedef enum {
    CMD_NEW,   // Represents the new command
    CMD_CLONE,  // Represents the clone command
    CMD_UNKNOWN
} Command;

#endif
