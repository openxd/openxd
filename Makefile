NPM := npm
CARGO := cargo
WEB_FRONTEND_DIR := web-frontend

.PHONY: list
list:
	@echo ""
	@echo "Useful targets:"
	@echo ""
	@echo "  watch_web_frontend > Watching the file changes of web frontend and recompile"
	@echo "  serve_web_frontend > Serving the web frontend in 127.0.0.1:8000"
	@echo "  build_web_frontend > Building the web frontend source code in development profile"

# Web Frontend

.PHONY: watch_web_frontend
watch_web_frontend: $(WEB_FRONTEND_DIR)/node_modules
	$(NPM) --prefix $(WEB_FRONTEND_DIR) run start

.PHONY: serve_web_frontend
serve_web_frontend: $(WEB_FRONTEND_DIR)/dist $(WEB_FRONTEND_DIR)/node_modules
	$(NPM) --prefix $(WEB_FRONTEND_DIR) run server

.PHONY: build_web_frontend
build_web_frontend: $(WEB_FRONTEND_DIR)/dist

$(WEB_FRONTEND_DIR)/dist: $(WEB_FRONTEND_DIR)/node_modules
	$(NPM) --prefix $(WEB_FRONTEND_DIR) run build

$(WEB_FRONTEND_DIR)/node_modules:
	$(NPM) --prefix $(WEB_FRONTEND_DIR) install

$(WEB_FRONTEND_DIR)/package-lock.json: $(WEB_FRONTEND_DIR)/node_modules
