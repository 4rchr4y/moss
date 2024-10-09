DESKTOP_DIR = view/desktop
STORYBOOK_DIR = view/storybook
DOCS_DIR = view/docs
WEB_DIR = view/web
THEME_GENERATOR_DIR = tools/theme-generator
ICONS_DIR = view/shared/icons

PNPM = pnpm
SURREAL = surreal

.DEFAULT_GOAL := run-desktop

.PHONY: \
	run-desktop \
	run-desktop-web \
	run-storybook \
	run-database \
	stop-database \
	run-docs \
	run-web \
	gen-themes \
	gen-icons \
	check-db \
	loc \
	cleanup \


run-desktop: check-db
	@cd $(DESKTOP_DIR) && $(PNPM) tauri dev

run-desktop-web:
	@cd $(DESKTOP_DIR) && $(PNPM) vite dev

run-storybook:
	@cd $(STORYBOOK_DIR) && $(PNPM) dev

run-database:
	@cd $(DESKTOP_DIR) && $(SURREAL) start file:rocksdb &

stop-database:
	@pkill -x surreal

run-docs:
	@cd $(DOCS_DIR) && $(PNPM) dev

run-web:
	@cd $(WEB_DIR) && $(PNPM) dev

gen-themes:
	@cd $(THEME_GENERATOR_DIR) && $(PNPM) start

gen-icons:
	@cd $(ICONS_DIR) && $(PNPM) run build

# Check if the database is running, if not, start it in the background
check-db:
	@if ! pgrep -x "surreal" > /dev/null; then \
		$(MAKE) run-database; \
	fi	

# Comma separated list of file extensions to count
SRC_EXT := rs,ts
# Comma separated list of directories to exclude
EXCLUDE_DIRS := target,node_modules

# Count lines of code
loc:
	@cloc --exclude-dir=$(EXCLUDE_DIRS) --include-ext=$(SRC_EXT) .

# Clean up merged branches except master, main, and dev
cleanup-git:
	@git branch --merged | grep -Ev "(^\*|master|main|dev)" | xargs git branch -d