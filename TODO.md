# TODO
# v1.0.0
This is to create the MVP version.
which will include features:
## MVP features
- [x] Two different modes new and clone
- [x] yaml config file.
- [x] Dynamcly parse language, and project type. Include it in the help messages.
- [ ] Abstract away pre, main and post phases (commands will be defined in the yaml config file).
- [ ] Execute phases
- [ ] DRY mode to see what will be the output, no running of actual commands
- [ ] Figure out templating coping, mby it could be a simple bash command
- [ ] All commands use user shell such that aliases and PATH works as expected

## Nice to haves
- [x] Dynamic checking and autosuggestions of the cli options.
- [ ] Dynamic template scaffolding based on keys in format of {{name_to_be_changed}}
