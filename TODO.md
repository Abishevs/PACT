## User space avreness
so that $HOME and others would be possible

## Some sort of option --force / --overide 
As when there are missing templates / `command_init` yet it still creates the dirs.
And on second run it will indeed open the tmux session.

## A option to only copy templates mby?
Eg if user made a syntax error in config.h
But then again, dont make mistakes and it works good.

## a bit dynamic templates thingie?
* if it startes with `templates/` then use the `PACT_templates` dir
Otherwise the passed path is to be expected to be an absolute path.
