all: help

.PHONY: help
help:
	@echo Usage:
	@echo   - make backend_watch
	@echo   - make frontend_watch
	@echo   - start_db
	@echo   - reset_db

backend_watch:
	cd backend; cargo watch -x run

reset_db:
	PGPASSWORD=postgres psql -h localhost -U postgres << backend/reset_db.sql < backend/reset_db.sql

frontend_watch:
	cd frontend; trunk watch

start_db:
	docker-compose up