SRC_DIR     := src
BIN_DIR     := $(HOME)/.local/bin
SCRIPT_NAME := pact
SRC_FILE    := $(SRC_DIR)/$(SCRIPT_NAME).sh
DEST_FILE   := $(BIN_DIR)/$(SCRIPT_NAME)
DEPS        := zsh awk git

.DEFAULT_GOAL := help

.PHONY: install uninstall deps help

deps: ## Check that required dependencies are installed
	@for dep in $(DEPS); do \
		if ! command -v $$dep >/dev/null 2>&1; then \
			echo "Missing dependency: $$dep"; exit 1; \
		fi; \
	done
	@echo "All dependencies present."

install: deps ## Install script to ~/.local/bin
	@mkdir -p $(BIN_DIR)
	@cp -f $(SRC_FILE) $(DEST_FILE)
	@chmod +x $(DEST_FILE)
	@echo "Installed $(SCRIPT_NAME) -> $(DEST_FILE)"

uninstall: ## Remove script from ~/.local/bin
	@rm -f $(DEST_FILE)
	@echo "Removed $(DEST_FILE)"

help: ## Show this help message
	@echo "Usage: make [target]"
	@awk -F':.*##' '/^[a-zA-Z0-9_.-]+:.*##/ {printf "  %-12s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

