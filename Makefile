all: help

.PHONY: help
help:
	@echo Usage:
	@echo   - make backend_watch
	@echo   - make frontend_watch

.PHONY: backend_watch
backend_watch:
	cd backend; cargo watch -x run

.PHONY: frontend_watch
frontend_watch:
	cd frontend; trunk watch